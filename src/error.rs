use crate::{
    response::{IntoResponse, Response},
    status_code::StatusCode,
};

pub enum Error {
    TcpStreamError(std::io::Error),
}

pub enum HttpError {
    GetPeerAddrError(std::io::Error),
    ContentTypeMissing,
    LengthMissing,
    InvalidLength(std::num::ParseIntError),
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        match self {
            HttpError::GetPeerAddrError(e) => {
                println!("[WARN] Error getting peer addr : {e}");
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
        }
    }
}
