use std::{
    error::Error,
    fmt::{Display, Formatter},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum SkadiError {
    CustomError { message: String },
}
impl Error for SkadiError {}

impl Display for SkadiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CustomError { message } => {
                write!(f, "{}", message)
            }
        }
    }
}
