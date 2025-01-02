/// Structure to represent a DNS zone
#[derive(Debug)]
struct DnsZone {
    name: String,           // Name of the zone (e.g., "example.com")
    ttl: u32,               // Default time to live (in seconds)
    soa: SoaRecord,         // SOA (Start of Authority) record
    ns_records: Vec<String>,// List of name servers (NS)
    a_records: Vec<ARecord>,// List of A records
    mx_records: Vec<MxRecord>, // List of MX records
    other_records: Vec<DnsRecord>, // Other records (CNAME, PTR, etc.)
}

/// Structure to represent an SOA record
#[derive(Debug)]
struct SoaRecord {
    primary_ns: String,      // Primary name server (MNAME)
    admin_email: String,     // Administrator's email (RNAME)
    serial: u32,             // Serial number
    refresh: u32,            // Refresh interval (in seconds)
    retry: u32,              // Retry interval (in seconds)
    expire: u32,             // Expiration time (in seconds)
    minimum_ttl: u32,        // Minimum TTL for zone records
}

impl SoaRecord {
    /// Create a new SOA record
    fn new(primary_ns: &str, admin_email: &str, serial: u32, refresh: u32, retry: u32, expire: u32, minimum_ttl: u32) -> Self {
        SoaRecord {
            primary_ns: primary_ns.to_string(),
            admin_email: admin_email.to_string(),
            serial,
            refresh,
            retry,
            expire,
            minimum_ttl,
        }
    }
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
    /// Create a new DNS zone
    fn new(name: &str, ttl: u32, soa: SoaRecord) -> Self {
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

    /// Add an NS record
    fn add_ns_record(&mut self, ns: &str) {
        self.ns_records.push(ns.to_string());
    }

    /// Add an A record
    fn add_a_record(&mut self, name: &str, ip: &str) {
        self.a_records.push(ARecord {
            name: name.to_string(),
            ip: ip.to_string(),
        });
    }

    /// Add an MX record
    fn add_mx_record(&mut self, priority: u16, mail_server: &str) {
        self.mx_records.push(MxRecord {
            priority,
            mail_server: mail_server.to_string(),
        });
    }

    /// Add a generic record
    fn add_generic_record(&mut self, record_type: &str, name: &str, value: &str) {
        self.other_records.push(DnsRecord {
            record_type: record_type.to_string(),
            name: name.to_string(),
            value: value.to_string(),
        });
    }

    /// Get complete zone information
    fn get_info(&self) -> String {
        format!("{:#?}", self)
    }
}