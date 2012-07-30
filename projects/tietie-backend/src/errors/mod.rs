use poem::IntoResponse;
use sqlx::Error;
use std::fmt::{Display, Formatter};

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
    fn into_response(self) -> poem::Response {
        todo!()
    }
}
