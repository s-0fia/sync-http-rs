use request::Request;
use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener},
};

pub mod errors;
pub mod mime;
pub mod request;

pub fn listen() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // Read one stream and then terminate
    let (mut stream, _addr) = listener.accept()?;
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
    let request = Request::parse(request);
    dbg!(&request);
    stream.shutdown(Shutdown::Read)?;

    let response = b"HTTP/1.1 200\r\n\r\n";
    stream.write_all(response)?;

    stream.shutdown(Shutdown::Write)?;

    Ok(())
}
