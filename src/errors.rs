use crate::api::grpc::google::cloud::dialogflow::v2beta1::{
    StreamingDetectIntentRequest, StreamingDetectIntentResponse,
};
use crate::api::grpc::google::cloud::speechtotext::v1::{
    StreamingRecognizeRequest, StreamingRecognizeResponse,
};
use gouth::Error as GAuthError;
use prost::DecodeError as ProstDecodeError;
use std::result;
use tokio::sync::mpsc::error::SendError;
use tonic::metadata::errors::InvalidMetadataValue;
use tonic::transport::Error as TTError;
use tonic::Status as TStatus;

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub code: Option<String>,
}

impl Error {
    pub fn new(message: String) -> Self {
        Error {
            message,
            code: None,
        }
    }
    pub fn new_with_code(message: String, code: String) -> Self {
        Error {
            message,
            code: Some(code),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Error {
        Error {
            message: format!("{}", error),
            code: None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error {
            message: format!("{}", error),
            code: None,
        }
    }
}

impl From<TTError> for Error {
    fn from(error: TTError) -> Error {
        Error {
            message: format!("{}", error),
            code: None,
        }
    }
}

impl From<GAuthError> for Error {
    fn from(error: GAuthError) -> Error {
        Error {
            message: format!("{}", error),
            code: None,
        }
    }
}

impl From<TStatus> for Error {
    fn from(error: TStatus) -> Error {
        Error {
            message: format!("{}", error),
            code: None,
        }
    }
}

impl From<SendError<StreamingRecognizeRequest>> for Error {
    fn from(error: SendError<StreamingRecognizeRequest>) -> Error {
        Error {
            message: format!("{}", error),
            code: None,
        }
    }
}

impl From<SendError<StreamingRecognizeResponse>> for Error {
    fn from(error: SendError<StreamingRecognizeResponse>) -> Error {
        Error {
            message: format!("{}", error),
            code: None,
        }
    }
}

impl From<SendError<StreamingDetectIntentRequest>> for Error {
    fn from(error: SendError<StreamingDetectIntentRequest>) -> Error {
        Error {
            message: format!("{}", error),
            code: None,
        }
    }
}

impl From<SendError<StreamingDetectIntentResponse>> for Error {
    fn from(error: SendError<StreamingDetectIntentResponse>) -> Error {
        Error {
            message: format!("{}", error),
            code: None,
        }
    }
}

impl From<ProstDecodeError> for Error {
    fn from(error: ProstDecodeError) -> Error {
        Error {
            message: format!("{}", error),
            code: None,
        }
    }
}

impl From<InvalidMetadataValue> for Error {
    fn from(error: InvalidMetadataValue) -> Error {
        Error {
            message: format!("{}", error),
            code: None,
        }
    }
}
