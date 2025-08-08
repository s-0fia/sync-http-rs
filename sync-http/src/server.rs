use std::{
    error::Error,
    io::{self, Read, Write},
    net::{Shutdown, TcpListener},
};

use crate::request::Request;

pub struct ServerBuilder {
    ip_address: Option<String>,
    port: Option<u16>,
    ttl: Option<u32>,
}

pub struct Server {
    listener: TcpListener,
}

impl Server {
    /// Creates a new ServerBuilder which defaults with values of:
    /// - ttl: None (time to live)
    /// - ip_address: 127.0.0.1
    /// - port: 80
    pub fn create() -> ServerBuilder {
        ServerBuilder {
            ip_address: None,
            port: None,
            ttl: None,
        }
    }

    pub fn get_request(&self) -> Result<Request, Box<dyn Error>> {
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
        Ok(request)
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

    pub fn bind(self) -> Result<Server, io::Error> {
        let ip = self.ip_address.unwrap_or("127.0.0.1".into());
        let port = self.port.unwrap_or(80);
        let listener = TcpListener::bind(format!("{ip}:{port}"))?;

        if let Some(ttl) = self.ttl {
            listener.set_ttl(ttl)?;
        }

        Ok(Server { listener })
    }
}
