#[derive(Debug)]
pub enum StatusCode {
    Ok = 200,
    Forbidden = 403,
    NotFound = 404,
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusCode::Ok => write!(f, "200 OK"),
            StatusCode::NotFound => write!(f, "404 NOT FOUND"),
            StatusCode::Forbidden => write!(f, "403 FORBIDDEN"),
        }
    }
}
