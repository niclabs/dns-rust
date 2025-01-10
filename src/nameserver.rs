pub mod DnsZone;

/// Structure to represent a Name Server
#[derive (PartialEq, Debug)]
pub struct NameServer {
    zones: HashMap<DomainName, DnsZone>, // Each zone is associated with a domain.
}

impl NameServer {
    /// Constructor to initialize a NameServer with a single zone from a master file.
    ///
    /// This function reads a master file, creates a `DnsZone`, and associates it with
    /// its domain name in the `zones` HashMap.
    ///
    /// # Examples
    ///
    /// ```
    /// let name_server = NameServer::new("masterfile.txt").unwrap();
    ///
    /// assert!(name_server.zones.contains_key(&DomainName::new_from_str("example.com.".to_string())));
    /// ```
    pub fn new(masterfile_path: &str) -> io::Result<Self> {
        // Leer la zona del archivo masterfile
        let dns_zone = DnsZone::from_master_file(masterfile_path)?;

        // Asociar la zona con su nombre en el HashMap
        let mut zones = HashMap::new();
        zones.insert(DomainName::new_from_str(dns_zone.name.clone()), dns_zone);

        Ok(NameServer { zones })
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
    pub fn get_list_zones(&self) -> Vec<String> {
        self.zones.keys().cloned().collect()
    }
    /// Returns the number of managed zones
    pub fn get_zone_count(&self) -> usize {
        self.zones.len()
    }
}
