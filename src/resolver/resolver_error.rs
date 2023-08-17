use std::fmt;
use std::fmt::Debug;

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
}

// Debug trait implementation for `?` formatting
impl Debug for ResolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::ResolverError::*;
        match self {
            Io(err) => write!(f, "io error: {}", err),
            Message(err) => write!(f, "{}", err),
        }
    }
}
