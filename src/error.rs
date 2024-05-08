use crate::{
    response::{IntoResponse, Response},
    status_code::StatusCode,
};

#[derive(Debug)]
pub enum Error {
    TcpStreamError(std::io::Error),
}

#[derive(Debug)]
pub enum HttpError {
    //400 Bad Request
    MissingBytesBody,
    MissingStringBody,
    InvalidQuery,

    //411 Length Required
    LengthMissing,
    InvalidLength(std::num::ParseIntError),

    //415 Unsupported Media Type
    ContentTypeMissing,
    InvalidBytesBody(std::io::Error),
    InvalidStringBody(std::string::FromUtf8Error),

    //500 Internal Server Error
    GetPeerAddrError(std::io::Error),
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        match self {
            HttpError::GetPeerAddrError(e) => {
                eprintln!("[ERROR] Error getting peer addr : {e}");
                StatusCode::InternalServerError.into_response()
            }
            HttpError::ContentTypeMissing => {
                println!("[WARN] Content-Type missing");
                StatusCode::UnsupportedMediaType.into_response()
            }
            HttpError::LengthMissing => {
                println!("[WARN] Length missing");
                StatusCode::LengthRequired.into_response()
            }
            HttpError::InvalidLength(e) => {
                println!("[WARN] Invalid length : {e}");
                StatusCode::LengthRequired.into_response()
            }
            HttpError::InvalidBytesBody(e) => {
                println!("[WARN] Invalid bytes body : {e}");
                StatusCode::UnsupportedMediaType.into_response()
            }
            HttpError::InvalidStringBody(e) => {
                println!("[WARN] Invalid String body : {e}");
                StatusCode::UnsupportedMediaType.into_response()
            }
            HttpError::MissingBytesBody => {
                println!("[WARN] Missing bytes body");
                StatusCode::BadRequest.into_response()
            }
            HttpError::MissingStringBody => {
                println!("[WARN] Missing String body");
                StatusCode::BadRequest.into_response()
            }
            HttpError::InvalidQuery => {
                println!("[WARN] Invalid query");
                StatusCode::BadRequest.into_response()
            }
        }
    }
}

impl HttpError {
    pub fn log(&self) {
        let message = match self {
            Self::MissingBytesBody => "Missing bytes body".to_string(),
            Self::MissingStringBody => "Missing String body".to_string(),
            Self::InvalidQuery => "Invalid query".to_string(),
            Self::LengthMissing => "Length missing".to_string(),
            Self::InvalidLength(e) => format!("Invalid length : {e}"),
            Self::ContentTypeMissing => "Content-Type header missing".to_string(),
            Self::InvalidBytesBody(e) => format!("Invalid bytes body : {e}"),
            Self::InvalidStringBody(e) => format!("Invali String body : {e}"),
            Self::GetPeerAddrError(e) => format!("Get peer addr error : {e}"),
        };
        match self {
            Self::MissingBytesBody
            | Self::MissingStringBody
            | Self::InvalidQuery
            | Self::LengthMissing
            | Self::InvalidLength(..)
            | Self::ContentTypeMissing
            | Self::InvalidBytesBody(..)
            | Self::InvalidStringBody(..) => println!("[WARN] {message}"),
            Self::GetPeerAddrError(..) => println!("[ERROR] {message}"),
        }
    }
}
