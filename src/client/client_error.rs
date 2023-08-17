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
}

// Debug trait implementation for `?` formatting
impl Debug for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::ClientError::*;
        match self {
            Io(err) => write!(f, "io error: {}", err),
            Message(err) => write!(f, "{}", err),
        }
    }
}
