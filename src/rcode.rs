use std::fmt;



#[derive(Clone, PartialEq, Debug, Hash, PartialOrd, Ord, Eq, Copy)]
// Enum for the RCODE of a DnsMessage
enum Rcode {
    NOERROR,
    FORMERR,
    SERVFAIL,
    NXDOMAIN,
    NOTIMP,
    REFUSED,
    UNKNOWN(u8),
}

impl Rcode {
    // Function to get the int equivalent of a Rcode
    pub fn from_rcode_to_int(rcode: Rcode) -> u8 {
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

    // Function to get the Rcode equivalent of an int
    pub fn from_int_to_rcode(int: u8) -> Rcode {
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

    // Function to get the Rcode equivalent of a string
    pub fn from_string_to_rcode(string: &str) -> Rcode {
        match string {
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
        match *self {
            Rcode::NOERROR => "NOERROR",
            Rcode::FORMERR => "FORMERR",
            Rcode::SERVFAIL => "SERVFAIL",
            Rcode::NXDOMAIN => "NXDOMAIN",
            Rcode::NOTIMP => "NOTIMP",
            Rcode::REFUSED => "REFUSED",
            Rcode::UNKNOWN(u8) => "UNKNOWN",
        }
    }
}
