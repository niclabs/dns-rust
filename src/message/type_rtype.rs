#[derive(Clone, PartialEq, Debug, Hash, PartialOrd, Ord, Eq, Copy)]
/// Enum For the Type of a RR in a DnsMessage with an Rdata implementation
pub enum Rtype {
    A,
    NS,
    CNAME,
    SOA,
    PTR,
    HINFO,
    MINFO,
    WKS,
    MX,
    TXT,
    AAAA,
    DNAME,
    OPT,
    DS,
    RRSIG,
    NSEC,   
    DNSKEY,
    NSEC3,
    TSIG,
    UNKNOWN(u16),
}

/// Functions for the RType Enum
impl Rtype{
    /// Function to get the int equivalent of a type
    pub fn from_rtype_to_int(rtype: Rtype) -> u16{
        match rtype {
            Rtype::A => 1,
            Rtype::NS => 2,
            Rtype::CNAME => 5,
            Rtype::SOA => 6,
            Rtype::WKS => 11,
            Rtype::PTR => 12,
            Rtype::HINFO => 13,
            Rtype::MINFO => 14,
            Rtype::MX => 15,
            Rtype::TXT => 16,
            Rtype::AAAA => 28,
            Rtype::DNAME => 39,
            Rtype::OPT => 41,
            Rtype::DS => 43,
            Rtype::RRSIG => 46,
            Rtype::NSEC => 47,
            Rtype::DNSKEY => 48,
            Rtype::NSEC3 => 50,
            Rtype::TSIG => 250,
            Rtype::UNKNOWN(val) => val
        }
    }
    /// Function to get the String equivalent of a type
    pub fn from_rtype_to_str(rtype: Rtype) -> String {
        match rtype {
            Rtype::A => String::from("A"),
            Rtype::NS => String::from("NS"),
            Rtype::CNAME => String::from("CNAME"),
            Rtype::SOA => String::from("SOA"),
            Rtype::WKS => String::from("WKS"),
            Rtype::PTR => String::from("PTR"),
            Rtype::HINFO => String::from("HINFO"),
            Rtype::MINFO => String::from("MINFO"),
            Rtype::MX => String::from("MX"),
            Rtype::TXT => String::from("TXT"),
            Rtype::AAAA => String::from("AAAA"),
            Rtype::DNAME => String::from("DNAME"),
            Rtype::OPT => String::from("OPT"),
            Rtype::DS => String::from("DS"),
            Rtype::RRSIG => String::from("RRSIG"),
            Rtype::NSEC => String::from("NSEC"),
            Rtype::DNSKEY => String::from("DNSKEY"),
            Rtype::NSEC3 => String::from("NSEC3"),
            Rtype::TSIG => String::from("TSIG"),
            Rtype::UNKNOWN(_val) => String::from("UNKNOWN TYPE") 
        }
    }

    /// Function to get the int equivalent of a type
    pub fn from_int_to_rtype(val: u16) -> Rtype{
        match val {
            1 => Rtype::A,
            2 => Rtype::NS,
            5 => Rtype::CNAME,
            6 => Rtype::SOA,
            11 => Rtype::WKS,
            12 => Rtype::PTR,
            13 => Rtype::HINFO,
            14 => Rtype::MINFO,
            15 => Rtype::MX,
            16 => Rtype::TXT,
            28 => Rtype::AAAA,
            39 => Rtype::DNAME,
            41 => Rtype::OPT,
            43 => Rtype::DS,
            46 => Rtype::RRSIG,
            47 => Rtype::NSEC,
            48 => Rtype::DNSKEY,
            50 => Rtype::NSEC3,
            250 => Rtype::TSIG,
            _ => Rtype::UNKNOWN(val),
        }
    }

    /// Function to get the Rtype from a String
    pub fn from_str_to_rtype(rtype: &str) -> Rtype {
        match rtype {
            "A" => Rtype::A,
            "NS" => Rtype::NS,
            "CNAME" => Rtype::CNAME,
            "SOA" => Rtype::SOA,
            "WKS" => Rtype::WKS,
            "PTR" => Rtype::PTR,
            "HINFO" => Rtype::HINFO,
            "MINFO" => Rtype::MINFO,
            "MX" => Rtype::MX,
            "TXT" => Rtype::TXT,
            "AAAA" => Rtype::AAAA,
            "DNAME" => Rtype::DNAME,
            "OPT" => Rtype::OPT,
            "DS" => Rtype::DS,
            "RRSIG" => Rtype::RRSIG,
            "NSEC" => Rtype::NSEC,
            "DNSKEY" => Rtype::DNSKEY,
            "NSEC3" => Rtype::NSEC3,
            "TSIG" => Rtype::TSIG,
            _ => Rtype::UNKNOWN(99),
        }
    }
}

impl Default for Rtype {
    fn default() -> Self { Rtype::A }
}