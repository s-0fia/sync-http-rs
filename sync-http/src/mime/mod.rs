mod media_type;
mod mime_type;
mod suffix;
use std::fmt::Debug;

pub use media_type::*;
pub use mime_type::*;
pub use suffix::*;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Clone, Copy)]
pub struct ContentType(pub MediaType, pub MimeType, pub MimeSuffix, pub f64);

impl ContentType {
    pub fn parse_many(content_types: String) -> Option<Vec<Self>> {
        let parts = content_types.split(',');
        let mut output = vec![];
        for part in parts {
            if let Some(ct) = Self::parse(part.to_string()) {
                output.push(ct);
            }
        }

        if output.is_empty() {
            None
        } else {
            Some(output)
        }
    }

    pub fn parse(content_type: String) -> Option<Self> {
        let parts: Vec<&str> = content_type.split(";q=").collect();
        let priority: f64 = if parts.len() > 1 {
            parts[1].parse().ok()?
        } else {
            1.0
        };
        let media: Vec<&str> = parts[0].split('/').collect();
        let mime: Vec<&str> = media[1].split('+').collect();
        let suffix = if mime.len() > 1 {
            MimeSuffix::parse(mime[1])?
        } else {
            MimeSuffix::None
        };
        Self(
            MediaType::parse(media[0])?,
            MimeType::parse(mime[0])?,
            suffix,
            priority,
        )
        .validate()
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

impl Debug for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.2 {
            MimeSuffix::None => write!(f, "ContentType({:?}/{:?};q={})", self.0, self.1, self.3),
            _ => write!(
                f,
                "ContentType({:?}/{:?}+{:?};q={})",
                self.0, self.1, self.2, self.3
            ),
        }
    }
}
