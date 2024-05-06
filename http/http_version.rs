#[derive(Debug)]
pub enum HttpVersion {
    HTTP1_1,
}

impl std::fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpVersion::HTTP1_1 => write!(f, "HTTP/1.1"),
        }
    }
}

impl HttpVersion {
    pub fn parse(http_version: &str) -> Option<Self> {
        match http_version.to_lowercase().as_str() {
            "http/1.1" => Some(Self::HTTP1_1),
            _ => None,
        }
    }
}
