use std::{error::Error, net::TcpStream};

use sync_http::{
    query::Query,
    server::{Server, ServerStream},
};

fn main() {
    let mut serv = Server::create()
        .ip_address("192.168.126.128".into())
        .get("/", &index)
        .get("/close?", &shutdown)
        .bind()
        .unwrap();

    loop {
        let (mut stream, req) = serv.request().unwrap();
        dbg!(&req);
        serv.handle_get(&mut stream, req).unwrap();
    }
}

fn index(query: Query) -> Result<String, Box<dyn Error>> {
    Ok(r#"<html>
        <head>
            <title>Hello, world!</title>
        </head>
        <body>
            <h1>Hello, world!</h1>
            <p>Example webpage</p>
            <form action="./close">
                <input type="submit" value="Shut Server Down" />
            </form>
        </body>
        </html>
        "#
    .to_string())
}

fn shutdown(query: Query) -> Result<String, Box<dyn Error>> {
    Ok(r#"<html>
        <head>
            <title>Shutting down</title>
        </head>
        <body>
            <h1>Shutting the server down</h1>
        </body>
        </html>
        "#
    .to_string())
}
