#[derive(Debug, PartialEq, Eq)]
pub enum RequestError {
    TooManyValues,
    BadMethod,
    BadProtocol,
}
