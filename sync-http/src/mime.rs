#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct ContentType(pub MediaType, pub MimeType);

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum MediaType {
    All,
    Application,
    Audio,
    Font,
    Image,
    Model,
    Text,
    Video,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum MimeType {
    All,

    // Application Types
    JSON,
    OctetStream,

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

impl MediaType {
    pub fn parse(media_type: &str) -> Option<Self> {
        Some(match media_type {
            "*" => MediaType::All,
            "application" => MediaType::Application,
            "audio" => MediaType::Audio,
            "font" => MediaType::Font,
            "image" => MediaType::Image,
            "model" => MediaType::Model,
            "text" => MediaType::Text,
            "video" => MediaType::Video,
            _ => None?,
        })
    }
}

impl MimeType {
    pub fn parse(mime_type: &str) -> Option<Self> {
        Some(match mime_type {
            "*" => MimeType::All,
            "json" => MimeType::JSON,
            "octet-stream" => MimeType::OctetStream,
            "apng" => MimeType::APNG,
            "avif" => MimeType::AVIF,
            "gif" => MimeType::GIF,
            "jpeg" => MimeType::JPEG,
            "png" => MimeType::PNG,
            "svg+xml" => MimeType::SVG,
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
            MimeType::JSON | MimeType::OctetStream => MediaType::Application,
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

impl ContentType {
    pub fn parse(content_type: String) -> Option<Self> {
        let parts: Vec<&str> = content_type.split("/").collect();
        Self(MediaType::parse(parts[0])?, MimeType::parse(parts[1])?).validate()
    }

    pub fn validate(self) -> Option<Self> {
        if self.0 == MediaType::All {
            return Some(self);
        }

        if self.0 == self.1.associated_media() {
            Some(self)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! check_new_mimes {
        ($v:expr) => {
            match $v {
                MimeType::All
                | MimeType::JSON
                | MimeType::OctetStream
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
        assert_eq!(ct, Some(ContentType(MediaType::All, MimeType::All)));
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
            assert_eq!(ct, Some(ContentType(MediaType::Application, mime_types[i])));
        }
    }

    #[test]
    fn parse_images() {
        const NUM_TYPES: usize = 7;
        let image_types: [_; NUM_TYPES] = ["apng", "avif", "gif", "jpeg", "png", "svg+xml", "webp"];
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
            assert_eq!(ct, Some(ContentType(MediaType::Image, mime_types[i])));
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
            assert_eq!(ct, Some(ContentType(MediaType::Text, mime_types[i])));
        }
    }
}
