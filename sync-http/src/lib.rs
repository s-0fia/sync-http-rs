use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener},
};

pub mod mime;

pub fn listen() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // Read one stream and then terminate
    let (mut stream, _addr) = listener.accept()?;
    let mut buf = [0; 128];
    let mut stop = String::new();
    'outer: loop {
        let length = stream.read(&mut buf)?;
        for b in buf.iter().take(length) {
            let ch = *b as char;
            print!("{}", ch);
            match ch {
                '\r' => stop.push(ch),
                '\n' => {
                    if !stop.is_empty() {
                        stop.push(ch)
                    }
                }
                _ => {
                    if !stop.is_empty() {
                        stop = String::new()
                    }
                }
            }
            if stop.contains("\r\n\r\n") {
                break 'outer;
            }
        }
    }
    stream.shutdown(Shutdown::Read)?;

    let response = b"HTTP/1.1 200\r\n\r\n";
    stream.write_all(response)?;

    stream.shutdown(Shutdown::Write)?;

    Ok(())
}
