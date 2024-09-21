use tokio::time::Instant;

use super::{server_entry::ServerEntry, server_info::ServerInfo};

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
    /// # Arguments
    /// * `timestamp` - A Instant that represents the time the request began.
    /// 
    /// # Example
    /// ```
    /// let state_block = StateBlock::new(Instant::now());
    /// ```
    pub fn new(request_global_limit: u16, server_transmission_limit: u16, servers: Vec<ServerInfo>) -> StateBlock {
        StateBlock {
            timestamp: Instant::now(),
            work_counter: request_global_limit,
            servers: servers.into_iter().map(|server| ServerEntry::new(server, server_transmission_limit)).collect(),
            current_server_index: 0,
        }
    }

    /// Decrements the `work_counter` of the request.
    /// 
    /// This function should be called each time the resolver performs work on behalf
    /// of the request. If the counter passes zero, the request is terminated with a 
    /// temporary error.
    /// 
    /// # Example
    /// ```
    /// let mut state_block = StateBlock::new(Instant::now());
    /// state_block.decrement_work_counter();
    /// ```
    pub fn decrement_work_counter(&mut self) {
        self.work_counter -= 1;

        // TODO: Implement the logic to terminate the request if the counter reaches zero.
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
}