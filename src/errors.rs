use reqwest::{self, header::InvalidHeaderValue};
use std::result;
use tonic::transport::Error as TTError;
use tonic::Status;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Error { message }
    }
}

pub type Result<T> = result::Result<T, Error>;

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Error {
        Error {
            message: format!("{}", error),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error {
            message: format!("{}", error),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(error: jsonwebtoken::errors::Error) -> Error {
        Error {
            message: format!("{}", error),
        }
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(error: InvalidHeaderValue) -> Error {
        Error {
            message: format!("{}", error),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        Error {
            message: format!("{}", error),
        }
    }
}

impl From<TTError> for Error {
    fn from(error: TTError) -> Error {
        Error {
            message: format!("{}", error),
        }
    }
}

impl From<Status> for Error {
    fn from(error: Status) -> Error {
        Error {
            message: format!("{}", error),
        }
    }
}
