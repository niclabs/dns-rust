use std::fmt;

#[derive(Clone, PartialEq, Debug, Hash, PartialOrd, Ord, Eq, Copy)]
/// Enum for the Class of a RR in a DnsMessage
pub enum Rclass {
    IN,
    CS,
    CH,
    HS,
    ANY,
    UNKNOWN(u16),
}

impl From<&str> for Rclass {
    fn from(rclass: &str) -> Self {
        match rclass {
            "IN" => Rclass::IN,
            "CS" => Rclass::CS,
            "CH" => Rclass::CH,
            "HS" => Rclass::HS,
            "ANY" => Rclass::ANY,
            _ => Rclass::UNKNOWN(99)
        }
    }
}

impl From<u16> for Rclass {
    fn from(val: u16) -> Self {
        match val {
            1 => Rclass::IN,
            2 => Rclass::CS,
            3 => Rclass::CH,
            4 => Rclass::HS,
            255 => Rclass::ANY,
            _ => Rclass::UNKNOWN(val)
        }
    }
}

impl From<Rclass> for u16 {
    fn from(class: Rclass) -> Self {
        match class {
            Rclass::IN => 1,
            Rclass::CS => 2,
            Rclass::CH => 3,
            Rclass::HS => 4,
            Rclass::ANY => 255,
            Rclass::UNKNOWN(val) => val,
        }
    }
}

impl Default for Rclass {
    fn default() -> Self { Rclass::IN }
}

impl fmt::Display for Rclass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Rclass::IN => "IN",
            Rclass::CS => "CS",
            Rclass::CH => "CH",
            Rclass::HS => "HS",
            Rclass::ANY => "ANY",
            Rclass::UNKNOWN(_) => "UNKNOWN",
        })
    }
}