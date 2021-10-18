use crate::domain_name::DomainName;
use crate::resource_record::{FromBytes, ToBytes};

#[derive(Clone)]
/// An struct that represents the rdata for ptr type
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// /                   PTRDNAME                    /
/// /                                               /
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///
pub struct PtrRdata {
    // A domain name which points to some location in the
    // domain name space.
    ptrdname: DomainName,
}

impl ToBytes for PtrRdata {
    /// Return a vec of bytes that represents the ptr rdata
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        let ptrdname = self.get_ptrdname();
        let ptrdname_bytes = ptrdname.to_bytes();

        for byte in ptrdname_bytes.as_slice() {
            bytes.push(*byte);
        }

        bytes
    }
}

impl FromBytes<PtrRdata> for PtrRdata {
    /// Creates a new PtrRdata from an array of bytes
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut ptr_rdata = PtrRdata::new();

        let (domain_name, _) = DomainName::from_bytes(bytes);
        ptr_rdata.set_ptrdname(domain_name);

        ptr_rdata
    }
}

impl PtrRdata {
    /// Creates a new PtrRdata with default values.
    ///
    /// # Examples
    /// ```
    /// let ptr_rdata = PtrRdata::new();
    ///
    /// assert_eq!(ptr_rdata.ptrdname.get_name(), String::from(""));
    /// ```
    ///
    pub fn new() -> Self {
        let ptr_rdata = PtrRdata {
            ptrdname: DomainName::new(),
        };

        ptr_rdata
    }
}

// Getters
impl PtrRdata {
    // Gets the ptrdname attribute from PtrRdata
    pub fn get_ptrdname(&self) -> DomainName {
        self.ptrdname.clone()
    }
}

// Setters
impl PtrRdata {
    // Sets the ptrdname attibute with a value
    pub fn set_ptrdname(&mut self, ptrdname: DomainName) {
        self.ptrdname = ptrdname;
    }
}

mod test {
    use crate::domain_name::DomainName;
    use crate::rdata::ptr_rdata::PtrRdata;
    use crate::resource_record::{FromBytes, ToBytes};

    #[test]
    fn constructor_test() {
        let ptr_rdata = PtrRdata::new();

        assert_eq!(ptr_rdata.ptrdname.get_name(), String::from(""));
    }

    #[test]
    fn set_and_get_nsdname_test() {
        let mut ptr_rdata = PtrRdata::new();

        assert_eq!(ptr_rdata.get_ptrdname().get_name(), String::from(""));

        let mut new_domain_name = DomainName::new();
        new_domain_name.set_name(String::from("test.com"));

        ptr_rdata.set_ptrdname(new_domain_name);

        assert_eq!(
            ptr_rdata.get_ptrdname().get_name(),
            String::from("test.com")
        );
    }

    #[test]
    fn to_bytes_test() {
        let mut domain_name = DomainName::new();
        let bytes_test: Vec<u8> = vec![
            4, 116, 101, 115, 116, 5, 116, 101, 115, 116, 50, 3, 99, 111, 109, 0,
        ];
        domain_name.set_name(String::from("test.test2.com"));

        let mut ptr_rdata = PtrRdata::new();
        ptr_rdata.set_ptrdname(domain_name);

        let bytes = ptr_rdata.to_bytes();

        for (index, byte) in bytes.iter().enumerate() {
            assert_eq!(*byte, bytes_test[index]);
        }
    }

    #[test]
    fn from_bytes_test() {
        let bytes_test: Vec<u8> = vec![
            4, 116, 101, 115, 116, 5, 116, 101, 115, 116, 50, 3, 99, 111, 109, 0,
        ];
        let ptr_rdata = PtrRdata::from_bytes(&bytes_test);

        assert_eq!(
            ptr_rdata.get_ptrdname().get_name(),
            String::from("test.test2.com")
        );
    }
}
