//! TCP, фрейминг, компрессия, состояния соединения.
pub mod chunkbatch;
pub mod conn;
pub mod crypt;
pub mod error;
pub mod frame;
pub mod play;
pub mod replay;
pub mod session;
pub mod srv;
pub mod status;

pub use conn::{Connection, Intent};
pub use error::{NetError, NetResult};
