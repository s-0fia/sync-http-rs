#[derive(Debug)]
pub enum RequestError {
    TooManyValues,
    BadMethod,
    BadProtocol,
}
