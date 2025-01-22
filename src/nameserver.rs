use std::collections::HashMap;
use std::io;
use crate::zones::DnsZone;
use crate::message::domain_name::DomainName;


/// Structure to represent a Name Server
#[derive (PartialEq, Debug)]
pub struct NameServer {
    zones: HashMap<String, DnsZone>, // Each zone is associated with a domain.
}

impl NameServer {
    /// Constructor to initialize a NameServer with one or more zones from a vector of master files.
    ///
    /// This function reads a master file, creates a `DnsZone`, and associates it with
    /// its domain name in the `zones` HashMap.
    ///
    /// # Examples
    ///
    /// ```
    /// let masterfile_paths = vec!["masterfile1.txt", "masterfile2.txt"];
    /// let name_server = NameServer::new(masterfile_paths).unwrap();
    ///
    /// assert!(name_server.zones.contains_key("example.com."));
    /// ```
    pub fn new(masterfile_paths: Vec<&str>) -> io::Result<Self> {
        // Create a new hashmap to store the zones
        let mut zones = HashMap::new();
        // For each masterfile path
        for path in masterfile_paths {
            // Create a new zone from the masterfile
            let dns_zone = DnsZone::from_master_file(path)?;
            // Check if the zone already exists
            if zones.contains_key(&dns_zone.name) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Zona duplicada encontrada: {}", dns_zone.name),
                ));
            }
            else { // If the zone does not exist 
                // Insert the zone into the hashmap            
                zones.insert(dns_zone.name.clone(), dns_zone);
            }
        }

        Ok(NameServer { zones })
    }

    /// Searches for a zone by its domain name
    /// 
    /// # Examples
    /// 
    /// ```
    /// let name_server = NameServer::new("masterfile.txt").unwrap();
    /// let domain = DomainName::new_from_str("example.com.".to_string());
    /// let zone = name_server.search_zone(&domain);
    /// 
    /// assert!(zone.is_some());
    /// ```
    pub fn search_zone(&self, domain: &DomainName) -> Option<&DnsZone> {
        self.zones.get(&domain.to_string())
    } 

    /// Adds a new zone to the NameServer
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut name_server = NameServer::new("masterfile.txt").unwrap();
    /// let domain = DomainName::new_from_str("example.com.".to_string());
    /// let zone = DnsZone::new("example.com.", 3600, SoaRdata::new());
    /// name_server.add_zone(zone);
    /// 
    /// assert!(name_server.zones.contains_key(&domain));
    /// ```
    pub fn add_zone(&mut self, zone: Zone) {
        self.zones.insert(zone.domain.clone(), zone);
    }
    /// Removes a zone by its domain name
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut name_server = NameServer::new("masterfile.txt").unwrap();
    /// let domain = DomainName::new_from_str("example.com.".to_string());
    /// let zone = DnsZone::new("example.com.", 3600, SoaRdata::new());
    /// name_server.add_zone(zone);
    /// 
    /// let removed = name_server.remove_zone("example.com.");
    /// assert!(removed);
    /// 
    /// assert!(!name_server.get_zones().contains_key(&domain));
    /// ```
    pub fn remove_zone(&mut self, domain: &str) -> bool {
        self.zones.remove(domain).is_some()
    }
}

/// Getters
impl NameServer {
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
    fn test_search_zone(){
        let mut name_server = NameServer {
            zones: HashMap::new(),
        };

        // Create the SOA RData for the zone
        let mut soa_data = SoaRdata::new();
        soa_data.set_name_server(DomainName::new_from_str("ns1.example.com.".to_string()));
        soa_data.set_responsible_person(DomainName::new_from_str("admin.example.com.".to_string()));
        soa_data.set_serial(20240101);
        soa_data.set_refresh(3600);
        soa_data.set_retry(1800);
        soa_data.set_expire(1209600);
        soa_data.set_minimum(3600);

        // Create and add a zone
        let zone = DnsZone::new(
            "example.com.",
            3600,
            soa_data,
        );

        name_server.add_zone(zone);

        // Search for the zone by its domain name
        let domain = DomainName::new_from_str("example.com.".to_string());
        let found_zone = name_server.search_zone(&domain);

        // Validate that the zone was found
        assert!(found_zone.is_some());
    }

    #[test]
    fn test_add_zone() {
        let mut name_server = NameServer {
            zones: HashMap::new(),
        };

        // Create the SOA RData for the zone
        let mut soa_data = SoaRdata::new();
        soa_data.set_name_server(DomainName::new_from_str("ns1.example.com.".to_string()));
        soa_data.set_responsible_person(DomainName::new_from_str("admin.example.com.".to_string()));
        soa_data.set_serial(20240101);
        soa_data.set_refresh(3600);
        soa_data.set_retry(1800);
        soa_data.set_expire(1209600);
        soa_data.set_minimum(3600);

        // Create a new zone to add
        let zone = DnsZone::new(
            "example.com.",
            3600,
            soa_data,
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

        // Create the SOA RData for the zone
        let mut soa_data = SoaRdata::new();
        soa_data.set_name_server(DomainName::new_from_str("ns1.example.com.".to_string()));
        soa_data.set_responsible_person(DomainName::new_from_str("admin.example.com.".to_string()));
        soa_data.set_serial(20240101);
        soa_data.set_refresh(3600);
        soa_data.set_retry(1800);
        soa_data.set_expire(1209600);
        soa_data.set_minimum(3600);

        // Create and add a zone
        let zone = DnsZone::new(
            "example.com.",
            3600,
            soa_data,
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


        // Create the SOA RData for the zone
        let mut soa_data1 = SoaRdata::new();
        soa_data.set_name_server(DomainName::new_from_str("ns1.example.com.".to_string()));
        soa_data.set_responsible_person(DomainName::new_from_str("admin.example.com.".to_string()));
        soa_data.set_serial(20240101);
        soa_data.set_refresh(3600);
        soa_data.set_retry(1800);
        soa_data.set_expire(1209600);
        soa_data.set_minimum(3600);

        // Create and add two zones
        let zone1 = DnsZone::new(
            "example.com.",
            3600,
            soa_data1,
        );

        let mut soa_data2 = SoaRdata::new();
        soa_data.set_name_server(DomainName::new_from_str("ns1.example.org.".to_string()));
        soa_data.set_responsible_person(DomainName::new_from_str("admin.example.org.".to_string()));
        soa_data.set_serial(20240102);
        soa_data.set_refresh(3600);
        soa_data.set_retry(1800);
        soa_data.set_expire(1209600);
        soa_data.set_minimum(3600);


        let zone2 = DnsZone::new(
            "example.org.",
            3600,
            soa_data2,
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


        // Create the SOA RData for the zone
        let mut soa_data = SoaRdata::new();
        soa_data.set_name_server(DomainName::new_from_str("ns1.example.com.".to_string()));
        soa_data.set_responsible_person(DomainName::new_from_str("admin.example.com.".to_string()));
        soa_data.set_serial(20240101);
        soa_data.set_refresh(3600);
        soa_data.set_retry(1800);
        soa_data.set_expire(1209600);
        soa_data.set_minimum(3600);

        // Add a zone and validate the count
        let zone = DnsZone::new(
            "example.com.",
            3600,
            soa_data,
        );

        name_server.add_zone(zone);
        assert_eq!(name_server.get_zone_count(), 1);
    }
}

