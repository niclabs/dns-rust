use super::type_rtype::Rtype;

#[derive(Clone, PartialEq, Debug,Copy)]
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
    DNAME,
    OPT,
    RRSIG,
    DNSKEY,
    ANY,
    TSIG,
    AXFR,
    MAILB,
    MAILA,
    UNKNOWN(u16),
}

/// Functions for the Qtype Enum
impl Qtype{
    /// Function to get the int equivalent of a type
    pub fn from_qtype_to_int(qtype: Qtype) -> u16{
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
            Qtype::DNAME => 39,
            Qtype::OPT => 41,
            Qtype::RRSIG => 46,
            Qtype::DNSKEY => 48,
            Qtype::AXFR => 252,
            Qtype::TSIG => 250,
            Qtype::MAILB => 253,
            Qtype::MAILA => 254,
            Qtype::ANY => 255,
            Qtype::UNKNOWN(val) => val
        }
    }
    /// Function to get the String equivalent of a type
    pub fn from_qtype_to_str(qtype: Qtype) -> String {
        match qtype {
            Qtype::A => String::from("A"),
            Qtype::NS => String::from("NS"),
            Qtype::CNAME => String::from("CNAME"),
            Qtype::SOA => String::from("SOA"),
            Qtype::WKS => String::from("WKS"),
            Qtype::PTR => String::from("PTR"),
            Qtype::HINFO => String::from("HINFO"),
            Qtype::MINFO => String::from("MINFO"),
            Qtype::MX => String::from("MX"),
            Qtype::TXT => String::from("TXT"),
            Qtype::DNAME => String::from("DNAME"),
            Qtype::OPT => String::from("OPT"),
            Qtype::RRSIG => String::from("RRSIG"),
            Qtype::DNSKEY => String::from("DNSKEY"),
            Qtype::TSIG => String::from("TSIG"),
            Qtype::AXFR => String::from("AXFR"),
            Qtype::MAILB => String::from("MAILB"),
            Qtype::MAILA => String::from("MAILA"),
            Qtype::ANY => String::from("ANY"),
            Qtype::UNKNOWN(_val) => String::from("UNKNOWN TYPE") 
        }
    }

    /// Function to get the int equivalent of a type
    pub fn from_int_to_qtype(val: u16) -> Qtype{
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
            39 => Qtype::DNAME,
            41 => Qtype::OPT,
            46 => Qtype::RRSIG,
            48 => Qtype::DNSKEY,
            250 => Qtype::TSIG,
            252 => Qtype::AXFR,
            253 => Qtype::MAILB,
            254 => Qtype::MAILA,
            255 => Qtype::ANY,
            _ => Qtype::UNKNOWN(val),
        }
    }

    /// Function to get the Qtype from a String
    pub fn from_str_to_qtype(qtype: &str) -> Qtype {
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
            "DNAME" => Qtype::DNAME,
            "OPT" => Qtype::OPT,
            "DNSKEY" => Qtype::DNSKEY,
            "TSIG" => Qtype::TSIG,
            "AXFR" => Qtype::AXFR,
            "MAILB" => Qtype::MAILB,
            "MAILA" => Qtype::MAILA,
            "ANY" => Qtype::ANY,
            _ => Qtype::UNKNOWN(99),
        }
    }

    /// Parse Qtype to Rtype
    pub fn to_rtype(qtype: Qtype) -> Rtype {
        match qtype {
            Qtype::A => Rtype::A,
            Qtype::NS => Rtype::NS,
            Qtype::CNAME => Rtype::CNAME,
            Qtype::SOA => Rtype::SOA,
            Qtype::WKS => Rtype::WKS,
            Qtype::PTR => Rtype::PTR,
            Qtype::HINFO => Rtype::HINFO,
            Qtype::MINFO => Rtype::MINFO,
            Qtype::MX => Rtype::MX,
            Qtype::TXT => Rtype::TXT,
            Qtype::DNAME => Rtype::DNAME,
            Qtype::OPT => Rtype::OPT,
            Qtype::RRSIG => Rtype::RRSIG,
            Qtype::DNSKEY => Rtype::DNSKEY,
            _ => Rtype::UNKNOWN(Self::from_qtype_to_int(qtype))
        }
    } 
}

impl Default for Qtype {
    fn default() -> Self { Qtype::A }
}