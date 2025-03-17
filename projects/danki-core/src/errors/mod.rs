use std::{
    error::Error,
    fmt::{Display, Formatter},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum DankiError {
    CustomError { message: String },
}
impl Error for DankiError {}

impl Display for DankiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CustomError { message } => {
                write!(f, "{}", message)
            }
        }
    }
}
