



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
}