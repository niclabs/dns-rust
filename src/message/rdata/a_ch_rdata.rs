use crate::domain_name::DomainName;
use crate::message::rdata::Rdata;
use crate::message::resource_record::{FromBytes, ResourceRecord, ToBytes};

use std::str::SplitWhitespace;

#[derive(Clone)]
/// An struct that represents the rdata for a type in CH class
/// For the CH class, a domain name followed
/// by a 16 bit octal Chaos address.
pub struct AChRdata {
    domain_name: DomainName,
    ch_address: u16,
}

impl ToBytes for AChRdata {
    /// Return a vec of bytes that represents the a rdata
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        // Add Domain Name to bytes
        let domain_bytes = self.get_domain_name().to_bytes();

        for byte in domain_bytes.as_slice() {
            bytes.push(*byte);
        }
        //

        // Add ch address to bytes
        let ch_address = self.get_ch_adress();

        bytes.push((ch_address >> 8) as u8);
        bytes.push(ch_address as u8);
        //

        bytes
    }
}

impl FromBytes<Result<Self, &'static str>> for AChRdata {
    /// Creates a new A ch class from an array of bytes
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
    pub fn new() -> Self {
        let a_ch_rdata = AChRdata {
            domain_name: DomainName::new(),
            ch_address: 0,
        };

        a_ch_rdata
    }

    pub fn rr_from_master_file(
        mut values: SplitWhitespace,
        ttl: u32,
        class: u16,
        host_name: String,
        origin: String,
    ) -> ResourceRecord {
        let mut a_ch_rdata = AChRdata::new();

        let name = values.next().unwrap();
        let mut domain_name = DomainName::from_master_file(name.to_string(), origin);

        a_ch_rdata.set_domain_name(domain_name);

        let ch_address = values.next().unwrap();

        a_ch_rdata.set_ch_address(ch_address.parse::<u16>().unwrap());

        let rdata = Rdata::SomeAChRdata(a_ch_rdata);

        let mut resource_record = ResourceRecord::new(rdata);

        let mut domain_name = DomainName::new();
        domain_name.set_name(host_name);

        resource_record.set_name(domain_name);
        resource_record.set_type_code(1);
        resource_record.set_class(class);
        resource_record.set_ttl(ttl);
        resource_record.set_rdlength(name.len() as u16 + 4);

        resource_record
    }
}

// Getters
impl AChRdata {
    pub fn get_domain_name(&self) -> DomainName {
        self.domain_name.clone()
    }

    pub fn get_ch_adress(&self) -> u16 {
        self.ch_address
    }
}

// Setters
impl AChRdata {
    pub fn set_domain_name(&mut self, domain_name: DomainName) {
        self.domain_name = domain_name;
    }

    pub fn set_ch_address(&mut self, ch_address: u16) {
        self.ch_address = ch_address;
    }
}
