use std::{
    sync::mpsc::channel,
    thread::{self},
    time::Duration,
};
use sync_http::{query::Query, server::Server, ServerResult};

static mut COUNTDOWN: usize = 10;

fn main() -> ServerResult<()> {
    let (send, recv) = channel();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));
            unsafe {
                COUNTDOWN -= 1;
            }
            if unsafe { COUNTDOWN } == 0 {
                break;
            }
            println!("Shutting down in {} seconds.", unsafe { COUNTDOWN });
        }
        send.send(()).unwrap();
    });
    Server::create()
        .ip_address("192.168.126.128".into())
        .get("*", &index)?
        .shutdown(recv)
        .bind()
}

fn index(_uri: String, _query: Query) -> ServerResult<String> {
    Ok(format!(
        r#"<html>
        <head>
            <title>Shutdown example</title>
        </head>
        <body>
            <h1>Shutting down in {} seconds.</h1>
        </body>
        </html>
        "#,
        unsafe { COUNTDOWN }
    ))
}
