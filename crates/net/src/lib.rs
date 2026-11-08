//! TCP, фрейминг, компрессия, состояния соединения.
pub mod conn;
pub mod error;
pub mod frame;
pub mod play;
pub mod replay;
pub mod session;
pub mod srv;

pub use conn::{Connection, Intent};
pub use error::{NetError, NetResult};
