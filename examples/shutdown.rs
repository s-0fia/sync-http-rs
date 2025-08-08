use std::{
    sync::mpsc::{channel, Sender},
    thread,
    time::Duration,
};
use sync_http::{query::Query, server::Server, ServerResult};

static mut SHUTDOWN: Option<Sender<()>> = None;

fn main() -> ServerResult<()> {
    let (send, recv) = channel();
    unsafe {
        SHUTDOWN = Some(send);
    }
    Server::create()
        .ip_address("192.168.126.128".into())
        .get("close", &shutdown)?
        .get("*", &index)?
        .shutdown(recv)
        .bind()
}

fn index(_uri: String, _query: Query) -> ServerResult<String> {
    Ok(r#"<html>
        <head>
            <title>Shutdown example</title>
        </head>
        <body>
            <form action="./close">
                <input type="submit" value="Shut Server Down" />
            </form>
        </body>
        </html>
        "#
    .to_string())
}

fn shutdown(_uri: String, _query: Query) -> ServerResult<String> {
    if let Some(shutdown) = unsafe { SHUTDOWN.clone() } {
        shutdown.send(())?;
        // Sleep 100 ms just to ensure that the
        // thread recieves the shutdown signal
        thread::sleep(Duration::from_millis(100));
    }
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
