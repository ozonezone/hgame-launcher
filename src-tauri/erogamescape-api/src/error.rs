use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    SqlDeserialize(#[from] crate::api::sql::deserializer::error::DeserializeError),
    #[error("Failed to execute sql on erogamescape: {0}")]
    SqlExecute(String),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("Failed to parse html: {0}")]
    HtmlParse(String),
}
