use std::fmt;
#[derive(Clone, PartialEq, Debug, Hash, PartialOrd, Ord, Eq, Copy)]
/// Enum For the Type of a RR in a DnsMessage with an Rdata implementation
pub enum Rrtype {
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

impl From<Rrtype> for u16 {
    fn from(rrtype: Rrtype) -> u16 {
        match rrtype {
            Rrtype::A => 1,
            Rrtype::NS => 2,
            Rrtype::CNAME => 5,
            Rrtype::SOA => 6,
            Rrtype::WKS => 11,
            Rrtype::PTR => 12,
            Rrtype::HINFO => 13,
            Rrtype::MINFO => 14,
            Rrtype::MX => 15,
            Rrtype::TXT => 16,
            Rrtype::AAAA => 28,
            Rrtype::DNAME => 39,
            Rrtype::OPT => 41,
            Rrtype::DS => 43,
            Rrtype::RRSIG => 46,
            Rrtype::NSEC => 47,
            Rrtype::DNSKEY => 48,
            Rrtype::NSEC3 => 50,
            Rrtype::NSEC3PARAM => 51,
            Rrtype::AXFR => 252,
            Rrtype::TSIG => 250,
            Rrtype::MAILB => 253,
            Rrtype::MAILA => 254,
            Rrtype::ANY => 255,
            Rrtype::UNKNOWN(val) => val,
        }
    }
}
impl From<u16> for Rrtype {
    fn from(val: u16) -> Rrtype {
        match val {
            1 => Rrtype::A,
            2 => Rrtype::NS,
            5 => Rrtype::CNAME,
            6 => Rrtype::SOA,
            11 => Rrtype::WKS,
            12 => Rrtype::PTR,
            13 => Rrtype::HINFO,
            14 => Rrtype::MINFO,
            15 => Rrtype::MX,
            16 => Rrtype::TXT,
            28 => Rrtype::AAAA,
            39 => Rrtype::DNAME,
            41 => Rrtype::OPT,
            43 => Rrtype::DS,
            46 => Rrtype::RRSIG,
            47 => Rrtype::NSEC,
            48 => Rrtype::DNSKEY,
            50 => Rrtype::NSEC3,
            51 => Rrtype::NSEC3PARAM,
            250 => Rrtype::TSIG,
            252 => Rrtype::AXFR,
            253 => Rrtype::MAILB,
            254 => Rrtype::MAILA,
            255 => Rrtype::ANY,
            _ => Rrtype::UNKNOWN(val),
        }
    }
}
impl From<&str> for Rrtype {
    fn from(rrtype: &str) -> Rrtype {
        match rrtype {
            "A" => Rrtype::A,
            "NS" => Rrtype::NS,
            "CNAME" => Rrtype::CNAME,
            "SOA" => Rrtype::SOA,
            "WKS" => Rrtype::WKS,
            "PTR" => Rrtype::PTR,
            "HINFO" => Rrtype::HINFO,
            "MINFO" => Rrtype::MINFO,
            "MX" => Rrtype::MX,
            "TXT" => Rrtype::TXT,
            "AAAA" => Rrtype::AAAA,
            "DNAME" => Rrtype::DNAME,
            "OPT" => Rrtype::OPT,
            "DS" => Rrtype::DS,
            "RRSIG" => Rrtype::RRSIG,
            "NSEC" => Rrtype::NSEC,
            "DNSKEY" => Rrtype::DNSKEY,
            "NSEC3" => Rrtype::NSEC3,
            "NSEC3PARAM" => Rrtype::NSEC3PARAM,
            "TSIG" => Rrtype::TSIG,
            "AXFR" => Rrtype::AXFR,
            "MAILB" => Rrtype::MAILB,
            "MAILA" => Rrtype::MAILA,
            "ANY" => Rrtype::ANY,
            _ => Rrtype::UNKNOWN(99),
        }
    }
}
impl Default for Rrtype {
    fn default() -> Self { Rrtype::A }
}
impl fmt::Display for Rrtype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Rrtype::A => "A",
            Rrtype::NS => "NS",
            Rrtype::CNAME => "CNAME",
            Rrtype::SOA => "SOA",
            Rrtype::PTR => "PTR",
            Rrtype::HINFO => "HINFO",
            Rrtype::MINFO => "MINFO",
            Rrtype::WKS => "WKS",
            Rrtype::MX => "MX",
            Rrtype::TXT => "TXT",
            Rrtype::AAAA => "AAAA",
            Rrtype::DNAME => "DNAME",
            Rrtype::OPT => "OPT",
            Rrtype::DS => "DS",
            Rrtype::RRSIG => "RRSIG",
            Rrtype::NSEC => "NSEC",
            Rrtype::DNSKEY => "DNSKEY",
            Rrtype::NSEC3 => "NSEC3",
            Rrtype::NSEC3PARAM => "NSEC3PARAM",
            Rrtype::TSIG => "TSIG",
            Rrtype::AXFR => "AXFR",
            Rrtype::MAILB => "MAILB",
            Rrtype::MAILA => "MAILA",
            Rrtype::ANY => "ANY",
            Rrtype::UNKNOWN(_) => "UNKNOWN",
        })
    }
}