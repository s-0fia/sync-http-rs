use sync_http::server::Server;

fn main() {
    let mut serv = Server::create()
        .ip_address("192.168.126.128".into())
        .bind()
        .unwrap();

    let (mut stream, req) = serv.get_request().unwrap();
    dbg!(req);
    let response = r#"
    <html>
    <head>
        <title> Hello, world!</title>
    </head>
    <body>
        <h1>Hello, world!</h1>
        <p>Example webpage</p>
    </body>
    </html>
    "#;
    serv.write_all(&mut stream, response.as_bytes()).unwrap();
}
