use crate::async_resolver::server_info::ServerInfo;

/// Struct that holds the state of a single server for a request.
#[derive(Clone)]
pub struct ServerEntry {
    info: ServerInfo,
    work_counter: u16,
}

impl ServerEntry {
    pub fn new(info: ServerInfo, work_counter: u16) -> ServerEntry {
        ServerEntry {
            info,
            work_counter: work_counter,
        }
    }

    pub fn get_info(&self) -> &ServerInfo {
        &self.info // TODO: see if this is necessary to use clone or not, in order to reuse TCP connections
    }

    pub fn get_work_counter(&self) -> u16 {
        self.work_counter
    }

    pub fn decrement_work_counter(&mut self) {
        self.work_counter -= 1;
    }
}
