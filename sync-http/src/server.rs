use crate::{
    query::Query,
    request::{Method, Request},
};
use std::{
    error::Error,
    io::{self, Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
};

pub trait ServerStream {
    fn close_response(&self) -> Result<(), Box<dyn Error>>;
    fn write_empty(&mut self) -> Result<(), Box<dyn Error>>;
    fn write_bytes<'a>(&mut self, response: impl Into<&'a [u8]>) -> Result<(), Box<dyn Error>>;
}

pub type GetHandler = dyn Fn(Query) -> Result<String, Box<dyn Error>>;
pub type GetHandlerMap = (String, &'static GetHandler);

pub struct ServerBuilder {
    ip_address: Option<String>,
    port: Option<u16>,
    ttl: Option<u32>,
    get_handlers: Vec<GetHandlerMap>,
}

#[derive()]
pub struct Server {
    listener: TcpListener,
    get_handlers: Vec<GetHandlerMap>,
}

impl Server {
    fn new(listener: TcpListener, get_handlers: Vec<GetHandlerMap>) -> Self {
        Self {
            listener,
            get_handlers,
        }
    }
    /// Creates a new ServerBuilder which defaults with values of:
    /// - ttl: None (time to live)
    /// - ip_address: 127.0.0.1
    /// - port: 8080
    pub fn create() -> ServerBuilder {
        ServerBuilder {
            ip_address: None,
            port: None,
            ttl: None,
            get_handlers: vec![],
        }
    }

    pub fn request(&mut self) -> Result<(TcpStream, Request), Box<dyn Error>> {
        let (mut stream, _addr) = self.listener.accept()?;
        let mut buf = [0; 128];
        let mut request = String::new();
        loop {
            let length = stream.read(&mut buf)?;
            for b in buf.iter().take(length) {
                let ch = *b as char;
                request.push(ch);
            }
            if request.contains("\r\n\r\n") {
                break;
            }
        }
        let request = Request::parse(request)?;
        stream.shutdown(Shutdown::Read)?;
        Ok((stream, request))
    }

    pub fn handle_get(
        &self,
        stream: &mut TcpStream,
        request: Request,
    ) -> Result<(), Box<dyn Error>> {
        if request.method != Method::Get {
            panic!("Non get request being handled by handle_get().");
        }

        for (path, handler) in self.get_handlers.iter() {
            if request.uri.eq(path) {
                let response = handler(Query::default())?;
                stream.write_bytes(response.as_bytes())?;
                break;
            }
        }

        Ok(())
    }
}

impl ServerStream for TcpStream {
    fn close_response(&self) -> Result<(), Box<dyn Error>> {
        self.shutdown(Shutdown::Write)?;
        Ok(())
    }

    fn write_empty(&mut self) -> Result<(), Box<dyn Error>> {
        let response = b"HTTP/1.1 200\r\n\r\n";
        self.write_all(response)?;
        self.close_response()
    }

    fn write_bytes<'a>(&mut self, response: impl Into<&'a [u8]>) -> Result<(), Box<dyn Error>> {
        let protocol = b"HTTP/1.1 200\r\n\r\n";
        self.write_all(protocol)?;
        let response: &[u8] = response.into();
        self.write_all(response)?;
        self.close_response()
    }
}

impl ServerBuilder {
    pub fn ip_address(self, ip: String) -> Self {
        let ip_address = Some(ip);
        Self { ip_address, ..self }
    }

    pub fn port(self, port: u16) -> Self {
        let port = Some(port);
        Self { port, ..self }
    }

    pub fn ttl(self, ttl: u32) -> Self {
        let ttl = Some(ttl);
        Self { ttl, ..self }
    }

    pub fn get(mut self, route: &str, handler: &'static GetHandler) -> Self {
        self.get_handlers.push((route.to_string(), handler));
        self
    }

    pub fn bind(self) -> Result<Server, io::Error> {
        let ip = self.ip_address.unwrap_or("127.0.0.1".into());
        let port = self.port.unwrap_or(8080);
        let addr = format!("{ip}:{port}");
        let listener = TcpListener::bind(addr)?;

        if let Some(ttl) = self.ttl {
            listener.set_ttl(ttl)?;
        }

        Ok(Server::new(listener, self.get_handlers))
    }
}
