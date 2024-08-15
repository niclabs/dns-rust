use crate::async_resolver::server_info::ServerInfo;
use crate::async_resolver::server_entry::ServerEntry;

/// Struct that holds the state of the queried server for a single request.
/// 
/// A structure which describes the name servers which the resolver is 
/// currently trying to query. This structure keeps track of the state of a 
/// request if it must wait for answers from other name servers.
#[derive(Clone)]
pub struct ServerState {
    servers: Vec<ServerEntry>,
    current_server_index: usize,
}

impl ServerState {
    /// Creates a new ServerState for a request.
    /// 
    /// # Arguments
    /// * `servers` - A vector of ServerInfo structs that represent the name servers to query.
    /// 
    /// # Example
    /// ```
    /// let server_state = ServerState::new(vec![ServerInfo::new("
    /// 
    /// ```
    pub fn new(servers: Vec<ServerInfo>) -> ServerState {
        ServerState {
            servers: servers.into_iter().map(ServerEntry::new).collect(),
            current_server_index: 0,
        }
    }

    /// Increments the `current_server_index` of the request.
    /// 
    /// It it used when the resolver must query the next name server in the list.
    pub fn increment_current_server_index(&mut self) {
        self.current_server_index = (self.current_server_index + 1)%(self.servers.len());
    }   

    // /// Returns a refererece to the current `ServerInfo` of the request.
    // pub fn get_current_server(&self) -> &ServerInfo {
    //     return &self.servers[self.current_server_index].get_info();
    // }

    pub fn get_current_server_entry(&mut self) -> &mut ServerEntry {
        &mut self.servers[self.current_server_index]
    }

    /// Returns a reference to the `servers` of the request.
    pub fn get_servers(&self) -> &Vec<ServerEntry> {
        &self.servers
    }
}