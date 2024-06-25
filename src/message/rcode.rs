use std::fmt;



#[derive(Clone, PartialEq, Debug, Hash, PartialOrd, Ord, Eq, Copy)]
// Enum for the RCODE of a DnsMessage
pub enum Rcode {
    NOERROR,
    FORMERR,
    SERVFAIL,
    NXDOMAIN,
    NOTIMP,
    REFUSED,
    UNKNOWN(u8),
}

impl From<u8> for Rcode {
    fn from(int: u8) -> Rcode {
        match int {
            0 => Rcode::NOERROR,
            1 => Rcode::FORMERR,
            2 => Rcode::SERVFAIL,
            3 => Rcode::NXDOMAIN,
            4 => Rcode::NOTIMP,
            5 => Rcode::REFUSED,
            _ => Rcode::UNKNOWN(int),
        }
    }
}

impl From<Rcode> for u8 {
    fn from(rcode: Rcode) -> u8 {
        match rcode {
            Rcode::NOERROR => 0,
            Rcode::FORMERR => 1,
            Rcode::SERVFAIL => 2,
            Rcode::NXDOMAIN => 3,
            Rcode::NOTIMP => 4,
            Rcode::REFUSED => 5,
            Rcode::UNKNOWN(u8) => u8,
        }
    }
}

impl From<&str> for Rcode {
    fn from(str: &str) -> Rcode {
        match str {
            "NOERROR" => Rcode::NOERROR,
            "FORMERR" => Rcode::FORMERR,
            "SERVFAIL" => Rcode::SERVFAIL,
            "NXDOMAIN" => Rcode::NXDOMAIN,
            "NOTIMP" => Rcode::NOTIMP,
            "REFUSED" => Rcode::REFUSED,
            _ => Rcode::UNKNOWN(0),
        }
    }
}

impl Default for Rcode {
    fn default() -> Rcode { Rcode::NOERROR }
}

impl fmt::Display for Rcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Rcode::NOERROR => "NOERROR",
            Rcode::FORMERR => "FORMERR",
            Rcode::SERVFAIL => "SERVFAIL",
            Rcode::NXDOMAIN => "NXDOMAIN",
            Rcode::NOTIMP => "NOTIMP",
            Rcode::REFUSED => "REFUSED",
            Rcode::UNKNOWN(_) => "UNKNOWN",
        })
    }
}
