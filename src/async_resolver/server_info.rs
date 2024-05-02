use crate::message::{resource_record::ResourceRecord, DnsMessage};
use std::fmt;


///This struscture is used to represent the information of a server.

#[derive(Debug, Clone)]
pub struct ServerInfo {
    //The IP address of the server.
    ip_addr: IpAddr,
    //The port of the server.
    port: u16,
    //The key of the server.
    key: String,
    // The algorithm of the server.
    algorithm: String,
}
