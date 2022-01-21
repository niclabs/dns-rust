use crate::domain_name::DomainName;
use crate::message::rdata::Rdata;
use crate::message::resource_record::{FromBytes, ResourceRecord, ToBytes};
use std::str::SplitWhitespace;

#[derive(Clone)]
/// An struct that represents the rdata for cname type
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                  CNAME                        |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///

pub struct CnameRdata {
    // Specifies the canonical or primary name for the owner. The owner name is an alias.
    cname: DomainName,
}

impl ToBytes for CnameRdata {
    /// Return a vec of bytes that represents the cname rdata
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        let cname_bytes = self.get_cname().to_bytes();

        for byte in cname_bytes.as_slice() {
            bytes.push(*byte);
        }

        bytes
    }
}

impl FromBytes<Result<Self, &'static str>> for CnameRdata {
    /// Creates a new Cname from an array of bytes
    fn from_bytes(bytes: &[u8], full_msg: &[u8]) -> Result<Self, &'static str> {
        let bytes_len = bytes.len();

        if bytes_len < 1 {
            return Err("Format Error");
        }

        let domain_result = DomainName::from_bytes(bytes, full_msg);

        match domain_result {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        let (cname, _) = domain_result.unwrap();

        let mut cname_rdata = CnameRdata::new();

        cname_rdata.set_cname(cname);

        Ok(cname_rdata)
    }
}

impl CnameRdata {
    /// Creates a new CnameRdata with default values.
    ///
    /// # Examples
    /// ```
    /// let cname_rdata = CnameRdata::new();
    ///
    /// ```
    ///

    pub fn new() -> Self {
        let cname_rdata = CnameRdata {
            cname: DomainName::new(),
        };
        cname_rdata
    }

    pub fn rr_from_master_file(
        mut values: SplitWhitespace,
        ttl: u32,
        class: u16,
        host_name: String,
        origin: String,
    ) -> ResourceRecord {
        let mut cname_rdata = CnameRdata::new();

        let name = values.next().unwrap();
        let mut domain_name = DomainName::from_master_file(name.to_string(), origin);

        cname_rdata.set_cname(domain_name);

        let rdata = Rdata::SomeCnameRdata(cname_rdata);

        let mut resource_record = ResourceRecord::new(rdata);

        let mut domain_name = DomainName::new();
        domain_name.set_name(host_name);

        resource_record.set_name(domain_name);
        resource_record.set_type_code(5);
        resource_record.set_class(class);
        resource_record.set_ttl(ttl);
        resource_record.set_rdlength(name.len() as u16 + 2);

        resource_record
    }
}

// Getter
impl CnameRdata {
    // Gets the cname attribute
    pub fn get_cname(&self) -> DomainName {
        self.cname.clone()
    }
}

// Setter
impl CnameRdata {
    // Sets the cname field with a value
    pub fn set_cname(&mut self, cname: DomainName) {
        self.cname = cname;
    }
}

mod test {
    use crate::domain_name::DomainName;
    use crate::message::rdata::cname_rdata::CnameRdata;
    use crate::message::resource_record::{FromBytes, ToBytes};

    #[test]
    fn constructor_test() {
        let cname_rdata = CnameRdata::new();

        assert_eq!(cname_rdata.cname.get_name(), String::from(""));
    }

    #[test]
    fn set_and_get_cname_test() {
        let mut cname_rdata = CnameRdata::new();

        assert_eq!(cname_rdata.get_cname().get_name(), String::from(""));

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test"));
        cname_rdata.set_cname(domain_name);

        assert_eq!(cname_rdata.get_cname().get_name(), String::from("test"));
    }

    #[test]
    fn to_bytes_test() {
        let mut cname_rdata = CnameRdata::new();

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("cname"));
        cname_rdata.set_cname(domain_name);

        let bytes_to_test: [u8; 7] = [5, 99, 110, 97, 109, 101, 0];
        let cname_rdata_to_bytes = cname_rdata.to_bytes();

        for (index, value) in cname_rdata_to_bytes.iter().enumerate() {
            assert_eq!(*value, bytes_to_test[index]);
        }
    }

    #[test]
    fn from_bytes_test() {
        let bytes: [u8; 7] = [5, 99, 110, 97, 109, 101, 0];

        let cname_rdata = CnameRdata::from_bytes(&bytes, &bytes).unwrap();

        assert_eq!(cname_rdata.get_cname().get_name(), String::from("cname"));
    }
}
