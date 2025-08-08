use sync_http::server::Server;

fn main() {
    let mut serv = Server::create().bind().unwrap();

    let (mut stream, req) = serv.get_request().unwrap();
    dbg!(req);
    serv.write_empty(&mut stream);
}
