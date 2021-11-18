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
    InvalidApiKeyActions(Value),
    #[error("indexes field value `{0}` is invalid. It should be an array of string representing index names.")]
    InvalidApiKeyIndexes(Value),
    #[error("expiresAt field value `{0}` is invalid. It should be in ISO-8601 format to represents a date or datetime in the future or specified as a null value. e.g. 'YYYY-MM-DD' or 'YYYY-MM-DDTHH:MM:SS'.")]
    InvalidApiKeyExpiresAt(Value),
    #[error("description field value `{0}` is invalid. It should be a string or specified as a null value.")]
    InvalidApiKeyDescription(Value),
    #[error("API key `{0}` not found.")]
    ApiKeyNotFound(String),
    #[error("Internal error: {0}")]
    Internal(Box<dyn Error + Send + Sync + 'static>),
}

internal_error!(AuthResolverError: heed::Error, std::io::Error);

impl ErrorCode for AuthResolverError {
    fn error_code(&self) -> Code {
        match self {
            Self::MissingParameter(_) => Code::MissingParameter,
            Self::InvalidApiKeyActions(_) => Code::InvalidApiKeyActions,
            Self::InvalidApiKeyIndexes(_) => Code::InvalidApiKeyIndexes,
            Self::InvalidApiKeyExpiresAt(_) => Code::InvalidApiKeyExpiresAt,
            Self::InvalidApiKeyDescription(_) => Code::InvalidApiKeyDescription,
            Self::ApiKeyNotFound(_) => Code::ApiKeyNotFound,
            Self::Internal(_) => Code::Internal,
        }
    }
}
