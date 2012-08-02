use poem::{error::ResponseError, http::StatusCode, IntoResponse};
use poem_openapi::{
    payload::Json,
    registry::{MetaResponses, Registry},
    ApiResponse,
};
use std::error::Error;

use poem_openapi::types::ToJSON;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

mod convert;

pub type Result<T> = std::result::Result<Json<T>, AppError>;

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum AppError {
    EncodeError { format: String, message: String },
    DecodeError { format: String, message: String },
    IoError { path: String, message: String },
    DatabaseError { message: String },
}

impl Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::DatabaseError { message } => {
                write!(f, "{}", message)
            }
            AppError::DecodeError { format, message } => {
                write!(f, "{}: {}", format, message)
            }
            AppError::EncodeError { format, message } => {
                write!(f, "{}: {}", format, message)
            }
            AppError::IoError { path, message } => {
                write!(f, "{}: {}", path, message)
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> poem::Response {
        let json = serde_json::to_string(&self).unwrap_or_default();
        poem::Response::builder().status(self.status()).body(json)
    }
}

impl ResponseError for AppError {
    fn status(&self) -> StatusCode {
        match self {
            AppError::DatabaseError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::DecodeError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::EncodeError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::IoError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl ApiResponse for AppError {
    fn meta() -> MetaResponses {
        MetaResponses { responses: vec![] }
    }

    fn register(_: &mut Registry) {}
}
