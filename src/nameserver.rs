use std::collections::HashMap;
use std::io;
use crate::zones::DnsZone;
use crate::domain_name::DomainName;


/// Structure to represent a Name Server
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
            if zones.contains_key(dns_zone.get_name()) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Zona duplicada encontrada: {}", dns_zone.get_name()),
                ));
            }
            else { // If the zone does not exist 
                // Insert the zone into the hashmap            
                zones.insert(dns_zone.get_name().clone(), dns_zone);
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
    /// let domain = DomainName::new_from_str("example.com.".);
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
    /// let domain = DomainName::new_from_str("example.com.".);
    /// let zone = DnsZone::new("example.com.", 3600, SoaRdata::new());
    /// name_server.add_zone(zone);
    /// 
    /// assert!(name_server.zones.contains_key(&domain));
    /// ```
    pub fn add_zone(&mut self, zone: DnsZone) {
        self.zones.insert(zone.get_name().clone(), zone);
    }
    /// Removes a zone by its domain name
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut name_server = NameServer::new("masterfile.txt").unwrap();
    /// let domain = DomainName::new_from_str("example.com.".);
    /// let zone = DnsZone::new("example.com.", 3600, SoaRdata::new());
    /// name_server.add_zone(zone);
    /// 
    /// let removed = name_server.remove_zone("example.com.");
    /// assert!(removed);
    /// 
    /// assert!(!name_server.get_list_domains().contains_key(&domain));
    /// ```
    pub fn remove_zone(&mut self, domain: &str) -> bool {
        self.zones.remove(domain).is_some()
    }
}

/// Getters
impl NameServer {
    /// Lists the domains managed by this server
    pub fn get_list_domains(&self) -> Vec<String> {
        self.zones.keys().cloned().collect()
    }
    /// Returns the number of managed zones
    pub fn get_zone_count(&self) -> usize {
        self.zones.len()
    }
}

#[cfg(test)]
mod test_name_server {
    use crate::message::rclass::Rclass;
    use crate::message::rdata::soa_rdata::SoaRdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;
    use crate::zones::DnsZone;
    use crate::domain_name::DomainName;
    use crate::nameserver::NameServer;
    use std::path::Path;
    use std::collections::HashMap;

    #[test]
    fn test_new_one_zone() {
        // Masterfile path to initialize the NameServer
        let masterfile_path1 = "1034-scenario-6.1-edu.txt";

        // Verify that the file exists before continuing
        assert!(Path::new(masterfile_path1).exists(), "Masterfile not found.");

        // Create a NameServer from the masterfile
        let name_server = NameServer::new(vec![masterfile_path1]).unwrap();

        // Validate that the zone was added correctly
        let domain_name = DomainName::new_from_str("EDU.");
        assert!(name_server.get_list_domains().contains(&domain_name.get_name()));
    }

    #[test]
    fn test_new_two_zone(){
        // Masterfile path to initialize the NameServer
        let masterfile_path1 = "1034-scenario-6.1-root.txt";
        // Verify that the file exists before continuing
        assert!(Path::new(masterfile_path1).exists(), "Masterfile not found.");
        // Master file for the root zone.
        let masterfile_path2 = "1034-scenario-6.1-edu.txt";
        // Verify that the file exists before continuing
        assert!(Path::new(masterfile_path2).exists(), "Masterfile not found.");

        // Create a NameServer from the masterfile
        let name_server = NameServer::new(vec![masterfile_path1,masterfile_path2]).unwrap();

        assert!(name_server.get_list_domains().contains(&DomainName::new_from_str("ROOT.").get_name()));
        assert!(name_server.get_list_domains().contains(&DomainName::new_from_str("EDU.").get_name())); 
    }

    #[test]
    fn test_search_zone(){
        let mut name_server = NameServer {
            zones: HashMap::new(),
        };

        // Create the SOA RData for the zone
        let mut soa_data = SoaRdata::new();
        soa_data.set_mname(DomainName::new_from_str("ns1.example.com."));
        soa_data.set_rname(DomainName::new_from_str("admin.example.com."));
        soa_data.set_serial(20240101);
        soa_data.set_refresh(3600);
        soa_data.set_retry(1800);
        soa_data.set_expire(1209600);
        soa_data.set_minimum(3600);

        // Create and add a zone
        let zone = DnsZone::new(
            "example.com.",
            Rclass::IN,
            3600,
            soa_data,
        );

        name_server.add_zone(zone);

        // Search for the zone by its domain name
        let domain = DomainName::new_from_str("example.com.");
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
        soa_data.set_mname(DomainName::new_from_str("ns1.example.com."));
        soa_data.set_rname(DomainName::new_from_str("admin.example.com."));
        soa_data.set_serial(20240101);
        soa_data.set_refresh(3600);
        soa_data.set_retry(1800);
        soa_data.set_expire(1209600);
        soa_data.set_minimum(3600);

        // Create a new zone to add
        let zone = DnsZone::new(
            "example.com.",
            Rclass::IN,
            3600,
            soa_data,
        );

        // Add the zone to the server
        name_server.add_zone(zone);

        // Validate that the zone was added correctly
        assert!(name_server.get_list_domains().contains(&DomainName::new_from_str("example.com.").get_name()));
    }

    #[test]
    fn test_remove_zone() {
        let mut name_server = NameServer {
            zones: HashMap::new(),
        };

        // Create the SOA RData for the zone
        let mut soa_data = SoaRdata::new();
        soa_data.set_mname(DomainName::new_from_str("ns1.example.com."));
        soa_data.set_rname(DomainName::new_from_str("admin.example.com."));
        soa_data.set_serial(20240101);
        soa_data.set_refresh(3600);
        soa_data.set_retry(1800);
        soa_data.set_expire(1209600);
        soa_data.set_minimum(3600);

        // Create and add a zone
        let zone = DnsZone::new(
            "example.com.",
            Rclass::IN,
            3600,
            soa_data,
        );

        name_server.add_zone(zone);

        // Remove the zone by its domain name
        let removed = name_server.remove_zone("example.com.");
        assert!(removed);

        // Verify that the zone was removed
        assert!(name_server.get_list_domains().contains(&DomainName::new_from_str("example.com.").get_name()));
    }

    #[test]
    fn test_get_list_domains() {
        let mut name_server = NameServer {
            zones: HashMap::new(),
        };


        // Create the SOA RData for the zone
        let mut soa_data1 = SoaRdata::new();
        soa_data1.set_mname(DomainName::new_from_str("ns1.example.com."));
        soa_data1.set_rname(DomainName::new_from_str("admin.example.com."));
        soa_data1.set_serial(20240101);
        soa_data1.set_refresh(3600);
        soa_data1.set_retry(1800);
        soa_data1.set_expire(1209600);
        soa_data1.set_minimum(3600);

        // Create and add two zones
        let zone1 = DnsZone::new(
            "example.com.",
            Rclass::IN,
            3600,
            soa_data1,
        );

        let mut soa_data2 = SoaRdata::new();
        soa_data2.set_mname(DomainName::new_from_str("ns1.example.org."));
        soa_data2.set_rname(DomainName::new_from_str("admin.example.org."));
        soa_data2.set_serial(20240102);
        soa_data2.set_refresh(3600);
        soa_data2.set_retry(1800);
        soa_data2.set_expire(1209600);
        soa_data2.set_minimum(3600);


        let zone2 = DnsZone::new(
            "example.org.",
            Rclass::IN,
            3600,
            soa_data2,
        );

        name_server.add_zone(zone1);
        name_server.add_zone(zone2);

        // Get the list of zones
        let zone_list = name_server.get_list_domains();

        // Validate that it contains both zones
        assert!(zone_list.contains(&DomainName::new_from_str("example.com.").get_name()));
        assert!(zone_list.contains(&DomainName::new_from_str("example.org.").get_name()));
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
        soa_data.set_mname(DomainName::new_from_str("ns1.example.com."));
        soa_data.set_rname(DomainName::new_from_str("admin.example.com."));
        soa_data.set_serial(20240101);
        soa_data.set_refresh(3600);
        soa_data.set_retry(1800);
        soa_data.set_expire(1209600);
        soa_data.set_minimum(3600);

        // Add a zone and validate the count
        let zone = DnsZone::new(
            "example.com.",
            Rclass::IN,
            3600,
            soa_data,
        );

        name_server.add_zone(zone);
        assert_eq!(name_server.get_zone_count(), 1);
    }
}

