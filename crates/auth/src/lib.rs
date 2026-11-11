//! Онлайн-вход (DEC-016, снимает запрет «без аккаунтов» ради проверки античита на online-mode серверах):
//! Microsoft device-code → Xbox Live → XSTS → Minecraft services → профиль, и `session/join`
//! для шифрованного логина. Токены в логи не попадают.
pub mod microsoft;
pub mod session;

/// Сессия игры: то, что клиент получает от лаунчера (через окружение, не через argv).
#[derive(Clone, PartialEq, Eq)]
pub struct GameSession {
    pub name: String,
    /// UUID без дефисов.
    pub uuid: String,
    pub access_token: String,
}

impl std::fmt::Debug for GameSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameSession")
            .field("name", &self.name)
            .field("uuid", &self.uuid)
            .field("access_token", &"<скрыт>")
            .finish()
    }
}

pub const ENV_NAME: &str = "EZZMC_SESSION_NAME";
pub const ENV_UUID: &str = "EZZMC_SESSION_UUID";
pub const ENV_TOKEN: &str = "EZZMC_SESSION_TOKEN";

impl GameSession {
    /// Прочитать сессию из окружения (и сразу стереть токен из окружения процесса).
    pub fn from_env() -> Option<Self> {
        let token = std::env::var(ENV_TOKEN).ok().filter(|s| !s.is_empty())?;
        let name = std::env::var(ENV_NAME).ok().filter(|s| !s.is_empty())?;
        let uuid = std::env::var(ENV_UUID).ok().filter(|s| !s.is_empty())?;
        std::env::remove_var(ENV_TOKEN);
        Some(Self {
            name,
            uuid: uuid.replace('-', ""),
            access_token: token,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("сеть: {0}")]
    Net(String),
    #[error("{0}")]
    Msg(String),
}

pub type AuthResult<T> = Result<T, AuthError>;

pub(crate) fn net(e: ureq::Error) -> AuthError {
    match e {
        ureq::Error::Status(code, r) => {
            let body = r.into_string().unwrap_or_default();
            let short: String = body.chars().take(300).collect();
            AuthError::Net(format!("HTTP {code}: {short}"))
        }
        other => AuthError::Net(other.to_string()),
    }
}
