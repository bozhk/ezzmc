//! Microsoft device-code → Xbox Live → XSTS → Minecraft services.
//! Azure client_id задаёт владелец (настройка лаунчера / `EZZMC_MSA_CLIENT_ID`): Mojang
//! разрешает Minecraft API только одобренным приложениям (aka.ms/AppRegInfo).
use crate::{net, AuthError, AuthResult};
use serde_json::{json, Value};
use std::time::Duration;

const SCOPE: &str = "XboxLive.signin offline_access";
const TENANT: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0";

fn agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(20))
        .build()
}

#[derive(Clone, Debug)]
pub struct DeviceCode {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}

pub fn device_code(client_id: &str) -> AuthResult<DeviceCode> {
    let v: Value = agent()
        .post(&format!("{TENANT}/devicecode"))
        .send_form(&[("client_id", client_id), ("scope", SCOPE)])
        .map_err(net)?
        .into_json()
        .map_err(|e| AuthError::Net(e.to_string()))?;
    let s = |k: &str| {
        v[k].as_str()
            .map(str::to_owned)
            .ok_or_else(|| AuthError::Msg(format!("devicecode: нет поля {k}")))
    };
    Ok(DeviceCode {
        device_code: s("device_code")?,
        user_code: s("user_code")?,
        verification_uri: s("verification_uri")?,
        expires_in: v["expires_in"].as_u64().unwrap_or(900),
        interval: v["interval"].as_u64().unwrap_or(5).max(1),
    })
}

/// Результат одного опроса токена.
pub enum Poll {
    Pending,
    SlowDown,
    Done(MsTokens),
}

#[derive(Clone)]
pub struct MsTokens {
    pub access_token: String,
    pub refresh_token: String,
}

fn token_request(form: &[(&str, &str)]) -> AuthResult<Poll> {
    match agent().post(&format!("{TENANT}/token")).send_form(form) {
        Ok(r) => {
            let v: Value = r.into_json().map_err(|e| AuthError::Net(e.to_string()))?;
            Ok(Poll::Done(MsTokens {
                access_token: v["access_token"].as_str().unwrap_or_default().into(),
                refresh_token: v["refresh_token"].as_str().unwrap_or_default().into(),
            }))
        }
        Err(ureq::Error::Status(_, r)) => {
            let v: Value = r.into_json().unwrap_or(Value::Null);
            match v["error"].as_str().unwrap_or("") {
                "authorization_pending" => Ok(Poll::Pending),
                "slow_down" => Ok(Poll::SlowDown),
                "authorization_declined" => Err(AuthError::Msg("вход отклонён".into())),
                "expired_token" | "code_expired" => Err(AuthError::Msg("код истёк".into())),
                "invalid_grant" => Err(AuthError::Msg(
                    "вход Microsoft устарел — войдите заново".into(),
                )),
                other => Err(AuthError::Msg(format!(
                    "Microsoft: {} {}",
                    other,
                    v["error_description"].as_str().unwrap_or("")
                ))),
            }
        }
        Err(e) => Err(net(e)),
    }
}

pub fn poll(client_id: &str, code: &DeviceCode) -> AuthResult<Poll> {
    token_request(&[
        ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
        ("client_id", client_id),
        ("device_code", &code.device_code),
    ])
}

pub fn refresh(client_id: &str, refresh_token: &str) -> AuthResult<MsTokens> {
    match token_request(&[
        ("grant_type", "refresh_token"),
        ("client_id", client_id),
        ("refresh_token", refresh_token),
        ("scope", SCOPE),
    ])? {
        Poll::Done(t) => Ok(t),
        _ => Err(AuthError::Msg("не удалось обновить вход".into())),
    }
}

#[derive(Clone)]
pub struct McLogin {
    pub name: String,
    /// UUID без дефисов.
    pub uuid: String,
    pub access_token: String,
    /// Unix-секунды, когда токен истечёт.
    pub expires_at: u64,
}

fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn xbox_err(code: u64) -> &'static str {
    match code {
        2148916233 => "у аккаунта нет Xbox-профиля (создайте на xbox.com)",
        2148916235 => "Xbox Live недоступен в вашей стране",
        2148916236 | 2148916237 => "нужно пройти проверку возраста на xbox.com",
        2148916238 => "детский аккаунт: добавьте его в семью Microsoft",
        _ => "XSTS отказал",
    }
}

/// Xbox Live → XSTS → Minecraft → профиль.
pub fn minecraft_login(ms_access_token: &str) -> AuthResult<McLogin> {
    let a = agent();
    let xbl: Value = a
        .post("https://user.auth.xboxlive.com/user/authenticate")
        .set("Accept", "application/json")
        .send_json(json!({
            "Properties": {"AuthMethod": "RPS", "SiteName": "user.auth.xboxlive.com",
                           "RpsTicket": format!("d={ms_access_token}")},
            "RelyingParty": "http://auth.xboxlive.com",
            "TokenType": "JWT"
        }))
        .map_err(net)?
        .into_json()
        .map_err(|e| AuthError::Net(e.to_string()))?;
    let xbl_token = xbl["Token"].as_str().unwrap_or_default().to_string();
    let uhs = xbl["DisplayClaims"]["xui"][0]["uhs"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    if xbl_token.is_empty() || uhs.is_empty() {
        return Err(AuthError::Msg("Xbox Live не выдал токен".into()));
    }

    let xsts = match a
        .post("https://xsts.auth.xboxlive.com/xsts/authorize")
        .set("Accept", "application/json")
        .send_json(json!({
            "Properties": {"SandboxId": "RETAIL", "UserTokens": [xbl_token]},
            "RelyingParty": "rp://api.minecraftservices.com/",
            "TokenType": "JWT"
        })) {
        Ok(r) => r
            .into_json::<Value>()
            .map_err(|e| AuthError::Net(e.to_string()))?,
        Err(ureq::Error::Status(_, r)) => {
            let v: Value = r.into_json().unwrap_or(Value::Null);
            return Err(AuthError::Msg(
                xbox_err(v["XErr"].as_u64().unwrap_or(0)).into(),
            ));
        }
        Err(e) => return Err(net(e)),
    };
    let xsts_token = xsts["Token"].as_str().unwrap_or_default();

    let mc: Value =
        match a
            .post("https://api.minecraftservices.com/authentication/login_with_xbox")
            .send_json(json!({"identityToken": format!("XBL3.0 x={uhs};{xsts_token}")}))
        {
            Ok(r) => r.into_json().map_err(|e| AuthError::Net(e.to_string()))?,
            Err(ureq::Error::Status(403, _)) => return Err(AuthError::Msg(
                "Mojang отклонил приложение (403): Azure client_id не одобрен для Minecraft API \
                 (форма aka.ms/AppRegInfo)"
                    .into(),
            )),
            Err(e) => return Err(net(e)),
        };
    let token = mc["access_token"].as_str().unwrap_or_default().to_string();
    if token.is_empty() {
        return Err(AuthError::Msg("Minecraft services не выдали токен".into()));
    }
    let expires_at = now() + mc["expires_in"].as_u64().unwrap_or(86400);

    let prof: Value = match a
        .get("https://api.minecraftservices.com/minecraft/profile")
        .set("Authorization", &format!("Bearer {token}"))
        .call()
    {
        Ok(r) => r.into_json().map_err(|e| AuthError::Net(e.to_string()))?,
        Err(ureq::Error::Status(404, _)) => {
            return Err(AuthError::Msg(
                "у аккаунта нет профиля Minecraft: Java Edition (создайте на minecraft.net)".into(),
            ))
        }
        Err(e) => return Err(net(e)),
    };
    Ok(McLogin {
        name: prof["name"].as_str().unwrap_or_default().into(),
        uuid: prof["id"].as_str().unwrap_or_default().into(),
        access_token: token,
        expires_at,
    })
}
