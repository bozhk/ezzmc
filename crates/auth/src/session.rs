//! Шифрованный логин: хэш serverId и `session/join`.
// VANILLA: Crypt#digestData, MinecraftSessionService#joinServer
use crate::{net, AuthError, AuthResult};
use sha1::{Digest, Sha1};
use std::time::Duration;

/// Minecraft-хэш: SHA-1 как знаковое big-endian число, hex без ведущих нулей.
pub fn server_hash(server_id: &str, shared_secret: &[u8], public_key_der: &[u8]) -> String {
    let mut h = Sha1::new();
    h.update(server_id.as_bytes());
    h.update(shared_secret);
    h.update(public_key_der);
    let mut d: [u8; 20] = h.finalize().into();
    let neg = d[0] & 0x80 != 0;
    if neg {
        // дополнение до двух
        let mut carry = true;
        for b in d.iter_mut().rev() {
            *b = !*b;
            if carry {
                let (v, c) = b.overflowing_add(1);
                *b = v;
                carry = c;
            }
        }
    }
    let hex: String = d.iter().map(|b| format!("{b:02x}")).collect();
    let t = hex.trim_start_matches('0');
    let t = if t.is_empty() { "0" } else { t };
    if neg {
        format!("-{t}")
    } else {
        t.to_string()
    }
}

/// POST sessionserver.mojang.com/session/minecraft/join.
pub fn join_server(access_token: &str, uuid: &str, hash: &str) -> AuthResult<()> {
    let body = serde_json::json!({
        "accessToken": access_token,
        "selectedProfile": uuid.replace('-', ""),
        "serverId": hash,
    });
    ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(15))
        .build()
        .post("https://sessionserver.mojang.com/session/minecraft/join")
        .send_json(body)
        .map_err(|e| match net(e) {
            AuthError::Net(m) => AuthError::Msg(format!("session/join отклонён ({m})")),
            o => o,
        })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Известные векторы с wiki.vg (Protocol Encryption): sha1 от строки.
    fn h(s: &str) -> String {
        server_hash(s, b"", b"")
    }

    #[test]
    fn wiki_vectors() {
        assert_eq!(h("Notch"), "4ed1f46bbe04bc756bcb17c0c7ce3e4632f06a48");
        assert_eq!(h("jeb_"), "-7c9d5b0044c130109a5d7b5fb5c317c02b4e28c1");
        assert_eq!(h("simon"), "88e16a1019277b15d58faf0541e11910eb756f6");
    }
}
