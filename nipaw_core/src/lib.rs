mod client;
pub mod error;

pub use client::Client;
pub use error::Error;
pub mod option;
pub mod types;

pub type Result<T> = std::result::Result<T, Error>;
