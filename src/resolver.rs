use crate::resource_record::ResourceRecord;
use std::collections::HashMap;
use std::vec::Vec;

#[derive(Clone)]
pub struct Resolver {
    ip_address: String,
    port: String,
    sbelt: Vec<ResourceRecord>,
    cache: DnsCache,
}

impl Resolver {
    pub fn new() -> Self {
        let resolver = Resolver{
            ip_address: String::from(""),
            port: String::from(""),
            sbelt: Vec<ResourceRecord>::new(),
            cache: DnsCache::new(),
        };
        resolver
    }
}

// Getters
impl Resolver {
    pub fn get_ip_address(&self) -> String {
        self.ip_address.clone()
    }

    pub fn get_port(&self) -> String {
        self.port.clone()
    }

    pub fn get_sbelt(&self) -> Vec<ResourceRecord> {
        self.sbelt
    }

    pub fn get_cache(&self) -> DnsCache {
        self.cache
    }
}

//Setters
impl Resolver {
    pub fn set_ip_address(&mut self, ip_address: String) {
        self.ip_address = ip_address;
    }

    pub fn set_port(&mut self, port: String) {
        self.port = port;
    }

    pub fn set_sbelt(&mut self, sbelt: Vec<ResourceRecord>) {
        self.sbelt = sbelt;
    }

    pub fn set_cache(&mut self, cache: DnsCache) {
        self.cache = cache;
    }
}
