use std::fmt::{Display, Formatter};
use aide::OperationOutput;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sqlx::Error;

pub enum AppError {
    DatabaseError { message: String },
}

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
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

impl OperationOutput for AppError {
    type Inner = ();
}
