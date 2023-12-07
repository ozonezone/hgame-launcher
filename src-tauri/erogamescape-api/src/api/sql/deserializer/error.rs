use serde::{de, ser};
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeserializeError {
    #[error("Failed to parse: {0}")]
    Parse(String),
    #[error("Unsupported: {0}")]
    Unsupported(String),
    #[error("Error: {0}")]
    Message(String),
}
pub(super) type Result<T> = std::result::Result<T, DeserializeError>;

impl ser::Error for DeserializeError {
    fn custom<T: Display>(msg: T) -> Self {
        DeserializeError::Message(msg.to_string())
    }
}

impl de::Error for DeserializeError {
    fn custom<T: Display>(msg: T) -> Self {
        DeserializeError::Message(msg.to_string())
    }
}
