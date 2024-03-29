use std::fmt;
use std::fmt::Debug;
use crate::client::client_error::ClientError;

#[derive(thiserror::Error)]
#[non_exhaustive] 
/// Common error types to handle errors from Resolver.
/// 
/// These types of errors are used to handle errors originating from 
/// this implementation of Resolver which works asynchronously.
pub enum ResolverError {
    /// An error io connection.
    /// 
    /// This error occurs when the connection to the server fails or
    /// the server does not respond.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// An error with a message to display.
    #[error("{0}")]
    Message(&'static str), 

    /// An error when the answer is empty.
    /// 
    /// This error occurs when the resolver receives an empty answer.
    /// This happens when the future `LookupFutureStub` is polled for
    /// the first time so the corresponding query has not been set yet.
    #[error("empty query")]
    EmptyQuery,

    /// An error when the resolver surpassed the number of retries allowed.
    #[error("retries limit exceeded")]
    RetriesLimitExceeded,

    /// An error with the format of the received response.
    /// 
    /// This error occurs after parsing arriving response datagram and
    /// the response is not a valid DNS message or is not a valid response.
    #[error("parse response error: {0}")]
    Parse(String),
}

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

impl Clone for ResolverError {
    fn clone(&self) -> Self {
        match self {
            ResolverError::Io(io) => Self::from(std::io::Error::from(io.kind())),
            ResolverError::Message(err) => ResolverError::Message(err),
            ResolverError::EmptyQuery => ResolverError::EmptyQuery,
            ResolverError::RetriesLimitExceeded => ResolverError::RetriesLimitExceeded,
            ResolverError::Parse(err) => ResolverError::Parse(err.to_string()),
        }
    }
}
