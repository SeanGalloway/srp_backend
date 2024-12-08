use actix_web::http::StatusCode;
use actix_web::HttpResponse;

#[derive(Debug)]
pub enum ErrorType {
    GenericFailure,
    InvalidOperation,
    NotFound,
    Unauthorized,
    AccessDenied,
    OutOfBounds,
    ParsingFailure,
    ProcessingFailure
}

impl ErrorType {
    pub fn http_error(&self) -> StatusCode {
        match *self {
            ErrorType::GenericFailure => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::InvalidOperation => StatusCode::BAD_REQUEST,
            ErrorType::NotFound => StatusCode::NOT_FOUND,
            ErrorType::Unauthorized => StatusCode::UNAUTHORIZED,
            ErrorType::AccessDenied => StatusCode::FORBIDDEN,
            ErrorType::OutOfBounds => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::ParsingFailure => StatusCode::BAD_REQUEST,
            ErrorType::ProcessingFailure => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}