use crate::util::error_handling::error_type::ErrorType;
use std::error::Error;
use std::fmt::Debug;
use std::fmt;
use actix_web::http::StatusCode;
use actix_web::ResponseError;

#[derive(Debug)]
pub struct ApplicationError {
    message: String,
    error_type: ErrorType
}

pub fn from_serializable<T: Debug>(e: T) -> ApplicationError {
    ApplicationError::new(format!("{:?}", e).as_str())
}

impl ApplicationError {
    pub fn new_with_type(message: &str, error_type: ErrorType) -> ApplicationError {
        ApplicationError {
            message: message.to_string(),
            error_type
        }
    }

    pub fn new(message: &str) -> ApplicationError {
        ApplicationError {
            message: message.to_string(),
            error_type: ErrorType::GenericFailure
        }
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", self.message, self.error_type)
    }
}

impl ResponseError for ApplicationError {
    fn status_code(&self) -> StatusCode {
        self.error_type.http_error()
    }
}

impl Error for ApplicationError {}