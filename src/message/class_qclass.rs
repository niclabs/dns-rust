use std::fmt;

#[derive(Clone, PartialEq, Debug, Hash, PartialOrd, Ord, Eq, Copy)]
/// Enum for the Class of a RR in a DnsMessage
pub enum Qclass {
    IN,
    CS,
    CH,
    HS,
    ANY,
    UNKNOWN(u16),
}

impl From<&str> for Qclass {
    fn from(qclass: &str) -> Self {
        match qclass {
            "IN" => Qclass::IN,
            "CS" => Qclass::CS,
            "CH" => Qclass::CH,
            "HS" => Qclass::HS,
            "ANY" => Qclass::ANY,
            _ => Qclass::UNKNOWN(99)
        }
    }
}

impl From<u16> for Qclass {
    fn from(val: u16) -> Self {
        match val {
            1 => Qclass::IN,
            2 => Qclass::CS,
            3 => Qclass::CH,
            4 => Qclass::HS,
            255 => Qclass::ANY,
            _ => Qclass::UNKNOWN(val)
        }
    }
}

impl From<Qclass> for u16 {
    fn from(class: Qclass) -> Self {
        match class {
            Qclass::IN => 1,
            Qclass::CS => 2,
            Qclass::CH => 3,
            Qclass::HS => 4,
            Qclass::ANY => 255,
            Qclass::UNKNOWN(val) => val,
        }
    }
}

impl Default for Qclass {
    fn default() -> Self { Qclass::IN }
}

impl fmt::Display for Qclass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Qclass::IN => "IN",
            Qclass::CS => "CS",
            Qclass::CH => "CH",
            Qclass::HS => "HS",
            Qclass::ANY => "ANY",
            Qclass::UNKNOWN(_) => "UNKNOWN",
        })
    }
}