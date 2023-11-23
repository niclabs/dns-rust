use std::fmt;
use std::fmt::Debug;

#[derive(thiserror::Error)]
#[non_exhaustive] 
/// Common error types to handle errors from the client.
/// 
/// Error of type `ClientError` are returned to the Client after the Resolver
/// has finished its job. 
/// 
/// These types of error may correspond to errors originating from the
/// the Name Server which answered the query, returning a DNS message
/// with an error code different from 0. Or they may correspond to errors
/// originating from the Resolver itself.
pub enum ClientError {
    /// An error io connection.
    /// 
    /// This error occurs when the connection to the server fails or
    /// the server does not respond.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// An error with a message to display.
    #[error("{0}")]
    Message(&'static str), 

    /// An error with the format of the message.
    /// 
    /// [RFC 1035]: https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1\
    /// 
    /// When this error occurs, the RCODE field of the header section of the 
    /// response message is set to 1.
    /// 
    /// Format error - The name server was unable to interpret the query.
    #[error("Format Error: {0}" )]
    FormatError(&'static str),

    /// An error when the server fails.
    /// 
    /// [RFC 1035]: https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1\
    /// 
    /// When this error occurs, the RCODE field of the header section of the 
    /// response message is set to 2.
    /// 
    /// Server failure - The name server was unable to process this query due to a
    /// problem with the name server.
    #[error("Server Failure: {0}")]
    ServerFailure(&'static str),

    /// An error when the name does not exist.
    /// 
    /// [RFC 1035]: https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1\
    /// 
    /// When this error occurs, the RCODE field of the header section of the
    /// response message is set to 3.
    /// 
    /// Name Error - Meaningful only for responses from an authoritative name 
    /// server, this code signifies that the domain name referenced in the query 
    /// does not exist.
    #[error("Name Error: {0}")]
    NameError(&'static str),

    /// An error when the query is not implemented.
    /// 
    /// [RFC 1035]: https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1\
    /// 
    /// When this error occurs, the RCODE field of the header section of the
    /// response message is set to 4.
    /// 
    /// Not Implemented - The name server does not support the requested kind of
    /// query.
    #[error("Not Implemented: {0}")]
    NotImplemented(&'static str),

    /// An error when the server refused to answer.
    /// 
    /// [RFC 1035]: https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1\
    /// 
    /// When this error occurs, the RCODE field of the header section of the
    /// response message is set to 5.
    /// 
    /// Refused - The name server refuses to perform the specified operation for
    /// policy reasons.  For example, a name server may not wish to provide the
    /// information to the particular requester, or a name server may not wish to
    /// perform a particular operation (e.g., zone transfer) for particular data.
    #[error("Refused: {0}")]
    Refused(&'static str),

    /// An error when the response has an error code between 6 and 15.
    /// 
    /// When this error occurs, the RCODE field of the header section of the
    /// response message is set to a value different from the ones described
    /// in the other variants of this enum. 
    #[error("Response with error code {0}")]
    ResponseError(u8),

    /// A temporary error when the server is not available.
    /// 
    /// This error involes most of the failures that may occur from this
    /// Stub Resolver implementation.
    /// 
    /// [RFC 1034]: https://datatracker.ietf.org/doc/html/rfc1034#section-5.2.3
    /// 
    /// 5.2.3. Temporary failures
    /// 
    /// In a less than perfect world, all resolvers will occasionally be unable
    /// to resolve a particular request.  This condition can be caused by a
    /// resolver which becomes separated from the rest of the network due to a
    /// link failure or gateway problem, or less often by coincident failure or
    /// unavailability of all servers for a particular domain.
    /// 
    /// It is essential that this sort of condition should not be signalled as a
    /// name or data not present error to applications.  This sort of behavior
    /// is annoying to humans, and can wreak havoc when mail systems use the
    /// DNS.
    /// 
    /// While in some cases it is possible to deal with such a temporary problem
    /// by blocking the request indefinitely, this is usually not a good choice,
    /// particularly when the client is a server process that could move on to
    /// other tasks.  The recommended solution is to always have temporary
    /// failure as one of the possible results of a resolver function, even
    /// though this may make emulation of existing HOSTS.TXT functions more
    /// difficult.
    #[error("Temporary Error: {0}")]
    TemporaryError(&'static str),
}

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
