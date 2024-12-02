use std::sync::Arc;

use crate::async_resolver::server_info::ServerInfo;

/// Struct that holds the state of a single server for a request.
#[derive(Clone)]
pub struct ServerEntry {
    /// Information about the server.
    info: Arc<ServerInfo>,
    /// The work counter to limit the amount of work done on a single server.
    /// 
    /// If the counter reaches zero, the resolver must return a response to the client.
    work_counter: u16,
}

impl ServerEntry {
    /// Creates a new ServerEntry for a request.
    /// 
    /// A reference to a previously created ServerInfo is required to create a ServerEntry. 
    /// The work counter is used to limit the amount of work done on a single server.
    pub fn new(info: Arc<ServerInfo>, work_counter: u16) -> ServerEntry {
        ServerEntry {
            info: info,
            work_counter: work_counter,
        }
    }

    /// Returns an atomic reference to the ServerInfo of the server.
    /// 
    /// A new  instance of Arc<ServerInfo> is returned, referencing the same ServerInfo 
    /// as the ServerEntry.
    pub fn get_info(&self) -> Arc<ServerInfo> {
        self.info.clone()
    }

    /// Returns the work counter of the server.
    pub fn get_work_counter(&self) -> u16 {
        self.work_counter
    }

    /// Decrements the work counter of the server.
    pub fn decrement_work_counter(&mut self) {
        self.work_counter -= 1;
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};
    use tokio::time::Duration;
    use crate::client::{client_connection::ClientConnection, tcp_connection::ClientTCPConnection, udp_connection::ClientUDPConnection};

    use super::*;

    #[test]
    fn constructor() {
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let port = 53;
        let key = String::from("key");
        let algorithm = String::from("algorithm");
        let udp_connection = ClientUDPConnection::new_default(ip_addr, Duration::from_secs(100));
        let tcp_connection = ClientTCPConnection::new_default(ip_addr, Duration::from_secs(100));
        let info = ServerInfo::new(ip_addr, port, key, algorithm, udp_connection, tcp_connection);

        let info_arc = Arc::new(info);
        let work_counter = 2;
        let server_entry = ServerEntry::new(info_arc.clone(), work_counter);

        assert_eq!(server_entry.get_work_counter(), 2);
        assert_eq!(server_entry.get_info(), info_arc);
    }

}