use crate::async_resolver::server_info::ServerInfo;

/// Struct that holds the state of a single server for a request.
#[derive(Clone)]
pub struct ServerEntry {
    /// Information about the server.
    info: ServerInfo,
    /// The work counter to limit the amount of work done on a single server.
    /// 
    /// If the counter reaches zero, the resolver must return a response to the client.
    work_counter: u16,
}

impl ServerEntry {
    /// Creates a new ServerEntry for a request.
    pub fn new(info: ServerInfo, work_counter: u16) -> ServerEntry {
        ServerEntry {
            info,
            work_counter: work_counter,
        }
    }

    /// Returns a reference to the ServerInfo of the server.
    pub fn get_info(&self) -> &ServerInfo {
        &self.info // TODO: see if this is necessary to use clone or not, in order to reuse TCP connections
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
