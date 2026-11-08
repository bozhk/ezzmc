//! Ошибки сетевого слоя. Паники от сетевых данных запрещены (TODO-025):
//! всё, что пришло с провода, превращается в `NetError`, а не в `unwrap`.
use ezzmc_protocol::ProtocolError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetError {
    #[error("ввод-вывод: {0}")]
    Io(#[from] io::Error),
    #[error("протокол: {0}")]
    Protocol(#[from] ProtocolError),
    #[error("соединение закрыто сервером")]
    Closed,
    #[error("таймаут: {0}")]
    Timeout(&'static str),
    #[error("сервер отключил: {0}")]
    Disconnected(String),
}

pub type NetResult<T> = Result<T, NetError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_error_converts() {
        let e: NetError = ProtocolError::BadUtf8.into();
        assert!(matches!(e, NetError::Protocol(ProtocolError::BadUtf8)));
        assert!(e.to_string().contains("UTF-8"));
    }
}
