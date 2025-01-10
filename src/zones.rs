use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
pub mod SoaRdata;
pub mod ResourceRecord;
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
    ns_records: Vec<String>,// List of name servers (NS)
    resource_records: Vec<ResourceRecord>,// List of resource records
}

impl DnsZone {
    /// Creates a new DNS zone.
    ///
    /// # Examples
    ///
    /// ```
    /// let soa = SoaRdata {
    ///     mname: DomainName::new_from_str("ns1.example.com.".to_string()),
    ///     rname: DomainName::new_from_str("admin.example.com.".to_string()),
    ///     serial: 20240101,
    ///     refresh: 3600,
    ///     retry: 1800,
    ///     expire: 1209600,
    ///     minimum: 3600,
    /// };
    ///
    /// let dns_zone = DnsZone::new("example.com.", 3600, soa);
    ///
    /// assert_eq!(dns_zone.name, "example.com.".to_string());
    /// assert_eq!(dns_zone.ttl, 3600);
    /// assert_eq!(dns_zone.soa.serial, 20240101);
    /// assert!(dns_zone.ns_records.is_empty());
    /// assert!(dns_zone.resource_records.is_empty());
    /// ```
    fn new(name: &str, ttl: u32, soa: SoaRdata) -> Self {
        DnsZone {
            name: name.to_string(),
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
    ///         mname: DomainName::new_from_str("ns1.example.com.".to_string()),
    ///         rname: DomainName::new_from_str("admin.example.com.".to_string()),
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
    /// assert_eq!(dns_zone.ns_records[0], "ns2.example.com.".to_string());
    /// ```
    fn add_ns_record(&mut self, ns: &str) {
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
    ///         mname: DomainName::new_from_str("ns1.example.com.".to_string()),
    ///         rname: DomainName::new_from_str("admin.example.com.".to_string()),
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
    fn add_resource_record(&mut self, record: ResourceRecord) {
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
    pub fn from_master_file(file_path: &str) -> io::Result<Self> {
        // Abrir el archivo
        let path = Path::new(file_path);
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        // Variables para la zona
        let mut name = String::new();
        let mut ttl = 3600; // Valor predeterminado
        let mut soa: Option<SoaRdata> = None;
        let mut ns_records = Vec::new();
        let mut resource_records = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            // Ignorar líneas vacías y comentarios
            if line.is_empty() || line.starts_with(';') {
                continue;
            }

            // Procesar directivas
            if line.starts_with("$ORIGIN") {
                name = line.split_whitespace().nth(1).unwrap_or("").to_string();
                continue;
            }
            if line.starts_with("$TTL") {
                ttl = line.split_whitespace().nth(1).unwrap_or("3600").parse().unwrap_or(3600);
                continue;
            }

            // Procesar registros
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 4 {
                continue; // Línea mal formada
            }

            let record_name = parts[0];
            let record_type = parts[2];
            match record_type {
                "SOA" => {
                    if parts.len() >= 7 {
                        soa = Some(SoaRdata {
                            mname: DomainName::new_from_str(parts[3].to_string()),
                            rname: DomainName::new_from_str(parts[4].to_string()),
                            serial: parts[5].parse().unwrap_or(0),
                            refresh: parts[6].parse().unwrap_or(3600),
                            retry: parts.get(7).unwrap_or(&"1800").parse().unwrap_or(1800),
                            expire: parts.get(8).unwrap_or(&"1209600").parse().unwrap_or(1209600),
                            minimum: parts.get(9).unwrap_or(&"3600").parse().unwrap_or(3600),
                        });
                    }
                }
                "NS" => {
                    ns_records.push(parts[3].to_string());
                }
                "A" | "AAAA" | "CNAME" | "MX" | "TXT" | "PTR" => {
                    let rdata = match record_type {
                        "A" => Rdata::A(parts[3].to_string().parse().unwrap()),
                        "AAAA" => Rdata::AAAA(parts[3].to_string().parse().unwrap()),
                        "CNAME" => Rdata::CNAME(DomainName::new_from_str(parts[3].to_string())),
                        "MX" => Rdata::MX(MxRdata::new(
                            parts[3].parse().unwrap_or(10),
                            DomainName::new_from_str(parts[4].to_string()),
                        )),
                        "TXT" => Rdata::TXT(TxtRdata::new(parts[3].to_string())),
                        "PTR" => Rdata::PTR(DomainName::new_from_str(parts[3].to_string())),
                        _ => continue,
                    };

                    let resource_record = ResourceRecord {
                        name: DomainName::new_from_str(record_name.to_string()),
                        rtype: match record_type {
                            "A" => Rrtype::A,
                            "AAAA" => Rrtype::AAAA,
                            "CNAME" => Rrtype::CNAME,
                            "MX" => Rrtype::MX,
                            "TXT" => Rrtype::TXT,
                            "PTR" => Rrtype::PTR,
                            _ => Rrtype::UNKNOWN(0),
                        },
                        rclass: Rclass::IN,
                        ttl,
                        rdlength: rdata.to_bytes().len() as u16,
                        rdata,
                    };

                    resource_records.push(resource_record);
                }
                _ => {}
            }
        }

        // Validar y construir la zona
        Ok(DnsZone {
            name,
            ttl,
            soa: soa.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "SOA record missing"))?,
            ns_records,
            resource_records,
        })
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


#[cfg(test)]
mod dns_zone_tests {
    use super::*; // Importar la estructura y métodos de DnsZone

    #[test]
    fn test_new() {
        let soa = SoaRdata {
            mname: DomainName::new_from_str("ns1.example.com.".to_string()),
            rname: DomainName::new_from_str("admin.example.com.".to_string()),
            serial: 20240101,
            refresh: 3600,
            retry: 1800,
            expire: 1209600,
            minimum: 3600,
        };

        let dns_zone = DnsZone::new("example.com.", 3600, soa);

        assert_eq!(dns_zone.name, "example.com.".to_string());
        assert_eq!(dns_zone.ttl, 3600);
        assert_eq!(dns_zone.soa.mname.get_name(), "ns1.example.com.".to_string());
        assert_eq!(dns_zone.soa.serial, 20240101);
        assert!(dns_zone.ns_records.is_empty());
        assert!(dns_zone.resource_records.is_empty());
    }

    #[test]
    fn test_add_ns_record() {
        let mut dns_zone = DnsZone::new(
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

        dns_zone.add_ns_record("ns2.example.com.");

        assert_eq!(dns_zone.ns_records.len(), 1);
        assert_eq!(dns_zone.ns_records[0], "ns2.example.com.".to_string());
    }

    #[test]
    fn test_add_resource_record() {
        let mut dns_zone = DnsZone::new(
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

        let txt_rdata = Rdata::TXT(TxtRdata::new(String::from("dcc")));
        let resource_record = ResourceRecord::new(txt_rdata);

        dns_zone.add_resource_record(resource_record);

        assert_eq!(dns_zone.resource_records.len(), 1);
        assert_eq!(dns_zone.resource_records[0].rdata.unwrap().get_text(),String::from("dcc"));
    }
}