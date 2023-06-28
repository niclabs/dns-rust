use crate::domain_name::DomainName;
use crate::message::rdata::Rdata;
use crate::message::Rtype;
use crate::message::Rclass;
use crate::message::resource_record::{FromBytes, ResourceRecord, ToBytes};

use std::str::SplitWhitespace;

#[derive(Clone, PartialEq, Debug)]
/// An struct that represents the RDATA for A TYPE in CH class.
/// 
/// For the CH class, a domain name followed by a 16 bit octal Chaos address.
pub struct AChRdata {
    domain_name: DomainName,
    ch_address: u16,
}

impl ToBytes for AChRdata {
    /// Return a `Vec<u8>` of bytes that represents the A RDATA.
    /// 
    /// # Examples
    /// ```
    /// let bytes: [u8; 4] = [128, 0, 0, 1];
    /// let a_rdata = ARdata::from_bytes(&bytes, &bytes).unwrap();
    /// 
    /// assert_eq!(a_rdata.get_address()[0], 128);
    /// assert_eq!(a_rdata.get_address()[1], 0);
    /// assert_eq!(a_rdata.get_address()[2], 0);
    /// assert_eq!(a_rdata.get_address()[3], 1);
    /// ```
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        // Add Domain Name to bytes
        let domain_bytes = self.get_domain_name().to_bytes();

        for byte in domain_bytes.as_slice() {
            bytes.push(*byte);
        }
        //

        // Add ch address to bytes
        let ch_address = self.get_ch_address();

        bytes.push((ch_address >> 8) as u8);
        bytes.push(ch_address as u8);
        //

        bytes
    }
}

impl FromBytes<Result<Self, &'static str>> for AChRdata {
    /// Creates a new A CH class from an array of bytes.
    /// 
    /// # Examples
    /// ```
    /// let bytes: [u8; 4] = [128, 0, 0, 1];
    /// let a_rdata = ARdata::from_bytes(&bytes, &bytes).unwrap();
    /// 
    /// assert_eq!(a_rdata.get_address()[0], 128);
    /// assert_eq!(a_rdata.get_address()[1], 0);
    /// assert_eq!(a_rdata.get_address()[2], 0);
    /// assert_eq!(a_rdata.get_address()[3], 1);
    /// ```	
    fn from_bytes(bytes: &[u8], full_msg: &[u8]) -> Result<Self, &'static str> {
        let bytes_len = bytes.len();

        if bytes_len < 1 {
            return Err("Format Error");
        }

        // Domain name from bytes
        let domain_result = DomainName::from_bytes(bytes, full_msg);

        match domain_result {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        let (dname, bytes_without_name) = domain_result.unwrap();
        //

        // Checks ip length
        if bytes_without_name.len() < 2 {
            return Err("Format Error");
        }
        //

        // Ch address from bytes
        let ch_address = (bytes_without_name[0] as u16) << 8 | bytes_without_name[1] as u16;
        //

        // Creates A Ch Rdata
        let mut a_ch_rdata = AChRdata::new();

        a_ch_rdata.set_domain_name(dname);
        a_ch_rdata.set_ch_address(ch_address);
        //

        Ok(a_ch_rdata)
    }
}

impl AChRdata {
    /// Creates a new `AChRdata` with default values.
    pub fn new() -> Self {
        let a_ch_rdata = AChRdata {
            domain_name: DomainName::new(),
            ch_address: 0,
        };

        a_ch_rdata
    }

    /// Returns a `ResourceRecord` from the given values.
    /// 
    /// # Examples
    /// ```
    /// let data_bytes = [4, 116, 101, 115, 116, 3, 99, 111, 109, 0, 0, 10];
    /// let ach_rdata = AChRdata::from_bytes(&data_bytes, &data_bytes).unwrap();
    /// 
    /// assert_eq!(ach_rdata.get_domain_name().get_name(), String::from("test.com"));
    /// assert_eq!(ach_rdata.get_ch_address(), 10 as u16);
    /// ```
    pub fn rr_from_master_file(
        mut values: SplitWhitespace,
        ttl: u32,
        class: &str,
        host_name: String,
        origin: String,
    ) -> ResourceRecord {
        let mut a_ch_rdata = AChRdata::new();

        let name = values.next().unwrap();
        let domain_name = DomainName::from_master_file(name.to_string(), origin);

        a_ch_rdata.set_domain_name(domain_name);

        let ch_address = values.next().unwrap();

        a_ch_rdata.set_ch_address(ch_address.parse::<u16>().unwrap());

        let rdata = Rdata::SomeAChRdata(a_ch_rdata);

        let mut resource_record = ResourceRecord::new(rdata);

        let mut domain_name = DomainName::new();
        domain_name.set_name(host_name);

        resource_record.set_name(domain_name);
        resource_record.set_type_code(Rtype::A);
        let rclass = Rclass::from_str_to_rclass(class);
        resource_record.set_class(rclass);
        resource_record.set_ttl(ttl);
        resource_record.set_rdlength(name.len() as u16 + 4);

        resource_record
    }
}

// Getters
impl AChRdata {
    /// Returns a clone of the `domain_name` attribute.
    pub fn get_domain_name(&self) -> DomainName {
        self.domain_name.clone()
    }

    /// Returns a clone of the `ch_address` attribute.
    pub fn get_ch_address(&self) -> u16 {
        self.ch_address
    }
}

// Setters
impl AChRdata {
    /// Sets the `domain_name` attibute with a given `DomainName`.
    pub fn set_domain_name(&mut self, domain_name: DomainName) {
        self.domain_name = domain_name;
    }

    /// Sets the `ch_address` attibute with a given address.
    pub fn set_ch_address(&mut self, ch_address: u16) {
        self.ch_address = ch_address;
    }
}

#[cfg(test)]
mod a_ch_rdata_test {
    use crate::domain_name::DomainName;
    use crate::message::Rtype;
    use crate::message::Rclass;
    use crate::message::rdata::a_ch_rdata::AChRdata;
    use crate::message::rdata::{ARdata, Rdata};
    use crate::message::resource_record::{FromBytes, ToBytes};

    #[test]
    fn constructor_test() {
        let a_rdata = AChRdata::new();
        assert_eq!(a_rdata.domain_name.get_name(), String::from(""));
        assert_eq!(a_rdata.ch_address, 0);
    }

    #[test]
    fn set_and_get_ch_address_test() {
        let mut ach_rdata = AChRdata::new();

        assert_eq!(ach_rdata.get_ch_address(), 0);

        ach_rdata.set_ch_address(1);

        assert_eq!(ach_rdata.get_ch_address(), 1);
    }

    #[test]
    fn to_bytes_test() {
        let mut a_rdata = ARdata::new();

        a_rdata.set_address([127, 0, 0, 1]);

        let a_rdata_to_bytes = a_rdata.to_bytes();

        assert_eq!(a_rdata_to_bytes[0], 127);
        assert_eq!(a_rdata_to_bytes[1], 0);
        assert_eq!(a_rdata_to_bytes[2], 0);
        assert_eq!(a_rdata_to_bytes[3], 1);
    }

    #[test]
    fn from_bytes_test() {
        let bytes: [u8; 4] = [128, 0, 0, 1];
        let a_rdata = ARdata::from_bytes(&bytes, &bytes).unwrap();

        assert_eq!(a_rdata.get_address()[0], 128);
        assert_eq!(a_rdata.get_address()[1], 0);
        assert_eq!(a_rdata.get_address()[2], 0);
        assert_eq!(a_rdata.get_address()[3], 1);
    }

    #[test]
    fn set_and_get_domain_name_test() {
        let mut ach_rdata = AChRdata::new();
        let mut domain_name = DomainName::new();

        assert_eq!(
            ach_rdata.get_domain_name().get_name(),
            domain_name.get_name()
        );

        domain_name.set_name(String::from("test.com"));
        ach_rdata.set_domain_name(domain_name);

        assert_eq!(
            ach_rdata.get_domain_name().get_name(),
            String::from("test.com")
        );
    }
    
    //ToDO: Revisar 
    #[test]
    fn to_bytes(){
        let mut domain_name = DomainName::new();
        let name = String::from("test.com");
        domain_name.set_name(name.clone());

        let mut ach_rdata = AChRdata::new();
        ach_rdata.set_ch_address(10);
        ach_rdata.set_domain_name(domain_name);

        let data_bytes = ach_rdata.to_bytes();

        assert_eq!(data_bytes[0], 4);
        assert_eq!(data_bytes[1], 116);
        assert_eq!(data_bytes[2], 101);
        assert_eq!(data_bytes[3], 115);
        assert_eq!(data_bytes[4], 116);
        assert_eq!(data_bytes[5], 3);
        assert_eq!(data_bytes[6], 99);
        assert_eq!(data_bytes[7], 111);
        assert_eq!(data_bytes[8], 109);
        assert_eq!(data_bytes[9], 0);
        assert_eq!(data_bytes[10], 0);
        assert_eq!(data_bytes[11], 10);
    }

    //ToDo: Revisar
    #[test]
    fn from_bytes(){
        let data_bytes = [4, 116, 101, 115, 116, 3, 99, 111, 109, 0, 0, 10];

        let ach_rdata = AChRdata::from_bytes(&data_bytes, &data_bytes).unwrap();

        assert_eq!(ach_rdata.get_domain_name().get_name(), String::from("test.com"));
        assert_eq!(ach_rdata.get_ch_address(), 10 as u16);
    }

    //ToDo: Revisar 
    #[test]
    #[should_panic]
    fn from_bytes_format_error(){
        let data_bytes: [u8; 0] = [];

        let _ach_rdata = AChRdata::from_bytes(&data_bytes, &data_bytes).unwrap();
    }
    
    //ToDo: Revisar 
    #[test]
    fn rr_from_master_file_test() {
        let ach_rr = AChRdata::rr_from_master_file(
            "204.13.100.3 10".split_whitespace(), 
            0, "CH",
            "admin.googleplex".to_string(), 
            "edu".to_string());

        assert_eq!(ach_rr.get_class(), Rclass::CH);
        assert_eq!(ach_rr.get_name().get_name(), String::from("admin.googleplex"));
        assert_eq!(ach_rr.get_rtype(), Rtype::A);
        assert_eq!(ach_rr.get_ttl(), 0);
        assert_eq!(ach_rr.get_rdlength(), 16);

        let ach_rdata = ach_rr.get_rdata();
        match ach_rdata {
            Rdata::SomeARdata(val) => assert_eq!(val.get_address(), [204, 13, 100, 3]),
            _ => {}
        }
    }
}
