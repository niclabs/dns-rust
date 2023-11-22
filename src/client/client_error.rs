use std::fmt;
use std::fmt::Debug;

#[derive(thiserror::Error)]
#[non_exhaustive] 
/// Common error type to hold errors from Client.
pub enum ClientError {
    /// An error io connection.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// An error with a message to display.
    #[error("{0}")]
    Message(&'static str), 

    /// An error with the format of the query
    #[error("Format Error: {0}" )]
    FormatError(&'static str),

    #[error("Server Failure: {0}")]
    ServerFailure(&'static str),

    #[error("Name Error: {0}")]
    NameError(&'static str),

    #[error("Not Implemented: {0}")]
    NotImplemented(&'static str),

    #[error("Refused: {0}")]
    Refused(&'static str),

    #[error("Response with error code {0}")]
    ResponseError(u8),

    #[error("Temporary Error: {0}")]
    TemporaryError(&'static str),
}

// Debug trait implementation for `?` formatting
impl Debug for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::ClientError::*;
        match self {
            Io(err) => write!(f, "io error: {}", err),
            Message(err) => write!(f, "{}", err),
            FormatError(err) => write!(f, "Format Error: {}", err),
            ServerFailure(err) => write!(f, "Server Failure: {}", err),
            NameError(err) => write!(f, "Name Error: {}", err),
            NotImplemented(err) => write!(f, "Not Implemented: {}", err),
            Refused(err) => write!(f, "Refused: {}", err),
            ResponseError(err) => write!(f, "Response with error code {}", err),
            TemporaryError(err) => write!(f, "Temporary Error: {}", err),
        }
    }
}
