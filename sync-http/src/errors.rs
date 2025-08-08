use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum RequestError {
    TooManyValues,
    BadMethod,
    BadProtocol,
}

#[derive(Debug)]
pub struct FailedToCompileRoute;

impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Display for FailedToCompileRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for RequestError {}
impl Error for FailedToCompileRoute {}
