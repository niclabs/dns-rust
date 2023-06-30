#[derive(Clone, PartialEq, Debug)]
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
            Rtype::UNKNOWN(_val) => String::from("UNKNOWN TYPE") 
        }
    }

    /// Function to get the String equivalent of a type
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
            _ => Rtype::UNKNOWN(99),
        }
    }
}

impl Default for Rtype {
    fn default() -> Self { Rtype::A }
}