use crate::message::DnsMessage;
use std::net::IpAddr;
use tokio::time::Duration;
use super::client_error::ClientError;

use async_trait::async_trait;
use crate::client::client_connection::ClientConnection;
use crate::client::tls_connection::ClientTLSConnection;

#[async_trait]
pub trait ClientSecurity: ClientConnection + Copy {//: 'static + Sized + Send + Sync + Unpin
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionProtocol {
    DTLS,
    TLS,
    UNKNOWN,
}

impl From<&str> for ConnectionProtocol {
    /// Function to connection type base on a str
    // from_str_to_connection_type
    fn from(conn: &str) -> ConnectionProtocol {
        match conn {
            "DTLS" => ConnectionProtocol::DTLS,
            "TLS" => ConnectionProtocol::TLS,
            _ => ConnectionProtocol::UNKNOWN,
        }
    } 
}