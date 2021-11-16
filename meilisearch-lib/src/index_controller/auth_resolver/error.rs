use std::error::Error;

use meilisearch_error::Code;
use meilisearch_error::ErrorCode;
use serde_json::Value;

pub type Result<T> = std::result::Result<T, AuthResolverError>;

#[derive(Debug, thiserror::Error)]
pub enum AuthResolverError {
    #[error("`{0}` field is mandatory.")]
    MissingParameter(&'static str),
    #[error("actions field value `{0}` is invalid. It should be an array of string representing action names.")]
    InvalidApiKeyAction(Value),
    #[error("indexes field value `{0}` is invalid. It should be an array of string representing index names.")]
    InvalidApiKeyIndexes(Value),
    #[error("expiresAt field value `{0}` is invalid. It should be in ISO-8601 format to represents a date or datetime in the future or specified as a null value. e.g. 'YYYY-MM-DD' or 'YYYY-MM-DDTHH:MM:SS'.")]
    InvalidApiKeyExpiresAt(Value),
    #[error("description field value `{0}` is invalid. It should be a string or specified as a null value.")]
    InvalidApiKeyDescription(Value),
    #[error("Internal error: {0}")]
    Internal(Box<dyn Error + Send + Sync + 'static>),
}

internal_error!(AuthResolverError: heed::Error, std::io::Error);

impl ErrorCode for AuthResolverError {
    fn error_code(&self) -> Code {
        match self {
            Self::MissingParameter(_) => Code::BadRequest,
            Self::InvalidApiKeyAction(_)
            | Self::InvalidApiKeyIndexes(_)
            | Self::InvalidApiKeyExpiresAt(_)
            | Self::InvalidApiKeyDescription(_) => Code::BadRequest,
            Self::Internal(_) => Code::Internal,
        }
    }
}
