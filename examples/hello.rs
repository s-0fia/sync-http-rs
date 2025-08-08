use sync_http::{query::Query, server::Server, ServerResult};

fn main() {
    Server::create()
        .ip_address("192.168.126.128".into())
        .get("/", &index)
        .bind()
        .unwrap();
}

fn index(_query: Query) -> ServerResult<String> {
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
