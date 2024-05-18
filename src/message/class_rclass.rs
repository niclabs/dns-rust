use std::fmt;

#[derive(Clone, PartialEq, Debug)]
/// Enum for the Class of a RR in a DnsMessage
pub enum Rclass {
    IN,
    CS,
    CH,
    HS,
    UNKNOWN(u16),
}

impl From<Rclass> for u16 {
    fn from(class: Rclass) -> u16 {
        match class {
            Rclass::IN => 1,
            Rclass::CS => 2,
            Rclass::CH => 3,
            Rclass::HS => 4,
            Rclass::UNKNOWN(val) => val,
        }
    }
}

impl From<u16> for Rclass {
    fn from(val: u16) -> Rclass {
        match val {
            1 => Rclass::IN,
            2 => Rclass::CS,
            3 => Rclass::CH,
            4 => Rclass::HS,
            _ => Rclass::UNKNOWN(val)
        }
    }
}

impl From<&str> for Rclass {
    fn from(rclass: &str) -> Rclass{
        match rclass {
            "IN" => Rclass::IN,
            "CS" => Rclass::CS,
            "CH" => Rclass::CH,
            "HS" => Rclass::HS,
            _ => Rclass::UNKNOWN(99)
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
            Rclass::UNKNOWN(_) => "UNKNOWN",
        })
    }
}