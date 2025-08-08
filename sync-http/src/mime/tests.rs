use super::*;
macro_rules! check_new_mimes {
    ($v:expr) => {
        match $v {
            MimeType::All
            | MimeType::JSON
            | MimeType::OctetStream
            | MimeType::XHTML
            | MimeType::XML
            | MimeType::APNG
            | MimeType::AVIF
            | MimeType::GIF
            | MimeType::JPEG
            | MimeType::PNG
            | MimeType::SVG
            | MimeType::Webp
            | MimeType::CSS
            | MimeType::HTML
            | MimeType::Javascript
            | MimeType::Plain => {}
        }
    };
}

#[test]
fn parse_all_types() {
    let ct = ContentType::parse("*/*".into());
    assert_eq!(
        ct,
        Some(ContentType(
            MediaType::All,
            MimeType::All,
            MimeSuffix::None,
            1.0
        ))
    );
}

#[test]
fn mismatched_type() {
    let ct = ContentType::parse("application/html".into());
    assert_eq!(ct, None);
}

#[test]
fn parse_applications() {
    const NUM_TYPES: usize = 2;
    let app_types: [_; NUM_TYPES] = ["octet-stream", "json"];
    let mime_types: [_; NUM_TYPES] = [MimeType::OctetStream, MimeType::JSON];

    // Will cause error if more mime types are added
    check_new_mimes!(mime_types[0]);

    for i in 0..NUM_TYPES {
        let ct = ContentType::parse(format!("application/{}", app_types[i]));
        assert_eq!(
            ct,
            Some(ContentType(
                MediaType::Application,
                mime_types[i],
                MimeSuffix::None,
                1.0
            ))
        );
    }
}

#[test]
fn parse_images() {
    const NUM_TYPES: usize = 7;
    let image_types: [_; NUM_TYPES] = ["apng", "avif", "gif", "jpeg", "png", "svg", "webp"];
    let mime_types: [_; NUM_TYPES] = [
        MimeType::APNG,
        MimeType::AVIF,
        MimeType::GIF,
        MimeType::JPEG,
        MimeType::PNG,
        MimeType::SVG,
        MimeType::Webp,
    ];

    // Will cause error if more mime types are added
    check_new_mimes!(mime_types[0]);

    for i in 0..NUM_TYPES {
        let ct = ContentType::parse(format!("image/{}", image_types[i]));
        assert_eq!(
            ct,
            Some(ContentType(
                MediaType::Image,
                mime_types[i],
                MimeSuffix::None,
                1.0
            ))
        );
    }
}

#[test]
fn parse_text() {
    const NUM_TYPES: usize = 4;
    let text_types: [_; NUM_TYPES] = ["css", "html", "javascript", "plain"];
    let mime_types: [_; NUM_TYPES] = [
        MimeType::CSS,
        MimeType::HTML,
        MimeType::Javascript,
        MimeType::Plain,
    ];

    // Will cause error if more mime types are added
    check_new_mimes!(mime_types[0]);

    for i in 0..NUM_TYPES {
        let ct = ContentType::parse(format!("text/{}", text_types[i]));
        assert_eq!(
            ct,
            Some(ContentType(
                MediaType::Text,
                mime_types[i],
                MimeSuffix::None,
                1.0
            ))
        );
    }
}
