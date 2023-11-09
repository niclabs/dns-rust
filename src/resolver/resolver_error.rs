use std::fmt;
use std::fmt::Debug;

use crate::client::client_error::ClientError;

#[derive(thiserror::Error)]
#[non_exhaustive] 
/// Common error type to hold errors from Resolver.
pub enum ResolverError {
    /// An error io connection.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// An error with a message to display.
    #[error("{0}")]
    Message(&'static str), 

    /// An error when the answer is empty.
    #[error("empty query")]
    EmptyQuery,

    /// An error when the resolver surpassed the number of retries allowed.
    #[error("retries limit exceeded")]
    RetriesLimitExceeded,

    #[error("parse response error: {0}")]
    Parse(String),
}

// Debug trait implementation for `?` formatting
impl Debug for ResolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::ResolverError::*;
        match self {
            Io(err) => write!(f, "io error: {}", err),
            Message(err) => write!(f, "Error Response: {}", err),
            EmptyQuery => write!(f, "Empty query"),
            RetriesLimitExceeded => write!(f, "Retries limit exceeded"),
            Parse(err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl From<ClientError> for ResolverError {
    fn from(err: ClientError) -> Self {
        match err {
            ClientError::Io(err) => ResolverError::Io(err),
            ClientError::Message(err) => ResolverError::Message(err),
            ClientError::FormatError(err) => ResolverError::Parse(err.to_string()),
            ClientError::ServerFailure(err) => ResolverError::Parse(err.to_string()),
            ClientError::NameError(err) => ResolverError::Parse(err.to_string()),
            ClientError::NotImplemented(err) => ResolverError::Parse(err.to_string()),
            ClientError::Refused(err) => ResolverError::Parse(err.to_string()),
            ClientError::ResponseError(err) => ResolverError::Parse(err.to_string()),
            ClientError::TemporaryError(err) => ResolverError::Parse(err.to_string()),
        }
    }
    
}
