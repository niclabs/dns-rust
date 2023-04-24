use crate::domain_name::DomainName;
use crate::message::rdata::Rdata;
use crate::message::resource_record::{FromBytes, ResourceRecord, ToBytes};
use std::str::SplitWhitespace;

#[derive(Clone, PartialEq, Debug)]
// An struct that represents the rdata for mx type
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
// |                  PREFERENCE                   |
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
// /                   EXCHANGE                    /
// /                                               /
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//
pub struct MxRdata {
    preference: u16,
    // A domain name
    exchange: DomainName,
}

impl ToBytes for MxRdata {
    // Return a vec of bytes that represents the mx rdata
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        let first_byte_preference = self.get_first_preference_byte();
        let second_byte_preference = self.get_second_preference_byte();

        let exchange_bytes = self.get_exchange().to_bytes();

        bytes.push(first_byte_preference);
        bytes.push(second_byte_preference);

        for byte in exchange_bytes.as_slice() {
            bytes.push(*byte);
        }

        bytes
    }
}

impl FromBytes<Result<Self, &'static str>> for MxRdata {
    // Creates a new MxRdata from an array of bytes
    fn from_bytes(bytes: &[u8], full_msg: &[u8]) -> Result<Self, &'static str> {
        let bytes_len = bytes.len();

        if bytes_len < 3 {
            return Err("Format Error");
        }

        let preference = (bytes[0] as u16) << 8 | bytes[1] as u16;

        let domain_name_result = DomainName::from_bytes(&bytes[2..], full_msg);

        match domain_name_result {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        let (exchange, _) = domain_name_result.unwrap();

        let mut mx_rdata = MxRdata::new();

        mx_rdata.set_preference(preference);
        mx_rdata.set_exchange(exchange);

        Ok(mx_rdata)
    }
}

impl MxRdata {
    // Creates a new MxRdata with default values.
    //
    // # Examples
    // ```
    // let mx_rdata = MxRdata::new();
    // assert_eq!(mx_rdata.preference, 0);
    // assert_eq!(mx_rdata.exchange, String::from(""));
    // ```
    //
    pub fn new() -> Self {
        let mx_rdata: MxRdata = MxRdata {
            preference: 0 as u16,
            exchange: DomainName::new(),
        };
        mx_rdata
    }

    pub fn rr_from_master_file(
        mut values: SplitWhitespace,
        ttl: u32,
        class: u16,
        host_name: String,
        origin: String,
    ) -> ResourceRecord {
        let mut mx_rdata = MxRdata::new();
        let preference = values.next().unwrap().parse::<u16>().unwrap();
        let name = values.next().unwrap();
        let domain_name = DomainName::from_master_file(name.to_string(), origin);

        mx_rdata.set_exchange(domain_name);
        mx_rdata.set_preference(preference);

        let rdata = Rdata::SomeMxRdata(mx_rdata);

        let mut resource_record = ResourceRecord::new(rdata);
        let mut domain_name = DomainName::new();
        domain_name.set_name(host_name);

        resource_record.set_name(domain_name);
        resource_record.set_type_code(15);
        resource_record.set_class(class);
        resource_record.set_ttl(ttl);
        resource_record.set_rdlength(name.len() as u16 + 4);

        resource_record
    }

    // Gets the first byte from the preference attribute
    pub fn get_first_preference_byte(&self) -> u8 {
        (self.get_preference() >> 8) as u8
    }

    // Gets the second byte from the preference attribute
    pub fn get_second_preference_byte(&self) -> u8 {
        self.get_preference() as u8
    }
}

// Getters
impl MxRdata {
    // Gets the preference attribute from MxRdata
    pub fn get_preference(&self) -> u16 {
        self.preference
    }

    // Gets the exchange attribute from MxRdata
    pub fn get_exchange(&self) -> DomainName {
        self.exchange.clone()
    }
}

// Setters
impl MxRdata {
    // Sets the preference attibute with a value
    pub fn set_preference(&mut self, preference: u16) {
        self.preference = preference;
    }

    // Sets the exchange attibute with a value
    pub fn set_exchange(&mut self, exchange: DomainName) {
        self.exchange = exchange;
    }
}

#[cfg(test)]
mod mx_rdata_test {
    use crate::domain_name::DomainName;
    use crate::message::rdata::Rdata;
    use crate::message::rdata::mx_rdata::MxRdata;
    use crate::message::resource_record::{FromBytes, ToBytes};

    #[test]
    fn constructor_test() {
        let mx_rdata = MxRdata::new();

        assert_eq!(mx_rdata.preference, 0);
        assert_eq!(mx_rdata.exchange.get_name(), String::from(""));
    }

    #[test]
    fn set_and_get_preference_test() {
        let mut mx_rdata = MxRdata::new();

        assert_eq!(mx_rdata.get_preference(), 0);

        mx_rdata.set_preference(16);

        assert_eq!(mx_rdata.get_preference(), 16);
    }

    #[test]
    fn set_and_get_exchange_test() {
        let mut mx_rdata = MxRdata::new();

        assert_eq!(mx_rdata.get_exchange().get_name(), String::from(""));

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test"));

        mx_rdata.set_exchange(domain_name);

        assert_eq!(mx_rdata.get_exchange().get_name(), String::from("test"));
    }

    #[test]
    fn to_bytes_test() {
        let mut mx_rdata = MxRdata::new();

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test.com"));

        mx_rdata.set_exchange(domain_name);
        mx_rdata.set_preference(128);

        let bytes_to_test: [u8; 12] = [0, 128, 4, 116, 101, 115, 116, 3, 99, 111, 109, 0];
        let mx_rdata_to_bytes = mx_rdata.to_bytes();

        for (index, value) in mx_rdata_to_bytes.iter().enumerate() {
            assert_eq!(*value, bytes_to_test[index]);
        }
    }

    #[test]
    fn from_bytes_test() {
        let bytes: [u8; 12] = [0, 128, 4, 116, 101, 115, 116, 3, 99, 111, 109, 0];

        let mx_rdata = MxRdata::from_bytes(&bytes, &bytes).unwrap();

        assert_eq!(mx_rdata.get_preference(), 128);
        assert_eq!(mx_rdata.get_exchange().get_name(), String::from("test.com"));
    }

    //ToDo: Revisar
    #[test]
    fn rr_from_master_file_test(){
        let mxrdata_rr = MxRdata::rr_from_master_file("3 dcc".split_whitespace(), 
        20, 1, 
        String::from("uchile.cl"), 
        String::from("uchile.cl"));

        assert_eq!(mxrdata_rr.get_class(), 1);
        assert_eq!(mxrdata_rr.get_ttl(), 20);
        assert_eq!(mxrdata_rr.get_name().get_name(), String::from("uchile.cl"));
        assert_eq!(mxrdata_rr.get_rdlength(), 7);
        
        let mx_rr_rdata = mxrdata_rr.get_rdata();
        match mx_rr_rdata {
            Rdata::SomeMxRdata(val) => assert_eq!((val.get_exchange().get_name(), val.get_preference()), 
            (String::from("dcc.uchile.cl"), 3)),
            _ => {}
        }
    }
}
