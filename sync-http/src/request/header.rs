use super::ContentType;

// TODO: Add more headers
#[derive(Debug, PartialEq, Clone)]
pub enum Header {
    Host(String),
    UserAgent(String),
    Accept(Vec<ContentType>),
}

impl Header {
    pub fn parse(header: &str) -> Option<Self> {
        let parts = header.split(":");
        let head_type: String = parts.clone().take(1).collect();
        let head_value: String = parts.skip(1).collect::<Vec<_>>().join(":");
        let head_value = head_value.trim().to_string();

        // Headers are case insensitive
        Some(match head_type.to_uppercase().trim() {
            "HOST" => Self::Host(head_value),
            "USER-AGENT" => Self::UserAgent(head_value),
            "ACCEPT" => Self::Accept(ContentType::parse_many(head_value)?),
            _ => None?,
        })
    }
}
