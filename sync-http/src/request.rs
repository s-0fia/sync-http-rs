use crate::errors::*;
use crate::mime::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Request {
    pub method: Method,
    pub uri: String,
    pub headers: Vec<Header>,
}

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

// TODO: Add more headers
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Header {
    Host(String),
    UserAgent(String),
    Accept(ContentType),
}

impl Header {
    pub fn parse(header: &str) -> Option<Self> {
        let parts = header.split(":");
        let head_type: String = parts.clone().take(1).collect();
        let head_value: String = parts.skip(1).collect::<Vec<_>>().join(":");
        let head_value = head_value.trim().to_string();

        // Headers are case insensitive
        Some(match head_type.to_uppercase().trim() {
            "HOST" => Self::Host(head_value),
            "USER-AGENT" => Self::UserAgent(head_value),
            "ACCEPT" => Self::Accept(ContentType::parse(head_value)?),
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
            .filter_map(|&line| Header::parse(line))
            .collect::<Vec<_>>();

        Ok(Request {
            method,
            uri,
            headers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_request() {
        let request = Request::parse(
            "GET / HTTP/1.1\r\n\
            Host: localhost:8080\r\n\
            User-Agent: curl/8.5.0\r\n\
            Accept: */*\r\n\r\n"
                .into(),
        );

        let accurate = Request {
            method: Method::Get,
            uri: "/".into(),
            headers: vec![
                Header::Host("localhost:8080".into()),
                Header::UserAgent("curl/8.5.0".into()),
                Header::Accept(ContentType(MediaType::All, MimeType::All)),
            ],
        };

        assert_eq!(request, Ok(accurate));
    }

    #[test]
    fn header_parsing() {
        const NUM_TESTS: usize = 5;
        let headers: [&str; NUM_TESTS] = [
            "Host: localhost:8080",
            "user-AGENT: curl/8.5.0",
            "AcCePt: text/html",
            "Pragma: no-cache",
            "",
        ];
        let test_vals: [Option<_>; NUM_TESTS] = [
            Some(Header::Host("localhost:8080".into())),
            Some(Header::UserAgent("curl/8.5.0".into())),
            Some(Header::Accept(ContentType(MediaType::Text, MimeType::HTML))),
            None,
            None,
        ];

        // Will fail to compile if more headers are added
        match test_vals[0].clone().unwrap() {
            Header::Host(_) | Header::UserAgent(_) | Header::Accept(_) => {}
        }

        for i in 0..NUM_TESTS {
            let parsed = Header::parse(headers[i]);
            assert_eq!(parsed, test_vals[i].clone());
        }
    }

    #[test]
    fn method_parsing() {
        const NUM_TESTS: usize = 4;
        let methods: [_; NUM_TESTS] = ["GET", "POST", "get", "post"];
        let test_vals: [Result<_, _>; NUM_TESTS] = [
            Ok(Method::Get),
            Ok(Method::Post),
            Err(RequestError::BadMethod),
            Err(RequestError::BadMethod),
        ];

        // Will fail to compile if more methods are added
        match Method::Get {
            Method::Get | Method::Post => {}
        }

        for i in 0..NUM_TESTS {
            let parsed = Method::parse(methods[i]);
            assert_eq!(parsed, test_vals[i]);
        }
    }
}
