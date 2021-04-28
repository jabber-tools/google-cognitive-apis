use crate::api::grpc::google::cloud::speechtotext::v1::StreamingRecognizeRequest;
use gouth::Error as GAuthError;
use reqwest::{self, header::InvalidHeaderValue};
use std::result;
use tokio::sync::mpsc::error::SendError;
use tonic::transport::Error as TTError;
use tonic::Status as TStatus;

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

impl From<GAuthError> for Error {
    fn from(error: GAuthError) -> Error {
        Error {
            message: format!("{}", error),
        }
    }
}

impl From<TStatus> for Error {
    fn from(error: TStatus) -> Error {
        Error {
            message: format!("{}", error),
        }
    }
}

impl From<SendError<StreamingRecognizeRequest>> for Error {
    fn from(error: SendError<StreamingRecognizeRequest>) -> Error {
        Error {
            message: format!("{}", error),
        }
    }
}
