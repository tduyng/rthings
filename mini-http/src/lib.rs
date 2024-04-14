mod date;
mod request;
mod response;

pub use date::*;
pub use request::*;
pub use response::*;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
