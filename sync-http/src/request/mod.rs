use crate::errors::*;
use crate::mime::*;
use crate::query::Query;
pub mod header;
pub mod method;
pub use header::*;
pub use method::*;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Request {
    pub method: Method,
    pub uri: String,
    pub query: Query,
    pub headers: Vec<Header>,
}

impl Request {
    pub fn parse(req: String) -> Result<Self, RequestError> {
        // Remove everything after the headers from the request
        let req: String = req.split("\r\n\r\n").take(1).collect();
        // Split the request into its parts
        let parts: Vec<&str> = req.split("\n").map(|line| line.trim()).collect();

        // Parse out first line
        let first = parts[0].split(" ").collect::<Vec<_>>();
        if first.len() != 3 {
            return Err(RequestError::TooManyValues);
        }
        let method = Method::parse(first[0])?;
        let uri_parts: Vec<&str> = first[1].split("?").collect();
        let uri = sanatise_uri(uri_parts[0]);
        let query = match uri_parts.get(1) {
            Some(part) => Query::parse(part.to_string()),
            None => Query::default(),
        };
        // We only support HTTP/1.1
        if first[2].to_uppercase().as_str() != "HTTP/1.1" {
            return Err(RequestError::BadProtocol);
        }

        // Parse out headers
        let headers = parts
            .iter()
            .skip(1)
            .filter_map(|&line| Header::parse(line))
            .collect::<Vec<_>>();

        // Return the request
        Ok(Request {
            method,
            uri,
            query,
            headers,
        })
    }
}

fn sanatise_uri(uri: &str) -> String {
    uri.trim_matches('.')
        .split("/")
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("/")
}
