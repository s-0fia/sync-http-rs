use std::error::Error;

pub mod errors;
pub mod mime;
pub mod query;
pub mod request;
pub mod server;

pub type ServerResult<T> = Result<T, Box<dyn Error>>;
