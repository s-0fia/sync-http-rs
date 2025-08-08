use crate::{
    errors::FailedToCompileRoute,
    query::Query,
    request::{Method, Request},
    ServerResult,
};
use regex::Regex;
use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    sync::{mpsc::Receiver, Arc, Mutex},
    thread,
};

pub trait ServerStream {
    fn close_response(&self) -> ServerResult<()>;
    fn write_empty(&mut self) -> ServerResult<()>;
    fn write_bytes<'a>(&mut self, response: impl Into<&'a [u8]>) -> ServerResult<()>;
}

pub type GetHandler = dyn Fn(Query) -> ServerResult<String>;
pub type GetHandlerMap = (Regex, &'static GetHandler);

pub struct ServerBuilder {
    ip_address: Option<String>,
    port: Option<u16>,
    ttl: Option<u32>,
    shutdown: Option<Receiver<()>>,
    get_handlers: Vec<GetHandlerMap>,
}

static mut RUNNING: bool = false;

#[derive()]
pub struct Server {
    listener: TcpListener,
    shutdown: Option<Arc<Mutex<Receiver<()>>>>,
    get_handlers: Vec<GetHandlerMap>,
}

impl Server {
    fn new(
        listener: TcpListener,
        shutdown: Option<Receiver<()>>,
        get_handlers: Vec<GetHandlerMap>,
    ) -> Self {
        if let Some(shutdown) = shutdown {
            let shutdown = Some(Arc::from(Mutex::new(shutdown)));
            Self {
                listener,
                shutdown,
                get_handlers,
            }
        } else {
            Self {
                listener,
                shutdown: None,
                get_handlers,
            }
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
            shutdown: None,
            get_handlers: vec![],
        }
    }

    pub fn request(&mut self) -> ServerResult<(TcpStream, Request)> {
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

    pub fn handle_loop(&mut self) -> ServerResult<()> {
        unsafe {
            RUNNING = true;
        }
        if let Some(shutdown) = self.shutdown.clone() {
            thread::spawn(move || {
                let lock = shutdown.lock().unwrap();
                if lock.recv().is_ok() {
                    unsafe { RUNNING = false };
                }
            });
        }
        while unsafe { RUNNING } {
            let (mut stream, req) = self.request()?;
            dbg!(&req);
            match req.method {
                Method::Get => self.handle_get(&mut stream, req)?,
                Method::Post => todo!(),
            }
        }

        Ok(())
    }

    pub fn handle_get(&self, stream: &mut TcpStream, request: Request) -> ServerResult<()> {
        if request.method != Method::Get {
            panic!("Non get request being handled by handle_get().");
        }

        for (path, handler) in self.get_handlers.iter() {
            if path.is_match(request.uri.as_str()) {
                let response = handler(Query::default())?;
                stream.write_bytes(response.as_bytes())?;
                break;
            }
        }

        Ok(())
    }
}

impl ServerStream for TcpStream {
    fn close_response(&self) -> ServerResult<()> {
        self.shutdown(Shutdown::Write)?;
        Ok(())
    }

    fn write_empty(&mut self) -> ServerResult<()> {
        let response = b"HTTP/1.1 200\r\n\r\n";
        self.write_all(response)?;
        self.close_response()
    }

    fn write_bytes<'a>(&mut self, response: impl Into<&'a [u8]>) -> ServerResult<()> {
        let protocol = b"HTTP/1.1 200\r\n\r\n";
        self.write_all(protocol)?;
        let response: &[u8] = response.into();
        self.write_all(response)?;
        self.close_response()
    }
}

impl ServerBuilder {
    pub fn ip_address(mut self, ip: String) -> Self {
        self.ip_address = Some(ip);
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn ttl(mut self, ttl: u32) -> Self {
        self.ttl = Some(ttl);
        self
    }

    pub fn shutdown(mut self, shutdown: Receiver<()>) -> Self {
        self.shutdown = Some(shutdown);
        self
    }

    pub fn get(mut self, route: &str, handler: &'static GetHandler) -> ServerResult<Self> {
        let route = route
            .replace("/", r"\/")
            .replace(".", r"\.")
            .replace("*", r"[A-Za-z0-9\-_~.]*");
        let route = format!("^{route}$");
        if let Ok(re_route) = Regex::new(route.as_str()) {
            self.get_handlers.push((re_route, handler));
        } else {
            Err(FailedToCompileRoute)?;
        }
        Ok(self)
    }

    pub fn bind(self) -> ServerResult<()> {
        let ip = self.ip_address.unwrap_or("127.0.0.1".into());
        let port = self.port.unwrap_or(8080);
        let addr = format!("{ip}:{port}");
        let listener = TcpListener::bind(addr)?;

        if let Some(ttl) = self.ttl {
            listener.set_ttl(ttl)?;
        }

        Server::new(listener, self.shutdown, self.get_handlers).handle_loop()
    }
}
