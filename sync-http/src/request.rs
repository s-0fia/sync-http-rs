use crate::errors::*;
use crate::mime::*;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub uri: String,
    pub headers: Vec<Header>,
}

// TODO: Implement more methods
#[derive(Debug)]
pub enum Method {
    Get,
    Post,
}

impl Method {
    pub fn parse(method: &str) -> Result<Method, RequestError> {
        Ok(match method.to_uppercase().as_str() {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Err(RequestError::BadMethod)?,
        })
    }
}

#[derive(Debug)]
pub enum Header {
    Host(String),
    UserAgent(String),
    Accept(ContentType),
}

impl Header {
    pub fn parse_header(header: &str) -> Option<Self> {
        let parts = header.split(":");
        let head_type: String = parts.clone().take(1).collect();
        let head_value: String = parts.skip(1).collect();
        let head_value = head_value.trim().to_string();

        Some(match head_type.trim() {
            "Host" => Self::Host(head_value),
            "User-Agent" => Self::UserAgent(head_value),
            "Accept" => Self::Accept(ContentType::parse(head_value)?),
            _ => None?,
        })
    }
}

impl Request {
    pub fn parse(req: String) -> Result<Self, RequestError> {
        // Remove everything after the headers from the request
        let req: String = req.split("\r\n\r\n").take(1).collect();
        // Split the request into its parts
        let parts: Vec<&str> = req.split("\n").map(|line| line.trim()).collect();

        // GET / HTTP/1.1
        // METHOD URI PROTOCOL
        let first = parts[0].split(" ").collect::<Vec<_>>();

        if first.len() != 3 {
            return Err(RequestError::TooManyValues);
        }
        let method = Method::parse(first[0])?;
        let uri = first[1].to_string();

        // We only support HTTP/1.1
        if first[2].to_uppercase().as_str() != "HTTP/1.1" {
            return Err(RequestError::BadProtocol);
        }

        let headers = parts
            .iter()
            .skip(1)
            .filter_map(|&line| Header::parse_header(line))
            .collect::<Vec<_>>();

        Ok(Request {
            method,
            uri,
            headers,
        })
    }
}
