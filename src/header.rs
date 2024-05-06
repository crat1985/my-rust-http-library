#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Header {
    ContentType,
    ContentLength,
    Host,
    UserAgent,
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Header::ContentType => write!(f, "Content-Type"),
            Header::ContentLength => write!(f, "Content-Length"),
            Header::Host => write!(f, "Host"),
            Header::UserAgent => write!(f, "User-Agent"),
        }
    }
}

impl Header {
    pub fn parse(header: &str) -> Option<Self> {
        match header.to_lowercase().as_str() {
            "content-type" => Some(Self::ContentType),
            "content-length" => Some(Self::ContentLength),
            "host" => Some(Self::Host),
            "user-agent" => Some(Self::UserAgent),
            header => {
                println!("Header {header} unhandled");
                None
            }
        }
    }
}
