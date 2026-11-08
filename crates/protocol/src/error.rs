//! Ошибки разбора протокола. Политика (TODO-025): данные из сети никогда не
//! вызывают панику — любое нарушение формата возвращается как `ProtocolError`.
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProtocolError {
    #[error("неожиданный конец данных (нужно ещё {needed} байт)")]
    Eof { needed: usize },
    #[error("VarInt/VarLong длиннее допустимого ({max} байт)")]
    VarTooLong { max: usize },
    #[error("длина {len} превышает лимит {limit} ({what})")]
    TooLarge {
        what: &'static str,
        len: usize,
        limit: usize,
    },
    #[error("отрицательная длина: {0}")]
    NegativeLength(i64),
    #[error("неверный UTF-8 в строке")]
    BadUtf8,
    #[error("неизвестный пакет: state={state:?} id={id:#x}")]
    UnknownPacket { state: crate::State, id: u32 },
    #[error("неверное значение {what}: {value}")]
    InvalidValue { what: &'static str, value: i64 },
    #[error("{0}")]
    Other(String),
}

pub type ProtocolResult<T> = Result<T, ProtocolError>;
