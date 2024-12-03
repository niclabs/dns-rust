use std::sync::Arc;
use tokio::time::Instant;
use super::{resolver_error::ResolverError, server_entry::ServerEntry, server_info::ServerInfo};

/// This struct represent the state of information of a pending request.
/// 
/// [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035#section-7.1)
/// 
/// Since a resolver must be able to multiplex multiple requests if it is to
/// perform its function efficiently, each pending request is usually
/// represented in some block of state information.
/// 
/// The key algorithm uses the state information of the request to select the 
/// next name server address to query.
#[derive(Clone)]
pub struct StateBlock {
    /// A timestamp indicating the time the request began.
    /// 
    /// The timestamp is used to decide whether RRs in the database
    /// can be used or are out of date. This timestamp uses the
    /// absolute time format.
    timestamp: Instant,

    /// Global per-request counter to limit work on a single request.
    /// 
    /// This counter should be initialized to the value specified in the
    /// request-global limit field of the resolver configuration. It must 
    /// be decremented each time the resolver performs work on behalf of
    /// the request. If the counter reaches zero, the resolver must
    /// return a response to the client.
    work_counter: u16,

    /// Information about the servers that are being queried.
    servers: Vec<ServerEntry>,

    /// The index of the current server being queried.
    current_server_index: usize,
}

impl StateBlock {
    /// Creates a new StateBlock for a request.
    /// 
    /// The `request_global_limit` is the global per-request counter to limit work on a single request. 
    /// This value will be used to initialize the `work_counter` of the request. 
    /// 
    /// The `server_transmission_limit` is the maximum number of simultaneous queries that can be sent
    /// to a single server. This value will be used to initialize the `work_counter` of each of the servers
    /// in `servers`. That information will be stored in the `ServerEntry` struct for each server.
    /// 
    /// The field `current_server_index` is initialized to zero.
    pub fn new(request_global_limit: u16, server_transmission_limit: u16, servers: Vec<Arc<ServerInfo>>) -> StateBlock {
        StateBlock {
            timestamp: Instant::now(),
            work_counter: request_global_limit,
            servers: servers.into_iter()
            .map(|info| ServerEntry::new(info, server_transmission_limit))
            .collect(), 
            current_server_index: 0,
        }
    }

    /// Decrements the `work_counter` of the request.
    /// 
    /// This function should be called each time the resolver performs work on behalf
    /// of the request. If the counter passes zero, the request is terminated with a 
    /// temporary error.
    pub fn decrement_work_counter(&mut self) -> Result<u16, ResolverError> {
        if self.work_counter == 0 {
            return Err(ResolverError::RetriesLimitExceeded);
        }
        self.work_counter -= 1;
        Ok(self.work_counter)
    }

    /// Increments the `current_server_index` of the request.
    /// 
    /// It it used when the resolver must query the next name server in the list.
    pub fn increment_current_server_index(&mut self) {
        self.current_server_index = (self.current_server_index + 1)%(self.servers.len());
    }

    /// Returns a reference to the `timestamp` of the request.
    pub fn get_timestamp(&self) -> &Instant {
        return &self.timestamp;
    }

    /// Returns a the `work_counter` of the request.
    pub fn get_work_counter(&self) -> u16 {
        return self.work_counter;
    }

    pub fn get_current_server_entry(&mut self) -> &mut ServerEntry {
        &mut self.servers[self.current_server_index]
    }

    pub fn get_servers(&self) -> &Vec<ServerEntry> {
        &self.servers
    }

    /// Returns the index of the current server being queried.
    pub fn get_current_server_index(&self) -> usize {
        self.current_server_index
    }
}

#[cfg(test)]
mod state_block_tests {
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
        let servers = vec![info_arc];
        let state_block = StateBlock::new(5, 2, servers);

        assert_eq!(state_block.get_work_counter(), 5);
        assert_eq!(state_block.get_servers().len(), 1);
        assert_eq!(state_block.get_current_server_index(), 0);
    }

    #[test]
    fn decrement_work_counter() {
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let port = 53;
        let key = String::from("key");
        let algorithm = String::from("algorithm");
        let udp_connection = ClientUDPConnection::new_default(ip_addr, Duration::from_secs(100));
        let tcp_connection = ClientTCPConnection::new_default(ip_addr, Duration::from_secs(100));
        let info = ServerInfo::new(ip_addr, port, key, algorithm, udp_connection, tcp_connection);

        let info_arc = Arc::new(info);
        let servers = vec![info_arc];

        let mut state_block = StateBlock::new(5, 2, servers);
        assert_eq!(state_block.get_work_counter(), 5);

        if let Ok(_) = state_block.decrement_work_counter() {
            assert_eq!(state_block.get_work_counter(), 4);
        }

        if let Ok(_) = state_block.decrement_work_counter() {
            assert_eq!(state_block.get_work_counter(), 3);
        }
    }

    #[test]
    fn decrement_work_counter_error() {
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let port = 53;
        let key = String::from("key");
        let algorithm = String::from("algorithm");
        let udp_connection = ClientUDPConnection::new_default(ip_addr, Duration::from_secs(100));
        let tcp_connection = ClientTCPConnection::new_default(ip_addr, Duration::from_secs(100));
        let info = ServerInfo::new(ip_addr, port, key, algorithm, udp_connection, tcp_connection);

        let info_arc = Arc::new(info);
        let servers = vec![info_arc];

        let mut state_block = StateBlock::new(1, 2, servers);
        assert_eq!(state_block.get_work_counter(), 1);

        if let Ok(_) = state_block.decrement_work_counter() {
            assert_eq!(state_block.get_work_counter(), 0);
        }

        if let Err(ResolverError::RetriesLimitExceeded) = state_block.decrement_work_counter() {
            assert_eq!(state_block.get_work_counter(), 0);
        }
    }

    #[test]
    fn increment_current_server_index() {
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let port = 53;
        let key = String::from("key");
        let algorithm = String::from("algorithm");
        let udp_connection = ClientUDPConnection::new_default(ip_addr, Duration::from_secs(100));
        let tcp_connection = ClientTCPConnection::new_default(ip_addr, Duration::from_secs(100));
        let info = ServerInfo::new(ip_addr, port, key, algorithm, udp_connection, tcp_connection);
        let info_clone = info.clone();

        let info_arc_1 = Arc::new(info);
        let info_arc_2 = Arc::new(info_clone);
        let servers = vec![info_arc_1, info_arc_2];

        let mut state_block = StateBlock::new(5, 2, servers);
        assert_eq!(state_block.get_current_server_index(), 0);

        state_block.increment_current_server_index();
        assert_eq!(state_block.get_current_server_index(), 1);

        state_block.increment_current_server_index();
        assert_eq!(state_block.get_current_server_index(), 0);
    }

    #[tokio::test]
    async fn get_timestamp() {
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let port = 53;
        let key = String::from("key");
        let algorithm = String::from("algorithm");
        let udp_connection = ClientUDPConnection::new_default(ip_addr, Duration::from_secs(100));
        let tcp_connection = ClientTCPConnection::new_default(ip_addr, Duration::from_secs(100));
        let info = ServerInfo::new(ip_addr, port, key, algorithm, udp_connection, tcp_connection);
        let info_clone = info.clone();

        let info_arc_1 = Arc::new(info);
        let info_arc_2 = Arc::new(info_clone);
        let servers = vec![info_arc_1, info_arc_2];

        let now = Instant::now();
        let state_block = StateBlock::new(5, 2, servers);
        assert!(state_block.get_timestamp().elapsed().as_secs() < 1);
        tokio::time::sleep(Duration::from_secs(1)).await;
        assert!(state_block.get_timestamp().elapsed().as_secs() >= 1);

        let time_difference = now - *state_block.get_timestamp();
        println!("{:?}", time_difference);
        assert!(time_difference.as_millis() < 1);
    }

    #[test]
    fn get_current_server_entry() {
        let port = 53;
        let key = String::from("key");
        let algorithm = String::from("algorithm");

        let ip_addr_1 = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let udp_connection_1 = ClientUDPConnection::new_default(ip_addr_1, Duration::from_secs(100));
        let tcp_connection_1 = ClientTCPConnection::new_default(ip_addr_1, Duration::from_secs(100));
        let info_1 = ServerInfo::new(ip_addr_1, port, key.clone(), algorithm.clone(), udp_connection_1, tcp_connection_1);

        let ip_addr_2 = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 2));
        let udp_connection_2 = ClientUDPConnection::new_default(ip_addr_2, Duration::from_secs(100));
        let tcp_connection_2 = ClientTCPConnection::new_default(ip_addr_2, Duration::from_secs(100));
        let info_2 = ServerInfo::new(ip_addr_2, port, key.clone(), algorithm.clone(), udp_connection_2, tcp_connection_2);

        let ip_addr_3 = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 3));
        let udp_connection_3 = ClientUDPConnection::new_default(ip_addr_3, Duration::from_secs(100));
        let tcp_connection_3 = ClientTCPConnection::new_default(ip_addr_3, Duration::from_secs(100));
        let info_3 = ServerInfo::new(ip_addr_3, port, key.clone(), algorithm.clone(), udp_connection_3, tcp_connection_3);

        let info_arc_1 = Arc::new(info_1);
        let info_arc_2 = Arc::new(info_2);
        let info_arc_3 = Arc::new(info_3);
        let servers = vec![info_arc_1, info_arc_2, info_arc_3];

        let mut state_block = StateBlock::new(5, 2, servers);
        assert_eq!(state_block.get_current_server_index(), 0);
        let current_server_entry = state_block.get_current_server_entry();
        assert_eq!(current_server_entry.get_work_counter(), 2);
        assert_eq!(current_server_entry.get_info().get_ip_addr(), ip_addr_1);

        if let Ok(_) = current_server_entry.decrement_work_counter() {
            assert_eq!(current_server_entry.get_work_counter(), 1);
        }
        else {
            assert!(false);
        }

        if let Ok(_) = state_block.decrement_work_counter() {
            state_block.increment_current_server_index();
            let current_server_entry: &mut ServerEntry = state_block.get_current_server_entry();
            assert_eq!(current_server_entry.get_work_counter(), 2);
            assert_eq!(current_server_entry.get_info().get_ip_addr(), ip_addr_2);
            assert_eq!(state_block.get_work_counter(), 4);
        }
        else {
            assert!(false);
        }

        if let Ok(_) = state_block.decrement_work_counter() {
            state_block.increment_current_server_index();
            let current_server_entry: &mut ServerEntry = state_block.get_current_server_entry();
            assert_eq!(current_server_entry.get_work_counter(), 2);
            assert_eq!(current_server_entry.get_info().get_ip_addr(), ip_addr_3);
            assert_eq!(state_block.get_work_counter(), 3);
        }
        else {
            assert!(false);
        }

        if let Ok(_) = state_block.decrement_work_counter() {
            state_block.increment_current_server_index();
            let current_server_entry: &mut ServerEntry = state_block.get_current_server_entry();
            assert_eq!(current_server_entry.get_work_counter(), 1);
            assert_eq!(current_server_entry.get_info().get_ip_addr(), ip_addr_1);
            assert_eq!(state_block.get_work_counter(), 2);
        }
        else {
            assert!(false);
        }
    }
}