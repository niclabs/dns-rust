use crate::async_resolver::server_info::ServerInfo;

/// Struct that holds the state of a single server for a request.
#[derive(Clone)]
pub struct ServerEntry {
    info: ServerInfo,
    retransmissions: u32,
    is_active: bool,
}

impl ServerEntry {
    pub fn new(info: ServerInfo) -> ServerEntry {
        ServerEntry {
            info,
            retransmissions: 0,
            is_active: true,
        }
    }

    pub fn get_info(&self) -> &ServerInfo {
        &self.info// TODO: see if this is necessary to use clone or not, in order to reuse TCP connections
    }

    pub fn get_retransmissions(&self) -> u32 {
        self.retransmissions
    }

    pub fn increment_retransmissions(&mut self) {
        self.retransmissions += 1;
    }

    pub fn reset_retransmissions(&mut self) {
        self.retransmissions = 0;
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn set_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }
}
