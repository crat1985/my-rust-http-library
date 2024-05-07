use crate::response::{IntoResponse, Response, ResponseBuilder};

#[derive(Debug, Clone)]
pub enum StatusCode {
    Ok = 200,
    Forbidden = 403,
    NotFound = 404,
    LengthRequired = 411,
    UnsupportedMediaType = 415,
    InternalServerError = 500,
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_text = match self {
            StatusCode::Ok => "OK",
            StatusCode::Forbidden => "FORBIDDEN",
            StatusCode::NotFound => "NOT FOUND",
            StatusCode::LengthRequired => "LENGTH REQUIRED",
            StatusCode::UnsupportedMediaType => "UNSUPPORTED MEDIA TYPE",
            StatusCode::InternalServerError => "INTERNAL SERVER ERROR",
        };
        write!(f, "{} {}", self.clone() as u8, status_text)
    }
}

impl IntoResponse for StatusCode {
    fn into_response(self) -> Response {
        ResponseBuilder::new().with_status_code(self).build()
    }
}
