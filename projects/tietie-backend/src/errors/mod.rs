use poem::{error::ResponseError, http::StatusCode, IntoResponse};
use poem_openapi::{
    payload::Json,
    registry::{MetaResponses, Registry},
    ApiResponse,
};
use sqlx::Error;
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<Json<T>, AppError>;

#[derive(Debug)]
pub enum AppError {
    DatabaseError { message: String },
}

impl std::error::Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::DatabaseError { message } => {
                write!(f, "{}", message)
            }
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: Error) -> Self {
        Self::DatabaseError { message: value.to_string() }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> poem::Response {
        todo!()
    }
}

impl ResponseError for AppError {
    fn status(&self) -> StatusCode {
        match self {
            AppError::DatabaseError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl ApiResponse for AppError {
    fn meta() -> MetaResponses {
        MetaResponses { responses: vec![] }
    }

    fn register(_: &mut Registry) {}
}
