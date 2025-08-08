use super::RequestError;

// TODO: Implement more methods
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Method {
    Get,
    Post,
}

impl Method {
    pub fn parse(method: &str) -> Result<Method, RequestError> {
        // Methods are case sensitive
        Ok(match method.trim() {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Err(RequestError::BadMethod)?,
        })
    }
}
