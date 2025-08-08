use super::*;

#[test]
fn example_request() {
    let request = Request::parse(
        "GET / HTTP/1.1\r\n\
            Host: localhost:8080\r\n\
            User-Agent: curl/8.5.0\r\n\
            Accept: */*\r\n\r\n"
            .into(),
    );

    let accurate = Request {
        method: Method::Get,
        uri: "/".into(),
        query: Query::default(),
        headers: vec![
            Header::Host("localhost:8080".into()),
            Header::UserAgent("curl/8.5.0".into()),
            Header::Accept(vec![ContentType(
                MediaType::All,
                MimeType::All,
                MimeSuffix::None,
                1.0,
            )]),
        ],
    };

    assert_eq!(request, Ok(accurate));
}

#[test]
fn header_parsing() {
    const NUM_TESTS: usize = 5;
    let headers: [&str; NUM_TESTS] = [
        "Host: localhost:8080",
        "user-AGENT: curl/8.5.0",
        "AcCePt: text/html",
        "Pragma: no-cache",
        "",
    ];
    let test_vals: [Option<_>; NUM_TESTS] = [
        Some(Header::Host("localhost:8080".into())),
        Some(Header::UserAgent("curl/8.5.0".into())),
        Some(Header::Accept(vec![ContentType(
            MediaType::Text,
            MimeType::HTML,
            MimeSuffix::None,
            1.0,
        )])),
        None,
        None,
    ];

    // Will fail to compile if more headers are added
    match test_vals[0].clone().unwrap() {
        Header::Host(_) | Header::UserAgent(_) | Header::Accept(_) => {}
    }

    for i in 0..NUM_TESTS {
        let parsed = Header::parse(headers[i]);
        assert_eq!(parsed, test_vals[i].clone());
    }
}

#[test]
fn method_parsing() {
    const NUM_TESTS: usize = 4;
    let methods: [_; NUM_TESTS] = ["GET", "POST", "get", "post"];
    let test_vals: [Result<_, _>; NUM_TESTS] = [
        Ok(Method::Get),
        Ok(Method::Post),
        Err(RequestError::BadMethod),
        Err(RequestError::BadMethod),
    ];

    // Will fail to compile if more methods are added
    match Method::Get {
        Method::Get | Method::Post => {}
    }

    for i in 0..NUM_TESTS {
        let parsed = Method::parse(methods[i]);
        assert_eq!(parsed, test_vals[i]);
    }
}
