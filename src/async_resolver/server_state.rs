use crate::async_resolver::server_info::ServerInfo;

/// Struct that holds the state of the queried server for a single request.
/// 
/// A structure which describes the name servers which the resolver is 
/// currently trying to query. This structure keeps track of the state of a 
/// request if it must wait for answers from other name servers.
pub struct ServerState {
    name_servers: Vec<ServerInfo>,
    current_server_index: usize,
}

impl ServerState {
    /// Creates a new ServerState for a request.
    /// 
    /// # Arguments
    /// * `name_servers` - A vector of ServerInfo structs that represent the name servers to query.
    /// 
    /// # Example
    /// ```
    /// let server_state = ServerState::new(vec![ServerInfo::new("
    /// 
    /// ```
    pub fn new(name_servers: Vec<ServerInfo>) -> ServerState {
        ServerState {
            name_servers: name_servers,
            current_server_index: 0,
        }
    }

    /// Increments the `current_server_index` of the request.
    /// 
    /// It it used when the resolver must query the next name server in the list.
    pub fn increment_current_server_index(&mut self) {
        self.current_server_index = (self.current_server_index + 1)%(self.name_servers.len());
    }   

    /// Returns a refererece to the current `ServerInfo` of the request.
    pub fn get_current_server(&self) -> &ServerInfo {
        return &self.name_servers[self.current_server_index];
    }

    /// Returns a reference to the `name_servers` of the request.
    pub fn get_name_servers(&self) -> &Vec<ServerInfo> {
        return &self.name_servers;
    }
}