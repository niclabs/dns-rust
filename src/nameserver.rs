pub mod DnsZone;

/// Structure to represent a Name Server
#[derive (PartialEq, Debug)]
pub struct NameServer {
    zones: HashMap<DomainName, DnsZone>, // Each zone is associated with a domain.
}

impl NameServer {
    /// Constructor to initialize an empty NameServer
    pub fn new(forwarders: Vec<String>) -> Self {
        NameServer {
            zones: HashMap::new(),
        }
    }
    /// Adds a new zone to the NameServer
    pub fn add_zone(&mut self, zone: Zone) {
        self.zones.insert(zone.domain.clone(), zone);
    }
    /// Removes a zone by its domain name
    pub fn remove_zone(&mut self, domain: &str) -> bool {
        self.zones.remove(domain).is_some()
    }
    /// Lists the domains managed by this server
    pub fn list_zones(&self) -> Vec<String> {
        self.zones.keys().cloned().collect()
    }
    /// Returns the number of managed zones
    pub fn zone_count(&self) -> usize {
        self.zones.len()
    }
}
