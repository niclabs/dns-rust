use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::message::rdata::a_rdata::ARdata;
use crate::message::rdata::cname_rdata::CnameRdata;
use crate::message::rdata::soa_rdata::SoaRdata;
use crate::message::rdata::aaaa_rdata::AAAARdata;
use crate::message::rdata::Rdata;
use crate::message::rdata::txt_rdata::TxtRdata;
use crate::message::rdata::mx_rdata::MxRdata;
use crate::domain_name::DomainName;
use crate::message::rdata::ptr_rdata::PtrRdata;
use crate::message::resource_record::ResourceRecord;
use crate::message::rclass::Rclass;
/*
The following entries are defined:
    <blank>[<comment>]

    $ORIGIN <domain-name> [<comment>]

    $INCLUDE <file-name> [<domain-name>] [<comment>]

    <domain-name><rr> [<comment>]

    <blank><rr> [<comment>]

<rr> contents take one of the following forms:

    [<TTL>] [<class>] <type> <RDATA>

    [<class>] [<TTL>] <type> <RDATA>
*/

/// Structure to represent a DNS zone
#[derive(Debug)]
pub struct DnsZone {
    name: String,           // Name of the zone (e.g., "example.com")
    class: Rclass,          // Class of the zone (e.g., "IN")
    ttl: u32,               // Default time to live (in seconds)
    soa: SoaRdata,         // SOA (Start of Authority) record
    ns_records: Vec<String>,// List of name servers (NS)
    resource_records: Vec<ResourceRecord>,// List of resource records
    //children: Vec<DnsZone>, // List of child zones
}

impl DnsZone {
    /// Creates a new DNS zone.
    ///
    /// # Examples
    ///
    /// ```
    /// let soa = SoaRdata {
    ///     mname: DomainName::new_from_str("ns1.example.com."),
    ///     rname: DomainName::new_from_str("admin.example.com."),
    ///     serial: 20240101,
    ///     refresh: 3600,
    ///     retry: 1800,
    ///     expire: 1209600,
    ///     minimum: 3600,
    /// };
    ///
    /// let dns_zone = DnsZone::new("example.com.", 3600, soa);
    ///
    /// assert_eq!(dns_zone.name, "example.com.");
    /// assert_eq!(dns_zone.ttl, 3600);
    /// assert_eq!(dns_zone.soa.serial, 20240101);
    /// assert!(dns_zone.ns_records.is_empty());
    /// assert!(dns_zone.resource_records.is_empty());
    /// ```
    pub fn new(name: &str,class: Rclass, ttl: u32, soa: SoaRdata) -> Self {
        DnsZone {
            name: name.to_string(),
            class,
            ttl,
            soa,
            ns_records: Vec::new(),
            resource_records: Vec::new(),
        }
    }

    /// Adds an NS (Name Server) record to the DNS zone.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut dns_zone = DnsZone::new(
    ///     "example.com.",
    ///     3600,
    ///     SoaRdata {
    ///         mname: DomainName::new_from_str("ns1.example.com."),
    ///         rname: DomainName::new_from_str("admin.example.com."),
    ///         serial: 20240101,
    ///         refresh: 3600,
    ///         retry: 1800,
    ///         expire: 1209600,
    ///         minimum: 3600,
    ///     },
    /// );
    ///
    /// dns_zone.add_ns_record("ns2.example.com.");
    ///
    /// assert_eq!(dns_zone.ns_records.len(), 1);
    /// assert_eq!(dns_zone.ns_records[0], "ns2.example.com.");
    /// ```
    pub fn add_ns_record(&mut self, ns: &str) {
        self.ns_records.push(ns.to_string());
    }

    /// Adds a generic resource record to the DNS zone.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut dns_zone = DnsZone::new(
    ///     "example.com.",
    ///     3600,
    ///     SoaRdata {
    ///         mname: DomainName::new_from_str("ns1.example.com."),
    ///         rname: DomainName::new_from_str("admin.example.com."),
    ///         serial: 20240101,
    ///         refresh: 3600,
    ///         retry: 1800,
    ///         expire: 1209600,
    ///         minimum: 3600,
    ///     },
    /// );
    ///
    /// let txt_rdata = Rdata::TXT(TxtRdata::new(String::from("dcc")));
    /// let resource_record = ResourceRecord::new(txt_rdata);
    ///
    /// dns_zone.add_resource_record(resource_record);
    ///
    /// assert_eq!(dns_zone.resource_records.len(), 1);
    /// assert_eq!(
    ///     dns_zone.resource_records[0].rdata.unwrap().get_text(),
    ///     String::from("dcc")
    /// );
    /// ```
    pub fn add_resource_record(&mut self, record: ResourceRecord) {
        self.resource_records.push(record);
    }

    /// Creates a `DnsZone` from a master file.
    ///
    /// This function parses a master file, extracts the SOA record, NS records,
    /// and other resource records, and returns the resulting `DnsZone`.
    ///
    /// # Examples
    ///
    /// ```
    /// let dns_zone = DnsZone::from_master_file("masterfile.txt").unwrap();
    ///
    /// assert_eq!(dns_zone.name, "example.com.");
    /// assert!(dns_zone.soa.serial > 0);
    /// assert!(!dns_zone.ns_records.is_empty());
    /// assert!(!dns_zone.resource_records.is_empty());
    /// ```
    /// Disclaimer!!
    /// The SOA must be all in only one line
    /// Example:
    /// EDU.   IN       SOA     SRI-NIC.ARPA.      HOSTMASTER.SRI-NIC.ARPA. ( 870729 1800 300 604800 86400 )
    /// .   IN        SOA     SRI-NIC.ARPA.      HOSTMASTER.SRI-NIC.ARPA. ( 870611 1800 300 604800 86400 )
    pub fn from_master_file(file_path: &str) -> io::Result<Self> { // Result<Self, io::Error> is the same as io::Result<Self>
        // Open the file
        let path = Path::new(file_path);
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        // Variables for the zone
        let mut name = String::new();
        let mut ttl = 3600; // Default value of ttl general
        let mut soa: SoaRdata = SoaRdata::new();
        let mut ns_records = Vec::new();
        let mut resource_records = Vec::new();
        //let class = "IN"; // Default value of class general, for the moment only IN is supported

        // Variables to work with the file
        let mut last_name = String::new();
        let mut last_ttl = 3600;
        let mut file_class = String::new(); // Default value of class general
        let mut first_line = true;
        //let mut count_ns = 0;

        // Read the file line by line
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            // Ignore empty lines and comments
            if line.is_empty() || line.starts_with(';') {
                continue;
            }
            // Process directives
            if line.starts_with("$ORIGIN") {
                name = line.split_whitespace().nth(1).unwrap_or("").to_string();
                continue;
            }
            if line.starts_with("$TTL") {
                ttl = line.split_whitespace().nth(1).unwrap_or("3600").parse().unwrap_or(3600);
                continue;
            }
            // $INCLUDE directive is not supported
            
            // Process records
            let mut parts: Vec<&str> = line.split_whitespace().collect();

            // Remove comments from the line
            if let Some(index) = parts.iter().position(|&x| x == ";") {
                parts.truncate(index);
            }

            // Assume that the first line is the SOA record
            if first_line {
                parts.retain(|x| x != &"(" && x != &")");
                let record_name = parts[0]; // The first part is the name of the record
                let record_name_string = record_name.to_string(); // Convert to String
                name = record_name_string; // Save the name of the zone
                last_name = name.clone(); // Save the last name for the next iteration

                let class = parts[1]; // The second part is the class of the record
                let class_string = class.to_string(); // Convert to String
                file_class = class_string; // Save the class of the zone
                
                // Set the SOA record
                soa.set_mname(DomainName::new_from_str(parts[3])); // The third part is the mname
                soa.set_rname(DomainName::new_from_str(parts[4])); // The fourth part is the rname
                soa.set_serial(parts[5].parse().unwrap_or(0));
                soa.set_refresh(parts[6].parse().unwrap_or(3600));
                soa.set_retry(parts.get(7).unwrap_or(&"1800").parse().unwrap_or(1800));
                soa.set_expire(parts.get(8).unwrap_or(&"1209600").parse().unwrap_or(1209600));
                soa.set_minimum(parts.get(9).unwrap_or(&"3600").parse().unwrap_or(3600));
                ttl = parts[9].parse().unwrap_or(3600); // Save the TTL for the next iteration
                // Change the flag to false
                first_line = false;
                continue;
            }

            // Check if the line has at least 4 parts
            if parts.len() >= 4 {
                let record_name = parts[0]; // The first part is the name of the record
                let record_name_string = record_name.to_string(); // Convert to String
                last_name = record_name_string; // Save the last name for the next iteration

                let record_ttl_or_type = parts[1]; // The second part is the TTL or the record type

                let mut record_ttl = ttl; // Default value of ttl for the record

                let rr_type_index = if record_ttl_or_type.parse::<u32>().is_ok() { // Check if the second part is a TTL
                    record_ttl = record_ttl_or_type.parse().unwrap_or(3600); // If it is a TTL, save it
                    2 // The index of the record type is 3
                } else { // If it is not a TTL, it is the record type
                    1 // The index of the record type is 2
                };

                last_ttl = record_ttl; // Save the last ttl for the next iteration

                let record_type = parts[rr_type_index]; // The third part is the record type
                match record_type {
                    "NS" => { // If the record type is NS
                        ns_records.push(parts[rr_type_index+1].to_string()); // Save the NS record
                    }
                    "A" => { // If the record type is A
                        // Create the A record
                        let resource_record = ARdata::rr_from_master_file(parts[rr_type_index+1].split_whitespace(), ttl, "IN", record_name.to_string());
                        resource_records.push(resource_record); // Save the A record
                    }
                    "AAAA" | "CNAME" | "MX" | "TXT" | "PTR" => { // If the record type is AAAA, CNAME, MX, TXT or PTR
                        let rdata = match record_type { // Create the Rdata
                            "AAAA" => { // If the record type is AAAA
                                let ip_addr: std::net::IpAddr = parts[rr_type_index+1].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid IP address"))?;
                                Rdata::AAAA(AAAARdata::new_from_addr(ip_addr))
                            },
                            "CNAME" => { // If the record type is CNAME
                                let cname = DomainName::new_from_str(parts[rr_type_index+1]); // Create the DomainName
                                let mut cname_rdata = CnameRdata::new(); // Create the CnameRdata
                                cname_rdata.set_cname(cname); // Set the cname
                                let rdata = Rdata::CNAME(cname_rdata); // Create the Rdata
                                rdata}, // CNAME
                            "MX" => Rdata::MX(MxRdata::new()), // If the record type is MX
                            "TXT" => Rdata::TXT(TxtRdata::new(vec![parts[rr_type_index+1].to_string()])), // If the record type is TXT
                            "PTR" => Rdata::PTR(PtrRdata::new()), // If the record type is PTR
                            _ => continue,
                        };

                        let mut resource_record = ResourceRecord::new(rdata);

                        resource_record.set_name(DomainName::new_from_str(record_name));
                        resource_record.set_ttl(last_ttl);

                        resource_records.push(resource_record);
                    }
                    _ => {
                        continue;
                    } // Here is where ZONEMD and other unknow types should be entered.
                }
            }  
            else if parts.len() < 4 {
                //print!("{:?}", parts);

                let record_ttl_or_type = parts[0];

                let mut record_ttl = ttl; // Default value of ttl for the record

                let rr_type_index = if record_ttl_or_type.parse::<u32>().is_ok() { // Check if the second part is a TTL
                    record_ttl = record_ttl_or_type.parse().unwrap_or(3600); // If it is a TTL, save it
                    1 // The index of the record type is 2
                } else { // If it is not a TTL, it is the record type
                    0 // The index of the record type is 1
                };


                let record_type = parts[rr_type_index];
                let record_data = parts[rr_type_index+1];

                match record_type {
                    "NS" => {
                        ns_records.push(record_data.to_string());
                    }
                    "A" => {
                        let resource_record = ARdata::rr_from_master_file(record_data.split_whitespace(), ttl, "IN", last_name.to_string());
                        resource_records.push(resource_record);
                    }
                    "AAAA" | "CNAME" | "MX" | "TXT" | "PTR" => {
                        let rdata = match record_type {
                            "AAAA" => {
                                let ip_addr: std::net::IpAddr = record_data.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid IP address"))?;
                                Rdata::AAAA(AAAARdata::new_from_addr(ip_addr))
                            },
                            "CNAME" => {
                                let cname = DomainName::new_from_str(record_data);
                                let mut cname_rdata = CnameRdata::new();
                                cname_rdata.set_cname(cname);
                                let rdata = Rdata::CNAME(cname_rdata);
                                rdata}, // CNAME
                            "MX" => Rdata::MX(MxRdata::new()),
                            "TXT" => Rdata::TXT(TxtRdata::new(vec![record_data.to_string()])),
                            "PTR" => Rdata::PTR(PtrRdata::new()),
                            _ => continue,
                        };

                        let mut resource_record = ResourceRecord::new(rdata);

                        resource_record.set_name(DomainName::new_from_str(last_name.as_str()));
                        resource_record.set_ttl(record_ttl);

                        resource_records.push(resource_record);
                    }
                    _ => {
                        continue;
                    }
                }
            }            
        }

        let class = Rclass::from(file_class.as_str()); // Convert the class to Rclass  

        // Validate and construct the zone  
        Ok(DnsZone {
            name,
            class,
            ttl,
            soa,
            ns_records,
            resource_records,
            })
    }   
}

/// Getters
impl DnsZone {
    /// Gets the name of the zone.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Gets the class of the zone.
    pub fn get_class(&self) -> Rclass {
        self.class
    }

    /// Gets the default TTL of the zone.
    pub fn get_ttl(&self) -> u32 {
        self.ttl
    }

    /// Gets the SOA (Start of Authority) record of the zone.
    pub fn get_soa(&self) -> &SoaRdata {
        &self.soa
    }

    /// Gets the list of name servers (NS) for the zone.
    pub fn get_ns_records(&self) -> &Vec<String> {
        &self.ns_records
    }

    /// Gets the list of resource records for the zone.
    pub fn get_resource_records(&self) -> &Vec<ResourceRecord> {
        &self.resource_records
    }
}


#[cfg(test)]
mod dns_zone_tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_new() {

        let mut soa = SoaRdata::new();
        soa.set_mname(DomainName::new_from_str("ns1.example.com."));
        soa.set_rname(DomainName::new_from_str("admin.example.com."));
        soa.set_serial(20240101);
        soa.set_refresh(3600);
        soa.set_retry(1800);
        soa.set_expire(1209600);
        soa.set_minimum(3600);

        let dns_zone = DnsZone::new("example.com.", Rclass::IN, 3600, soa);

        assert_eq!(dns_zone.name, "example.com.");
        assert_eq!(dns_zone.ttl, 3600);
        assert_eq!(dns_zone.soa.get_mname().get_name(), "ns1.example.com.");
        assert_eq!(dns_zone.soa.get_serial(), 20240101);
        assert!(dns_zone.get_ns_records().is_empty());
        assert!(dns_zone.get_resource_records().is_empty());
    }

    #[test]
    fn test_add_ns_record() {

        let mut soa_data = SoaRdata::new();
        soa_data.set_mname(DomainName::new_from_str("ns1.example.com."));
        soa_data.set_rname(DomainName::new_from_str("admin.example.com."));
        soa_data.set_serial(20240101);
        soa_data.set_refresh(3600);
        soa_data.set_retry(1800);
        soa_data.set_expire(1209600);
        soa_data.set_minimum(3600);

        let mut dns_zone = DnsZone::new(
            "example.com.",
            Rclass::IN,
            3600,
            soa_data,
        );

        dns_zone.add_ns_record("ns2.example.com.");

        assert_eq!(dns_zone.get_ns_records().len(), 1);
        assert_eq!(dns_zone.get_ns_records()[0], "ns2.example.com.");
    }

    #[test]
    fn test_add_resource_record() {
        let mut soa_data = SoaRdata::new();
        soa_data.set_mname(DomainName::new_from_str("ns1.example.com."));
        soa_data.set_rname(DomainName::new_from_str("admin.example.com."));
        soa_data.set_serial(20240101);
        soa_data.set_refresh(3600);
        soa_data.set_retry(1800);
        soa_data.set_expire(1209600);
        soa_data.set_minimum(3600);

        let mut dns_zone = DnsZone::new(
            "example.com.",
            Rclass::IN,
            3600,
            soa_data,
        );

        let txt_rdata = Rdata::TXT(TxtRdata::new(vec![String::from("dcc")]));
        let resource_record = ResourceRecord::new(txt_rdata);

        dns_zone.add_resource_record(resource_record);

        assert_eq!(dns_zone.get_resource_records().len(), 1);
        assert_eq!(dns_zone.get_resource_records()[0].get_rdata(), Rdata::TXT(TxtRdata::new(vec![String::from("dcc")])));
    }


    /// Disclaimer!!
    /// The SOA must be all in only one line
    /// The file used in the next text does not have the SOA in one line
    /// Change the file to test the function
    /// New SOA:
    /// EDU.   IN       SOA     SRI-NIC.ARPA.      HOSTMASTER.SRI-NIC.ARPA. ( 870729 1800 300 604800 86400 )
    #[test]
    fn test_from_master_file_edu() {
        // Master file for the EDU domain.
        let masterfile_path = "1034-scenario-6.1-edu.txt";

        // Verify that the file exists before continuing
        assert!(Path::new(masterfile_path).exists(), "EDU master file not found.");

        // Create the zone from the master file
        let dns_zone = DnsZone::from_master_file(masterfile_path).unwrap();

        // Validate main properties of the zone
        assert_eq!(dns_zone.name, "EDU."); // The example does not have a line with $ORIGIN
        assert_eq!(dns_zone.class, Rclass::IN);
        assert_eq!(dns_zone.ttl, 86400); // Default TTL in the file is 3600, the example does not have a line with $TTL
        assert_eq!(dns_zone.soa.get_mname().get_name(), "SRI-NIC.ARPA.");
        assert_eq!(dns_zone.soa.get_rname().get_name(), "HOSTMASTER.SRI-NIC.ARPA.");
        assert_eq!(dns_zone.soa.get_serial(), 870729);
        assert_eq!(dns_zone.soa.get_refresh(), 1800);
        assert_eq!(dns_zone.soa.get_retry(), 300);
        assert_eq!(dns_zone.soa.get_expire(), 604800);
        assert_eq!(dns_zone.soa.get_minimum(), 86400);

        // Validate name server records
        assert_eq!(dns_zone.get_ns_records().len(), 13);
        assert!(dns_zone.get_ns_records().contains(&"SRI-NIC.ARPA.".to_string()));
        assert!(dns_zone.get_ns_records().contains(&"C.ISI.EDU.".to_string()));

        // Validate resource records
        assert_eq!(dns_zone.get_resource_records().len(), 11); // Count A, NS, etc. records
        assert!(dns_zone.get_resource_records().iter().any(|rr| rr.get_name().get_name() == "ICS.UCI" && matches!(rr.get_rdata(), Rdata::A(_))));
        //assert!(dns_zone.get_resource_records().iter().any(|rr| rr.get_name().get_name() == "YALE.EDU." && matches!(rr.get_rdata(), Rdata::NS(_))));
    }


    /// Disclaimer!!
    /// The SOA must be all in only one line
    /// The file used in the next text does not have the SOA in one line
    /// Change the file to test the function
    /// New SOA:
    /// .   IN        SOA     SRI-NIC.ARPA.      HOSTMASTER.SRI-NIC.ARPA. ( 870611 1800 300 604800 86400 )
    #[test]
    fn test_from_master_file_root() {
        // Master file for the root zone.
        let masterfile_path = "1034-scenario-6.1-root.txt";

        // Verify that the file exists before continuing
        assert!(Path::new(masterfile_path).exists(), "Root master file not found.");

        // Create the zone from the master file
        let dns_zone = DnsZone::from_master_file(masterfile_path).unwrap();

        // Validate main properties of the zone
        assert_eq!(dns_zone.name, ".");
        assert_eq!(dns_zone.class, Rclass::IN);
        assert_eq!(dns_zone.ttl, 86400);
        assert_eq!(dns_zone.soa.get_mname().get_name(), "SRI-NIC.ARPA.");
        assert_eq!(dns_zone.soa.get_rname().get_name(), "HOSTMASTER.SRI-NIC.ARPA.");
        assert_eq!(dns_zone.soa.get_serial(), 870611);
        assert_eq!(dns_zone.soa.get_refresh(), 1800);
        assert_eq!(dns_zone.soa.get_retry(), 300);
        assert_eq!(dns_zone.soa.get_expire(), 604800);
        assert_eq!(dns_zone.soa.get_minimum(), 86400);

        // Validate name server records
        assert_eq!(dns_zone.ns_records.len(), 7);
        assert!(dns_zone.get_ns_records().contains(&"A.ISI.EDU.".to_string()));
        assert!(dns_zone.get_ns_records().contains(&"C.ISI.EDU.".to_string()));
        assert!(dns_zone.get_ns_records().contains(&"SRI-NIC.ARPA.".to_string()));

        // Validate resource records
        assert_eq!(dns_zone.get_resource_records().len(), 5); // Count A, MX, HINFO, etc. records // It Should be 16
        //assert!(dns_zone.get_resource_records().iter().any(|rr| rr.get_name().get_name() == "MIL." && matches!(rr.get_rdata(), Rdata::NS(_))));
        assert!(dns_zone.get_resource_records().iter().any(|rr| rr.get_name().get_name() == "A.ISI.EDU" && matches!(rr.get_rdata(), Rdata::A(_))));
    }
}