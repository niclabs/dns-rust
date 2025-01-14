use std::collections::HashMap;
use std::io;
use crate::zones::DnsZone;
use crate::message::domain_name::DomainName;
pub mod zones;
mod server_connection;

/// Structure to represent a Name Server
#[derive (PartialEq, Debug)]
pub struct NameServer {
    zones: HashMap<String, DnsZone>, // Each zone is associated with a domain.
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

#[cfg(test)]
mod test_name_server {
    use crate::zones::DnsZone;
    use crate::message::domain_name::DomainName;
    use std::path::Path;
    use crate::message::resource_record::ResourceRecord;

    #[test]
    fn test_new() {
        // Masterfile path to initialize the NameServer
        let masterfile_path = "1034-scenario-6.1-edu.txt";

        // Verify that the file exists before continuing
        assert!(Path::new(masterfile_path).exists(), "Masterfile not found.");

        // Create a NameServer from the masterfile
        let name_server = NameServer::new(masterfile_path).unwrap();

        // Validate that the zone was added correctly
        let domain_name = DomainName::new_from_str("EDU.".to_string());
        assert!(name_server.zones.contains_key(&domain_name));
    }

    #[test]
    fn test_add_zone() {
        let mut name_server = NameServer {
            zones: HashMap::new(),
        };

        // Create a new zone to add
        let zone = DnsZone::new(
            "example.com.",
            3600,
            SoaRdata {
                mname: DomainName::new_from_str("ns1.example.com.".to_string()),
                rname: DomainName::new_from_str("admin.example.com.".to_string()),
                serial: 20240101,
                refresh: 3600,
                retry: 1800,
                expire: 1209600,
                minimum: 3600,
            },
        );

        // Add the zone to the server
        name_server.add_zone(zone.clone());

        // Validate that the zone was added correctly
        assert!(name_server.zones.contains_key(&DomainName::new_from_str("example.com.".to_string())));
    }

    #[test]
    fn test_remove_zone() {
        let mut name_server = NameServer {
            zones: HashMap::new(),
        };

        // Create and add a zone
        let zone = DnsZone::new(
            "example.com.",
            3600,
            SoaRdata {
                mname: DomainName::new_from_str("ns1.example.com.".to_string()),
                rname: DomainName::new_from_str("admin.example.com.".to_string()),
                serial: 20240101,
                refresh: 3600,
                retry: 1800,
                expire: 1209600,
                minimum: 3600,
            },
        );

        name_server.add_zone(zone);

        // Remove the zone by its domain name
        let removed = name_server.remove_zone("example.com.");
        assert!(removed);

        // Verify that the zone was removed
        assert!(!name_server.zones.contains_key(&DomainName::new_from_str("example.com.".to_string())));
    }

    #[test]
    fn test_get_list_zones() {
        let mut name_server = NameServer {
            zones: HashMap::new(),
        };

        // Create and add two zones
        let zone1 = DnsZone::new(
            "example.com.",
            3600,
            SoaRdata {
                mname: DomainName::new_from_str("ns1.example.com.".to_string()),
                rname: DomainName::new_from_str("admin.example.com.".to_string()),
                serial: 20240101,
                refresh: 3600,
                retry: 1800,
                expire: 1209600,
                minimum: 3600,
            },
        );

        let zone2 = DnsZone::new(
            "example.org.",
            3600,
            SoaRdata {
                mname: DomainName::new_from_str("ns1.example.org.".to_string()),
                rname: DomainName::new_from_str("admin.example.org.".to_string()),
                serial: 20240102,
                refresh: 3600,
                retry: 1800,
                expire: 1209600,
                minimum: 3600,
            },
        );

        name_server.add_zone(zone1);
        name_server.add_zone(zone2);

        // Get the list of zones
        let zone_list = name_server.get_list_zones();

        // Validate that it contains both zones
        assert!(zone_list.contains(&"example.com.".to_string()));
        assert!(zone_list.contains(&"example.org.".to_string()));
        assert_eq!(zone_list.len(), 2);
    }

    #[test]
    fn test_get_zone_count() {
        let mut name_server = NameServer {
            zones: HashMap::new(),
        };

        // Validate that initially there are no zones
        assert_eq!(name_server.get_zone_count(), 0);

        // Add a zone and validate the count
        let zone = DnsZone::new(
            "example.com.",
            3600,
            SoaRdata {
                mname: DomainName::new_from_str("ns1.example.com.".to_string()),
                rname: DomainName::new_from_str("admin.example.com.".to_string()),
                serial: 20240101,
                refresh: 3600,
                retry: 1800,
                expire: 1209600,
                minimum: 3600,
            },
        );

        name_server.add_zone(zone);
        assert_eq!(name_server.get_zone_count(), 1);
    }
}

