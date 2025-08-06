use crate::domain_name::DomainName;
use crate::message::Rclass;
use crate::message::rrtype::Rrtype;
use crate::message::rdata::Rdata;
use crate::message::resource_record::{FromBytes, ResourceRecord, ToBytes};
use std::str::SplitWhitespace;
use std::fmt;

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
/// Struct that represents the `Rdata` for PTR TYPE.
/// 
/// [RFC 1035](https://tools.ietf.org/html/rfc1035#section-3.3.12)
/// 
/// ```text
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// /                   PTRDNAME                    /
/// /                                               /
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
pub struct PtrRdata {
    /// A domain name which points to some location in the
    /// domain name space.
    ptrdname: DomainName,
}

impl ToBytes for PtrRdata {
    /// Return a vec of bytes that represents the ptr rdata.
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

impl FromBytes<Result<Self, &'static str>> for PtrRdata {
    /// Creates a new `PtrRdata` from an array of bytes.
    fn from_bytes(bytes: &[u8], full_msg: &[u8]) -> Result<Self, &'static str> {
        let bytes_len = bytes.len();

        if bytes_len < 2 {
            return Err("Format Error");
        }

        let domain_name_result = DomainName::from_bytes(bytes, full_msg);

        match domain_name_result {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        let mut ptr_rdata = PtrRdata::new();
        let (domain_name, _) = domain_name_result.unwrap();

        ptr_rdata.set_ptrdname(domain_name);

        Ok(ptr_rdata)
    }
}

impl PtrRdata {
    /// Creates a new `PtrRdata` with default values.
    ///
    /// # Examples
    /// ```
    /// let ptr_rdata = PtrRdata::new();
    ///
    /// assert_eq!(ptr_rdata.ptrdname.get_name(), String::from(""));
    /// ```
    pub fn new() -> Self {
        let ptr_rdata = PtrRdata {
            ptrdname: DomainName::new(),
        };

        ptr_rdata
    }

    /// Returns a `ResourceRecord` from the given values.
    /// 
    /// # Examples
    /// ```
    /// let ptr_rdata_rr = PtrRdata::rr_from_master_file(
    /// "dcc".split_whitespace(),
    /// 35,
    /// String::from("IN"), 
    /// String::from("uchile.cl"), 
    /// String::from("uchile.cl"));
    /// 
    ///  assert_eq!(ptr_rdata_rr.get_class(), Rclass::IN);
    ///  assert_eq!(ptr_rdata_rr.get_ttl(), 35);
    ///  assert_eq!(ptr_rdata_rr.get_name().get_name(), String::from("uchile.cl"));
    ///  assert_eq!(ptr_rdata_rr.get_rdlength(), 5);
    ///  assert_eq!(ptr_rdata_rr.get_rtype(), Rtype::PTR);
    /// ```
    pub fn rr_from_master_file(
        mut values: SplitWhitespace,
        ttl: u32,
        class: &str,
        host_name: String,
        origin: String,
    ) -> ResourceRecord {
        let mut ptr_rdata = PtrRdata::new();
        let name = values.next().unwrap();
        let domain_name = DomainName::from_master_file(name.to_string(), origin);

        ptr_rdata.set_ptrdname(domain_name);

        let rdata = Rdata::PTR(ptr_rdata);

        let mut resource_record = ResourceRecord::new(rdata);

        let mut domain_name = DomainName::new();
        domain_name.set_name(host_name);

        resource_record.set_name(domain_name);
        resource_record.set_type_code(Rrtype::PTR);
        let rclass = Rclass::from(class);
        resource_record.set_rclass(rclass);
        resource_record.set_ttl(ttl);
        resource_record.set_rdlength(name.len() as u16 + 2);

        resource_record
    }

    //---------DNSSEC--------
    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        self.get_ptrdname().to_canonical_bytes()
    }
}

// Getters
impl PtrRdata {
    /// Returns a copy of the `ptrdname` attribute from `PtrRdata`.
    pub fn get_ptrdname(&self) -> DomainName {
        self.ptrdname.clone()
    }
}

// Setters
impl PtrRdata {
    /// Sets the `ptrdname` attibute with the given value.
    pub fn set_ptrdname(&mut self, ptrdname: DomainName) {
        self.ptrdname = ptrdname;
    }
}

impl fmt::Display for PtrRdata {
    /// Formats the record data for display
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_ptrdname().get_name())
    }
}

#[cfg(test)]
mod ptr_rdata_test {
    use crate::domain_name::DomainName;
    use crate::message::Rclass;
    use crate::message::rrtype::Rrtype;
    use crate::message::rdata::Rdata;
    use crate::message::rdata::ptr_rdata::PtrRdata;
    use crate::message::resource_record::{FromBytes, ToBytes};

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
        let ptr_rdata = PtrRdata::from_bytes(&bytes_test, &bytes_test).unwrap();

        assert_eq!(
            ptr_rdata.get_ptrdname().get_name(),
            String::from("test.test2.com")
        );
    }

    //ToDo: Revisar
    #[test]
    fn rr_from_master_file(){
        let ptr_rdata_rr = PtrRdata::rr_from_master_file(
            "dcc".split_whitespace(),
            35,
            "IN", 
            String::from("uchile.cl"), 
            String::from("uchile.cl"));

         assert_eq!(ptr_rdata_rr.get_rclass(), Rclass::IN);
         assert_eq!(ptr_rdata_rr.get_ttl(), 35);
         assert_eq!(ptr_rdata_rr.get_name().get_name(), String::from("uchile.cl"));
         assert_eq!(ptr_rdata_rr.get_rdlength(), 5);
         assert_eq!(ptr_rdata_rr.get_rtype(), Rrtype::PTR);
         
         let ptr_rr_rdata = ptr_rdata_rr.get_rdata();
         match ptr_rr_rdata {
            Rdata::PTR(val) => assert_eq!(val.get_ptrdname().get_name(), 
            String::from("dcc.uchile.cl")),
            _ => {}
        }
    }
}
