use crate::domain_name::DomainName;
use crate::message::resource_record::{FromBytes, ToBytes};

#[derive(Clone)]
/// An struct that represents the rdata for ns type
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// /                   NSDNAME                     /
/// /                                               /
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///
pub struct NsRdata {
    // A domain name which specifies a host which should be
    // authoritative for the specified class and domain.
    nsdname: DomainName,
}

impl ToBytes for NsRdata {
    /// Return a vec of bytes that represents the ns rdata
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        let nsdname = self.get_nsdname();
        let nsdname_bytes = nsdname.to_bytes();

        for byte in nsdname_bytes.as_slice() {
            bytes.push(*byte);
        }

        bytes
    }
}

impl FromBytes<NsRdata> for NsRdata {
    /// Creates a new NsRdata from an array of bytes
    fn from_bytes(bytes: &[u8], full_msg: &[u8]) -> Self {
        let mut ns_rdata = NsRdata::new();

        let (domain_name, _) = DomainName::from_bytes(bytes, full_msg);
        ns_rdata.set_nsdname(domain_name);

        ns_rdata
    }
}

impl NsRdata {
    /// Creates a new NsRdata with default values.
    ///
    /// # Examples
    /// ```
    /// let ns_rdata = NsRdata::new();
    ///
    /// assert_eq!(ns_rdata.nsdname.get_name(), String::from(""));
    /// ```
    ///
    pub fn new() -> Self {
        let ns_rdata = NsRdata {
            nsdname: DomainName::new(),
        };

        ns_rdata
    }
}

// Getters
impl NsRdata {
    // Gets the nsdname attribute from NsRdata
    pub fn get_nsdname(&self) -> DomainName {
        self.nsdname.clone()
    }
}

// Setters
impl NsRdata {
    // Sets the nsdname attibute with a value
    pub fn set_nsdname(&mut self, nsdname: DomainName) {
        self.nsdname = nsdname;
    }
}

mod test {
    use crate::domain_name::DomainName;
    use crate::message::rdata::ns_rdata::NsRdata;
    use crate::message::resource_record::{FromBytes, ToBytes};

    #[test]
    fn constructor_test() {
        let ns_rdata = NsRdata::new();

        assert_eq!(ns_rdata.nsdname.get_name(), String::from(""));
    }

    #[test]
    fn set_and_get_nsdname_test() {
        let mut ns_rdata = NsRdata::new();

        assert_eq!(ns_rdata.get_nsdname().get_name(), String::from(""));

        let mut new_domain_name = DomainName::new();
        new_domain_name.set_name(String::from("test.com"));

        ns_rdata.set_nsdname(new_domain_name);

        assert_eq!(ns_rdata.get_nsdname().get_name(), String::from("test.com"));
    }

    #[test]
    fn to_bytes_test() {
        let mut domain_name = DomainName::new();
        let bytes_test: Vec<u8> = vec![
            4, 116, 101, 115, 116, 5, 116, 101, 115, 116, 50, 3, 99, 111, 109, 0,
        ];
        domain_name.set_name(String::from("test.test2.com"));

        let mut ns_rdata = NsRdata::new();
        ns_rdata.set_nsdname(domain_name);

        let bytes = ns_rdata.to_bytes();

        for (index, byte) in bytes.iter().enumerate() {
            assert_eq!(*byte, bytes_test[index]);
        }
    }

    #[test]
    fn from_bytes_test() {
        let bytes_test: Vec<u8> = vec![
            4, 116, 101, 115, 116, 5, 116, 101, 115, 116, 50, 3, 99, 111, 109, 0,
        ];
        let ns_rdata = NsRdata::from_bytes(&bytes_test);

        assert_eq!(
            ns_rdata.get_nsdname().get_name(),
            String::from("test.test2.com")
        );
    }
}
