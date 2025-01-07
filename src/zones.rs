use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
pub mod SoaRdata;
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
struct DnsZone {
    name: String,           // Name of the zone (e.g., "example.com")
    ttl: u32,               // Default time to live (in seconds)
    soa: SoaRdata,         // SOA (Start of Authority) record
    ns_records: Vec<String>,// List of name servers (NS) // Chech this part in the furure ********
    a_records: Vec<ARecord>,// List of A records
    mx_records: Vec<MxRecord>, // List of MX records
    other_records: Vec<DnsRecord>, // Other records (CNAME, PTR, etc.)
}

/// Structure for an A record (IPv4 address)
#[derive(Debug)]
struct ARecord {
    name: String,            // Subdomain name (e.g., "www")
    ip: String,              // IPv4 address (e.g., "192.0.2.1")
}

/// Structure for an MX record (mail server)
#[derive(Debug)]
struct MxRecord {
    priority: u16,           // Mail server priority
    mail_server: String,     // Mail server address
}

/// Generic structure for other records (e.g., CNAME, PTR, TXT)
#[derive(Debug)]
struct DnsRecord {
    record_type: String,     // Record type (e.g., "CNAME", "TXT")
    name: String,            // Domain/subdomain name
    value: String,           // Value associated with the record
}

impl DnsZone {
    /// Creates a new DNS zone.
    ///
    /// # Examples
    ///
    /// ```
    /// let soa = SoaRdata::new(
    ///     "ns1.example.com.",
    ///     "admin.example.com.",
    ///     20240101,
    ///     3600,
    ///     1800,
    ///     1209600,
    ///     3600,
    /// );
    ///
    /// let dns_zone = DnsZone::new("example.com.", 3600, soa);
    ///
    /// assert_eq!(dns_zone.name, "example.com.".to_string());
    /// assert_eq!(dns_zone.ttl, 3600);
    /// assert_eq!(dns_zone.soa.primary_ns, "ns1.example.com.".to_string());
    /// assert_eq!(dns_zone.soa.admin_email, "admin.example.com.".to_string());
    /// ```
    fn new(name: &str, ttl: u32, soa: SoaRdata) -> Self {
        DnsZone {
            name: name.to_string(),
            ttl,
            soa,
            ns_records: Vec::new(),
            a_records: Vec::new(),
            mx_records: Vec::new(),
            other_records: Vec::new(),
        }
    }

    /// Adds a new NS (Name Server) record to the DNS zone.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut dns_zone = DnsZone::new(
    ///     "example.com.",
    ///     3600,
    ///     SoaRdata::new("ns1.example.com.", "admin.example.com.", 20240101, 3600, 1800, 1209600, 3600),
    /// );
    ///
    /// dns_zone.add_ns_record("ns2.example.com.");
    ///
    /// assert_eq!(dns_zone.ns_records.len(), 1);
    /// assert_eq!(dns_zone.ns_records[0], "ns2.example.com.".to_string());
    /// ```
    fn add_ns_record(&mut self, ns: &str) {
        self.ns_records.push(ns.to_string());
    }

    /// Adds a new A (Address) record to the DNS zone.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut dns_zone = DnsZone::new(
    ///     "example.com.",
    ///     3600,
    ///     SoaRdata::new("ns1.example.com.", "admin.example.com.", 20240101, 3600, 1800, 1209600, 3600),
    /// );
    ///
    /// dns_zone.add_a_record("www", "192.0.2.1");
    ///
    /// assert_eq!(dns_zone.a_records.len(), 1);
    /// assert_eq!(dns_zone.a_records[0].name, "www".to_string());
    /// assert_eq!(dns_zone.a_records[0].ip, "192.0.2.1".to_string());
    /// ```
    fn add_a_record(&mut self, name: &str, ip: &str) {
        self.a_records.push(ARecord {
            name: name.to_string(),
            ip: ip.to_string(),
        });
    }

    /// Adds a new MX (Mail Exchange) record to the DNS zone.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut dns_zone = DnsZone::new(
    ///     "example.com.",
    ///     3600,
    ///     SoaRdata::new("ns1.example.com.", "admin.example.com.", 20240101, 3600, 1800, 1209600, 3600),
    /// );
    ///
    /// dns_zone.add_mx_record(10, "mail.example.com.");
    ///
    /// assert_eq!(dns_zone.mx_records.len(), 1);
    /// assert_eq!(dns_zone.mx_records[0].priority, 10);
    /// assert_eq!(dns_zone.mx_records[0].mail_server, "mail.example.com.".to_string());
    /// ```

    fn add_mx_record(&mut self, priority: u16, mail_server: &str) {
        self.mx_records.push(MxRecord {
            priority,
            mail_server: mail_server.to_string(),
        });
    }

    /// Adds a generic DNS record (e.g., CNAME, PTR, TXT) to the DNS zone.
    ///
    /// This method allows adding DNS records that do not have a dedicated function,
    /// such as CNAME, PTR, or TXT records, by specifying their type, name, and value.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut dns_zone = DnsZone::new(
    ///     "example.com.",
    ///     3600,
    ///     SoaRdata::new("ns1.example.com.", "admin.example.com.", 20240101, 3600, 1800, 1209600, 3600),
    /// );
    ///
    /// dns_zone.add_generic_record("CNAME", "blog", "www.example.com.");
    /// dns_zone.add_generic_record("TXT", "@", "v=spf1 include:example.com ~all");
    ///
    /// assert_eq!(dns_zone.other_records.len(), 2);
    /// assert_eq!(dns_zone.other_records[0].record_type, "CNAME".to_string());
    /// assert_eq!(dns_zone.other_records[0].name, "blog".to_string());
    /// assert_eq!(dns_zone.other_records[0].value, "www.example.com.".to_string());
    /// assert_eq!(dns_zone.other_records[1].record_type, "TXT".to_string());
    /// assert_eq!(dns_zone.other_records[1].name, "@".to_string());
    /// assert_eq!(dns_zone.other_records[1].value, "v=spf1 include:example.com ~all".to_string());
    /// ```
    fn add_generic_record(&mut self, record_type: &str, name: &str, value: &str) {
        self.other_records.push(DnsRecord {
            record_type: record_type.to_string(),
            name: name.to_string(),
            value: value.to_string(),
        });
    }

    /// Create a new DNS zone from a master file
    fn from_master_file(file_path: &str) -> io::Result<Self> {
        // Open the master file
        let path = Path::new(file_path);
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        let mut name = String::new();
        let mut ttl = 3600; // Default TTL
        let mut soa = None;
        let mut ns_records = Vec::new();
        let mut a_records = Vec::new();
        let mut mx_records = Vec::new();
        let mut other_records = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            // Ignore empty lines and comments
            if line.is_empty() || line.starts_with(';') {
                continue;
            }

            // Process special directives
            if line.starts_with("$ORIGIN") {
                name = line.split_whitespace().nth(1).unwrap_or("").to_string();
                continue;
            }
            if line.starts_with("$TTL") {
                ttl = line.split_whitespace().nth(1).unwrap_or("3600").parse().unwrap_or(3600);
                continue;
            }

            // Split line into fields
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 4 {
                continue; // Malformed lines
            }

            let record_name = parts[0];
            let record_type = parts[2];
            match record_type {
                "SOA" => {
                    // Process SOA
                    if parts.len() >= 7 {
                        soa = Some(SoaRdata {
                            primary_ns: parts[3].to_string(),
                            admin_email: parts[4].to_string(),
                            serial: parts[5].parse().unwrap_or(0),
                            refresh: parts[6].parse().unwrap_or(3600),
                            retry: parts.get(7).unwrap_or(&"3600").parse().unwrap_or(3600),
                            expire: parts.get(8).unwrap_or(&"3600").parse().unwrap_or(1209600),
                            minimum_ttl: parts.get(9).unwrap_or(&"3600").parse().unwrap_or(3600),
                        });
                    }
                }
                "NS" => {
                    // Process NS
                    ns_records.push(parts[3].to_string());
                }
                "A" => {
                    // Process A
                    a_records.push(ARecord {
                        name: record_name.to_string(),
                        ip: parts[3].to_string(),
                    });
                }
                "MX" => {
                    // Process MX
                    if parts.len() >= 5 {
                        mx_records.push(MxRecord {
                            priority: parts[3].parse().unwrap_or(10),
                            mail_server: parts[4].to_string(),
                        });
                    }
                }
                _ => {
                    // Process other records (CNAME, PTR, etc.)
                    if parts.len() >= 4 {
                        other_records.push(DnsRecord {
                            record_type: record_type.to_string(),
                            name: record_name.to_string(),
                            value: parts[3].to_string(),
                        });
                    }
                }
            }
        }
    }

    /// Returns a formatted string with all the DNS zone's information.
    ///
    /// # Examples
    ///
    /// ```
    /// let soa = SoaRdata::new(
    ///     "ns1.example.com.",
    ///     "admin.example.com.",
    ///     20240101,
    ///     3600,
    ///     1800,
    ///     1209600,
    ///     3600,
    /// );
    ///
    /// let dns_zone = DnsZone::new("example.com.", 3600, soa);
    ///
    /// let info = dns_zone.get_info();
    /// assert!(info.contains("example.com."));
    /// assert!(info.contains("ns1.example.com."));
    /// assert!(info.contains("admin.example.com."));
    /// ```
    fn get_info(&self) -> String {
        format!("{:#?}", self)
    }
}