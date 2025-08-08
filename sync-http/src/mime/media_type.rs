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
