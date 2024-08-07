use crate::async_resolver::server_info::ServerInfo;

/// Struct that holds the state of a single server for a request.
pub struct ServerEntry {
    info: ServerInfo,
    retransmissions: u32,
}

impl ServerEntry {
    pub fn new(info: ServerInfo) -> ServerEntry {
        ServerEntry {
            info,
            retransmissions: 0,
        }
    }

    pub fn get_info(&self) -> &ServerInfo {
        &self.info
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
}
