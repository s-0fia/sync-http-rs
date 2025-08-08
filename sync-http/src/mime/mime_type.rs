use super::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum MimeType {
    All,

    // Application Types
    JSON,
    OctetStream,
    XHTML,
    XML,

    // TODO: Audio Types
    // TODO: Font Types
    // Image Types
    APNG,
    AVIF,
    GIF,
    JPEG,
    PNG,
    SVG,
    Webp,

    // TODO: Model Types
    // Text Types
    CSS,
    HTML,
    Javascript,
    Plain,
    // TODO: Video Types
}

impl MimeType {
    pub fn parse(mime_type: &str) -> Option<Self> {
        Some(match mime_type {
            "*" => MimeType::All,
            "json" => MimeType::JSON,
            "octet-stream" => MimeType::OctetStream,
            "xhtml" => MimeType::XHTML,
            "xml" => MimeType::XML,
            "apng" => MimeType::APNG,
            "avif" => MimeType::AVIF,
            "gif" => MimeType::GIF,
            "jpeg" => MimeType::JPEG,
            "png" => MimeType::PNG,
            "svg" => MimeType::SVG,
            "webp" => MimeType::Webp,
            "css" => MimeType::CSS,
            "html" => MimeType::HTML,
            "javascript" => MimeType::Javascript,
            "plain" => MimeType::Plain,
            _ => None?,
        })
    }

    pub fn associated_media(&self) -> MediaType {
        match self {
            MimeType::All => MediaType::All,
            MimeType::JSON | MimeType::OctetStream | MimeType::XHTML | MimeType::XML => {
                MediaType::Application
            }
            MimeType::APNG
            | MimeType::AVIF
            | MimeType::GIF
            | MimeType::JPEG
            | MimeType::PNG
            | MimeType::SVG
            | MimeType::Webp => MediaType::Image,
            MimeType::CSS | MimeType::HTML | MimeType::Javascript | MimeType::Plain => {
                MediaType::Text
            }
        }
    }
}
