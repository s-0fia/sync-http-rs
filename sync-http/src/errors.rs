use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum RequestError {
    TooManyValues,
    BadMethod,
    BadProtocol,
}

impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for RequestError {}
