use std::fmt;

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
    NSEC3PARAM,
    TSIG,
    UNKNOWN(u16),
}

/// Functions for the RType Enum
impl Rtype {
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
            Rtype::NSEC3PARAM => 51,
            Rtype::TSIG => 250,
            Rtype::UNKNOWN(val) => val
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
            51 => Rtype::NSEC3PARAM,
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
            "NSEC3PARAM" => Rtype::NSEC3PARAM,
            "TSIG" => Rtype::TSIG,
            _ => Rtype::UNKNOWN(99),
        }
    }
}

impl Default for Rtype {
    fn default() -> Self { Rtype::A }
}

impl fmt::Display for Rtype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Rtype::A => "A",
            Rtype::NS => "NS",
            Rtype::CNAME => "CNAME",
            Rtype::SOA => "SOA",
            Rtype::PTR => "PTR",
            Rtype::HINFO => "HINFO",
            Rtype::MINFO => "MINFO",
            Rtype::WKS => "WKS",
            Rtype::MX => "MX",
            Rtype::TXT => "TXT",
            Rtype::AAAA => "AAAA",
            Rtype::DNAME => "DNAME",
            Rtype::OPT => "OPT",
            Rtype::DS => "DS",
            Rtype::RRSIG => "RRSIG",
            Rtype::NSEC => "NSEC",
            Rtype::DNSKEY => "DNSKEY",
            Rtype::NSEC3 => "NSEC3",
            Rtype::NSEC3PARAM => "NSEC3PARAM",
            Rtype::TSIG => "TSIG",
            Rtype::UNKNOWN(_) => "UNKNOWN",
        })
    }
}