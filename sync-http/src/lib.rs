use std::{io::Read, net::TcpListener};

pub fn listen() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        let bytes = stream?.bytes();
        for byte in bytes {
            print!("{}", byte? as char);
        }
        println!();
    }

    Ok(())
}
