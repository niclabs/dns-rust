use std::fmt;

#[derive(Clone, PartialEq, Debug, Hash, PartialOrd, Ord, Eq, Copy)]
/// Enum For the Type of a RR in a DnsMessage with an Rdata implementation
pub enum Qtype {
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
    ANY,
    TSIG,
    AXFR,
    MAILB,
    MAILA,
    UNKNOWN(u16),
}

impl From<Qtype> for u16 {
    fn from(qtype: Qtype) -> u16 {
        match qtype {
            Qtype::A => 1,
            Qtype::NS => 2,
            Qtype::CNAME => 5,
            Qtype::SOA => 6,
            Qtype::WKS => 11,
            Qtype::PTR => 12,
            Qtype::HINFO => 13,
            Qtype::MINFO => 14,
            Qtype::MX => 15,
            Qtype::TXT => 16,
            Qtype::AAAA => 28,
            Qtype::DNAME => 39,
            Qtype::OPT => 41,
            Qtype::DS => 43,
            Qtype::RRSIG => 46,
            Qtype::NSEC => 47,
            Qtype::DNSKEY => 48,
            Qtype::NSEC3 => 50,
            Qtype::NSEC3PARAM => 51,
            Qtype::AXFR => 252,
            Qtype::TSIG => 250,
            Qtype::MAILB => 253,
            Qtype::MAILA => 254,
            Qtype::ANY => 255,
            Qtype::UNKNOWN(val) => val
        }
    }
}

impl From<u16> for Qtype {
    fn from(val: u16) -> Qtype {
        match val {
            1 => Qtype::A,
            2 => Qtype::NS,
            5 => Qtype::CNAME,
            6 => Qtype::SOA,
            11 => Qtype::WKS,
            12 => Qtype::PTR,
            13 => Qtype::HINFO,
            14 => Qtype::MINFO,
            15 => Qtype::MX,
            16 => Qtype::TXT,
            28 => Qtype::AAAA,
            39 => Qtype::DNAME,
            41 => Qtype::OPT,
            43 => Qtype::DS,
            46 => Qtype::RRSIG,
            47 => Qtype::NSEC,
            48 => Qtype::DNSKEY,
            50 => Qtype::NSEC3,
            51 => Qtype::NSEC3PARAM,
            250 => Qtype::TSIG,
            252 => Qtype::AXFR,
            253 => Qtype::MAILB,
            254 => Qtype::MAILA,
            255 => Qtype::ANY,
            _ => Qtype::UNKNOWN(val),
        }
    }
}

impl From<&str> for Qtype {
    fn from(qtype: &str) -> Qtype {
        match qtype {
            "A" => Qtype::A,
            "NS" => Qtype::NS,
            "CNAME" => Qtype::CNAME,
            "SOA" => Qtype::SOA,
            "WKS" => Qtype::WKS,
            "PTR" => Qtype::PTR,
            "HINFO" => Qtype::HINFO,
            "MINFO" => Qtype::MINFO,
            "MX" => Qtype::MX,
            "TXT" => Qtype::TXT,
            "AAAA" => Qtype::AAAA,
            "DNAME" => Qtype::DNAME,
            "OPT" => Qtype::OPT,
            "DS" => Qtype::DS,
            "RRSIG" => Qtype::RRSIG,
            "NSEC" => Qtype::NSEC,
            "DNSKEY" => Qtype::DNSKEY,
            "NSEC3" => Qtype::NSEC3,
            "NSEC3PARAM" => Qtype::NSEC3PARAM,
            "TSIG" => Qtype::TSIG,
            "AXFR" => Qtype::AXFR,
            "MAILB" => Qtype::MAILB,
            "MAILA" => Qtype::MAILA,
            "ANY" => Qtype::ANY,
            _ => Qtype::UNKNOWN(99),
        }
    }
}

impl Default for Qtype {
    fn default() -> Self { Qtype::A }
}

impl fmt::Display for Qtype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Qtype::A => "A",
            Qtype::NS => "NS",
            Qtype::CNAME => "CNAME",
            Qtype::SOA => "SOA",
            Qtype::PTR => "PTR",
            Qtype::HINFO => "HINFO",
            Qtype::MINFO => "MINFO",
            Qtype::WKS => "WKS",
            Qtype::MX => "MX",
            Qtype::TXT => "TXT",
            Qtype::AAAA => "AAAA",
            Qtype::DNAME => "DNAME",
            Qtype::OPT => "OPT",
            Qtype::DS => "DS",
            Qtype::RRSIG => "RRSIG",
            Qtype::NSEC => "NSEC",
            Qtype::DNSKEY => "DNSKEY",
            Qtype::NSEC3 => "NSEC3",
            Qtype::NSEC3PARAM => "NSEC3PARAM",
            Qtype::TSIG => "TSIG",
            Qtype::AXFR => "AXFR",
            Qtype::MAILB => "MAILB",
            Qtype::MAILA => "MAILA",
            Qtype::ANY => "ANY",
            Qtype::UNKNOWN(_) => "UNKNOWN",
        })
    }
}