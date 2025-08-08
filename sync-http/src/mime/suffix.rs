#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum MimeSuffix {
    None,
    GZip,
    Json,
    WbXML,
    XML,
    Zip,
}

impl MimeSuffix {
    pub fn parse(suffix: &str) -> Option<Self> {
        Some(match suffix {
            "gzip" => MimeSuffix::GZip,
            "json" => MimeSuffix::Json,
            "wbxml" => MimeSuffix::WbXML,
            "xml" => MimeSuffix::XML,
            "zip" => MimeSuffix::Zip,
            _ => MimeSuffix::None,
        })
    }
}
