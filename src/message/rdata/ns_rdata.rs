use crate::domain_name::DomainName;
use crate::message::Rclass;
use crate::message::rrtype::Rrtype;
use crate::message::rdata::Rdata;
use crate::message::resource_record::{FromBytes, ResourceRecord, ToBytes};
use std::str::SplitWhitespace;
use std::fmt;

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
/// [RFC 1035]: https://datatracker.ietf.org/doc/html/rfc1035#section-3.3.11
/// An struct that represents the `rdata` for NS TYPE.
/// 
/// ```text
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// /                   NSDNAME                     /
/// /                                               /
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
pub struct NsRdata {
    /// A domain name which specifies a host which should be
    /// authoritative for the specified class and domain.
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

impl FromBytes<Result<Self, &'static str>> for NsRdata {
    /// Creates a new NsRdata from an array of bytes
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

        let mut ns_rdata = NsRdata::new();
        let (domain_name, _) = domain_name_result.unwrap();

        ns_rdata.set_nsdname(domain_name);

        Ok(ns_rdata)
    }
}

impl NsRdata {
    /// Creates a new `NsRdata` with default values.
    ///
    /// # Examples
    /// ```
    /// let ns_rdata = NsRdata::new();
    ///
    /// assert_eq!(ns_rdata.nsdname.get_name(), String::from(""));
    /// ```
    pub fn new() -> Self {
        let ns_rdata = NsRdata {
            nsdname: DomainName::new(),
        };

        ns_rdata
    }

    /// Returns a `ResourceRecord` from the given values.
    /// 
    /// # Examples
    /// ```
    /// let nsrdata_rr = NsRdata::rr_from_master_file(
    /// "dcc".split_whitespace(),
    /// 35, 
    /// String::from("IN"), 
    /// String::from("uchile.cl"), 
    /// String::from("uchile.cl"));
    /// 
    /// assert_eq!(nsrdata_rr.get_class(), Rclass::IN);
    /// assert_eq!(nsrdata_rr.get_ttl(), 35);
    /// assert_eq!(nsrdata_rr.get_name().get_name(), String::from("uchile.cl"));
    /// assert_eq!(nsrdata_rr.get_rdlength(), 5);
    /// assert_eq!(nsrdata_rr.get_rtype(), Rrtype::NS);
    /// ```
    pub fn rr_from_master_file(
        mut values: SplitWhitespace,
        ttl: u32,
        class: &str,
        host_name: String,
        origin: String,
    ) -> ResourceRecord {
        let mut ns_rdata = NsRdata::new();
        let name = values.next().unwrap();
        let domain_name = DomainName::from_master_file(name.to_string(), origin);

        ns_rdata.set_nsdname(domain_name);

        let rdata = Rdata::NS(ns_rdata);

        let mut resource_record = ResourceRecord::new(rdata);
        let mut domain_name = DomainName::new();
        domain_name.set_name(host_name);

        resource_record.set_name(domain_name);
        resource_record.set_type_code(Rrtype::NS);
        let rclass = Rclass::from(class);
        resource_record.set_rclass(rclass);
        resource_record.set_ttl(ttl);
        resource_record.set_rdlength(name.len() as u16 + 2);

        resource_record
    }
}

// Getters
impl NsRdata {
    /// Gets the nsdname attribute from NsRdata
    pub fn get_nsdname(&self) -> DomainName {
        self.nsdname.clone()
    }
}

// Setters
impl NsRdata {
    /// Sets the nsdname attibute with a value
    pub fn set_nsdname(&mut self, nsdname: DomainName) {
        self.nsdname = nsdname;
    }
}

impl fmt::Display for NsRdata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_nsdname().get_name())
    }
}

#[cfg(test)]
mod ns_rdata_test {
    use crate::domain_name::DomainName;
    use crate::message::Rclass;
    use crate::message::rrtype::Rrtype;
    use crate::message::rdata::Rdata;
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
        let ns_rdata = NsRdata::from_bytes(&bytes_test, &bytes_test).unwrap();

        assert_eq!(
            ns_rdata.get_nsdname().get_name(),
            String::from("test.test2.com")
        );
    }

    //ToDo: Revisar
    #[test]
    fn rr_from_master_file(){
        let nsrdata_rr = NsRdata::rr_from_master_file(
            "dcc".split_whitespace(),
            35, 
            "IN", 
            String::from("uchile.cl"), 
            String::from("uchile.cl"));

         assert_eq!(nsrdata_rr.get_rclass(), Rclass::IN);
         assert_eq!(nsrdata_rr.get_ttl(), 35);
         assert_eq!(nsrdata_rr.get_name().get_name(), String::from("uchile.cl"));
         assert_eq!(nsrdata_rr.get_rdlength(), 5);
         assert_eq!(nsrdata_rr.get_rtype(), Rrtype::NS);
         
         let ns_rr_rdata = nsrdata_rr.get_rdata();
         match ns_rr_rdata {
            Rdata::NS(val) => assert_eq!(val.get_nsdname().get_name(), 
            String::from("dcc.uchile.cl")),
            _ => {}
        }
    }
}
