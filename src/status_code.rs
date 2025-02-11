use crate::response::{IntoResponse, Response, ResponseBuilder};

#[derive(Debug, Clone)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    LengthRequired = 411,
    UnsupportedMediaType = 415,
    InternalServerError = 500,
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_text = match self {
            StatusCode::Ok => "OK",
            StatusCode::BadRequest => "BAD REQUEST",
            StatusCode::Forbidden => "FORBIDDEN",
            StatusCode::NotFound => "NOT FOUND",
            StatusCode::MethodNotAllowed => "METHOD NOT ALLOWED",
            StatusCode::LengthRequired => "LENGTH REQUIRED",
            StatusCode::UnsupportedMediaType => "UNSUPPORTED MEDIA TYPE",
            StatusCode::InternalServerError => "INTERNAL SERVER ERROR",
        };
        write!(f, "{} {}", self.clone() as u16, status_text)
    }
}

impl IntoResponse for StatusCode {
    fn into_response(self) -> Response {
        ResponseBuilder::new().with_status_code(self).build()
    }
}
