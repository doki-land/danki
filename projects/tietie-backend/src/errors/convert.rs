use super::*;
use std::env::VarError;

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        Self::DatabaseError { message: value.to_string() }
    }
}

impl From<VarError> for AppError {
    fn from(error: VarError) -> Self {
        Self::DecodeError { format: "env".to_string(), message: error.to_string() }
    }
}

impl From<std::io::Error> for AppError {
    #[track_caller]
    fn from(error: std::io::Error) -> Self {
        let location = std::panic::Location::caller();
        Self::IoError { path: location.to_string(), message: error.to_string() }
    }
}
