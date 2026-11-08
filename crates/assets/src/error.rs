use thiserror::Error;

#[derive(Debug, Error)]
pub enum AssetError {
    #[error("ввод-вывод ({path}): {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("zip ({path}): {msg}")]
    Zip { path: String, msg: String },
    #[error("json ({path}): {msg}")]
    Json { path: String, msg: String },
    #[error("игра не найдена: {0}")]
    NotFound(String),
    #[error("распаковка {path}: {msg}")]
    Inflate { path: String, msg: String },
}

pub type AssetResult<T> = Result<T, AssetError>;

pub(crate) fn io(path: &std::path::Path, source: std::io::Error) -> AssetError {
    AssetError::Io {
        path: path.display().to_string(),
        source,
    }
}
