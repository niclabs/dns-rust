use crate::message::resource_record::{FromBytes, ToBytes};
use crate::domain_name::DomainName;
use crate::message::rrtype::Rrtype;

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
/// Struct for NSEC Rdata
/// [RFC 4034](https://tools.ietf.org/html/rfc4034#section-4.1)
///                        1 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 3 3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// /                      Next Domain Name                         /
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// /                       Type Bit Maps                           /
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

pub struct NsecRdata {
    pub next_domain_name: DomainName,
    pub type_bit_maps: Vec<Rrtype>,
}

impl ToBytes for NsecRdata{
    /// Returns a `Vec<u8>` of bytes that represents the NSEC RDATA.
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        let next_domain_name_bytes = self.get_next_domain_name().to_bytes();

        for byte in next_domain_name_bytes.as_slice() {
            bytes.push(*byte);
        }

        let bitmap = self.get_type_bit_maps();

        let mut encoded_types = Vec::new();
        let mut current_window: Option<u8> = None;
        let mut current_bitmap = Vec::new();

        for rtype in bitmap {
            let window = match rtype {
                Rrtype::UNKNOWN(rr_type) => (rr_type / 256) as u8,
                _ => (u16::from(rtype) / 256) as u8,
            };

            if let Some(current_window_value) = current_window {
                if current_window_value == window {
                    // We're still in the same window, continue adding to the current bitmap
                    NsecRdata::add_rtype_to_bitmap(&rtype, &mut current_bitmap);
                    continue;
                } else {
                    // New window encountered, write the previous window's data
                    encoded_types.push(current_window_value);
                    encoded_types.push(current_bitmap.len() as u8);
                    encoded_types.extend_from_slice(&current_bitmap);
                }
            }

            // Start a new window
            current_window = Some(window);
            current_bitmap.clear();
            NsecRdata::add_rtype_to_bitmap(&rtype, &mut current_bitmap);
        }

        // Write the final window information
        if let Some(current_window_value) = current_window {
            encoded_types.push(current_window_value);
            encoded_types.push(current_bitmap.len() as u8);
            encoded_types.extend_from_slice(&current_bitmap);
        }

        bytes.extend_from_slice(&encoded_types);

        bytes
    }
}

impl FromBytes<Result<Self, &'static str>> for NsecRdata {
    /// Reads the next_domain_name and type_bit_maps from the slice and returns a `NsecRdata` struct.
    
    fn from_bytes(bytes: &[u8], full_msg: &[u8]) -> Result<Self, &'static str> {
        let bytes_len = bytes.len();
        
        if bytes_len < 5 {
            return Err("Format Error");
        }

        let domain_result = DomainName::from_bytes(bytes, full_msg);

        match domain_result {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        let (mut next_domain_name, rest_bytes) = domain_result.unwrap();

        if next_domain_name.get_name() == ""{
            next_domain_name.set_name(String::from("."));
        }

        let mut decoded_types = Vec::new();
        let mut offset = 0;

        while offset < rest_bytes.len() {
            let window_number = rest_bytes[offset];
            let bitmap_length: usize = rest_bytes[offset + 1] as usize;
            //check if the bitmap_lenght is in the range [0,32]
            if bitmap_length > 32 {
                println!("The bitmap lenght is {}", bitmap_length);
                return Err("Some bitmap_lenght is greather than 32");
            }
            let bitmap = &rest_bytes[offset + 2..offset + 2 + bitmap_length];
            for i in 0..bitmap.len() {
                let byte = bitmap[i];
                for j in 0..8 {
                    let rr_type = window_number as u16 * 256 + i as u16 * 8 + j as u16;
                    let bit_mask = 1 << (7 - j);
                    if byte & bit_mask != 0 {
                        decoded_types.push(Rrtype::from(rr_type));
                    }
                }
            }
            // Move the offset to the next window block
            offset += 2 + bitmap_length;
        }

        let nsec_rdata = NsecRdata::new(next_domain_name, decoded_types);

        Ok(nsec_rdata)
    }
}

impl NsecRdata{
    /// Creates a new `NsecRdata` with next_domain_name and type_bit_maps
    pub fn new(next_domain_name: DomainName, type_bit_maps: Vec<Rrtype>) -> Self {
        if next_domain_name.get_name() == ""{
            panic!("The next_domain_name can't be empty");
        }
        NsecRdata {
            next_domain_name,
            type_bit_maps,
        }
    }

    /// Returns the next_domain_name of the `NsecRdata`.
    /// # Example
    /// ```
    /// let nsec_rdata = NsecRdata::new(DomainName::new_from_str("example.com"), vec![Rrtype::A, Rrtype::NS]);
    /// assert_eq!(nsec_rdata.get_next_domain_name().get_name(), String::from("www.example.com"));
    /// ```
    pub fn get_next_domain_name(&self) -> DomainName {
        self.next_domain_name.clone()
    }

    /// Returns the type_bit_maps of the `NsecRdata`.
    /// # Example
    /// ```
    /// let nsec_rdata = NsecRdata::new(DomainName::new_from_str("example.com"), vec![Rrtype::A, Rrtype::NS]);
    /// assert_eq!(nsec_rdata.get_type_bit_maps(), vec![Rrtype::A, Rrtype::NS]);
    /// ```
    pub fn get_type_bit_maps(&self) -> Vec<Rrtype> {
        self.type_bit_maps.clone()
    }
}

impl NsecRdata{
    /// Setters
    
    /// Set the next_domain_name of the `NsecRdata`.
    /// # Example
    /// ```
    /// let mut nsec_rdata = NsecRdata::new(DomainName::new_from_str("example.com"), vec![Rrtype::A, Rrtype::NS]);
    /// nsec_rdata.set_next_domain_name(DomainName::new_from_str("www.example2.com"));
    /// assert_eq!(nsec_rdata.get_next_domain_name().get_name(), String::from("www.example2.com"));
    /// ```
    pub fn set_next_domain_name(&mut self, next_domain_name: DomainName) {
        self.next_domain_name = next_domain_name;
    }

    /// Set the type_bit_maps of the `NsecRdata`.
    /// # Example   
    /// ```
    /// let mut nsec_rdata = NsecRdata::new(DomainName::new_from_str("example.com"), vec![Rrtype::A, Rrtype::NS]);
    /// nsec_rdata.set_type_bit_maps(vec![Rrtype::A, Rrtype::NS, Rrtype::CNAME]);
    /// assert_eq!(nsec_rdata.get_type_bit_maps(), vec![Rrtype::A, Rrtype::NS, Rrtype::CNAME]);
    /// ```
    pub fn set_type_bit_maps(&mut self, type_bit_maps: Vec<Rrtype>) {
        self.type_bit_maps = type_bit_maps;
    }
}

impl NsecRdata{
    /// Complementary functions for to_bytes
    pub fn add_rtype_to_bitmap(rtype: &Rrtype, bitmap: &mut Vec<u8>) {
        // Calculate the offset and bit for the specific Qtype
        let rr_type = u16::from(*rtype);
        let offset = (rr_type % 256) / 8;
        let bit = 7 - (rr_type % 8);
    
        // Ensure the bitmap has enough space
        if offset >= bitmap.len() as u16 {
            bitmap.resize((offset + 1) as usize, 0);
        }
    
        // Set the bit in the bitmap
        bitmap[offset as usize] |= 1 << bit;
    }
}

impl fmt::Display for NsecRdata {
    /// Formats the NSEC Rdata for display
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.get_next_domain_name().get_name(), self.get_type_bit_maps())
    }
}

#[cfg(test)]
mod nsec_rdata_test{
    use super::*;

    #[test]
    fn constructor_test() {
        let nsec_rdata = NsecRdata::new(DomainName::new_from_str("."), vec![]);

        assert_eq!(nsec_rdata.next_domain_name.get_name(), String::from("."));
        assert_eq!(nsec_rdata.type_bit_maps, vec![]);
    }

    #[test]
    fn set_and_get_next_domain_name_test() {
        let mut nsec_rdata = NsecRdata::new(DomainName::new_from_str("."), vec![]);

        assert_eq!(nsec_rdata.get_next_domain_name().get_name(), String::from("."));

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test"));
        nsec_rdata.set_next_domain_name(domain_name);

        assert_eq!(nsec_rdata.get_next_domain_name().get_name(), String::from("test"));
    }

    #[test]
    fn set_and_get_type_bit_maps_test() {
        let mut nsec_rdata = NsecRdata::new(DomainName::new_from_str("."), vec![]);

        assert_eq!(nsec_rdata.get_type_bit_maps(), vec![]);

        nsec_rdata.set_type_bit_maps(vec![Rrtype::A, Rrtype::NS]);

        assert_eq!(nsec_rdata.get_type_bit_maps(), vec![Rrtype::A, Rrtype::NS]);
    }

    #[test]
    fn to_bytes_test() {
        let mut nsec_rdata = NsecRdata::new(DomainName::new_from_str("."), vec![]);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("host.example.com"));
        nsec_rdata.set_next_domain_name(domain_name);

        nsec_rdata.set_type_bit_maps(vec![Rrtype::A, Rrtype::MX, Rrtype::RRSIG, Rrtype::NSEC, Rrtype::UNKNOWN(1234)]);

        let next_domain_name_bytes = vec![4, 104, 111, 115, 116, 7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109, 0];

        let bit_map_bytes_to_test = vec![0, 6, 64, 1, 0, 0, 0, 3, 
                                    4, 27, 0, 0, 0, 0, 0, 0, 0,
                                    0, 0, 0, 0, 0, 0, 0, 0, 0, 
                                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32];

        let bytes_to_test = [next_domain_name_bytes, bit_map_bytes_to_test].concat();

        assert_eq!(nsec_rdata.to_bytes(), bytes_to_test);
    }

    #[test]
    fn from_bytes_test(){
        let next_domain_name_bytes = vec![4, 104, 111, 115, 116, 7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109, 0];

        let bit_map_bytes_to_test = vec![0, 6, 64, 1, 0, 0, 0, 3, 
                                    4, 27, 0, 0, 0, 0, 0, 0, 0,
                                    0, 0, 0, 0, 0, 0, 0, 0, 0, 
                                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32];

        let bytes_to_test = [next_domain_name_bytes, bit_map_bytes_to_test].concat();

        let nsec_rdata = NsecRdata::from_bytes(&bytes_to_test, &bytes_to_test).unwrap();

        let expected_next_domain_name = String::from("host.example.com");

        assert_eq!(nsec_rdata.get_next_domain_name().get_name(), expected_next_domain_name);

        let expected_type_bit_maps = vec![Rrtype::A, Rrtype::MX, Rrtype::RRSIG, Rrtype::NSEC, Rrtype::UNKNOWN(1234)];

        assert_eq!(nsec_rdata.get_type_bit_maps(), expected_type_bit_maps);
    }

    #[test]
    fn from_bytes_error_test(){
        let error_bytes = vec![0, 6, 64, 1];

        let result = NsecRdata::from_bytes(&error_bytes, &error_bytes);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn from_bytes_empty_bit_map(){
        let bytes_to_test = vec![4, 104, 111, 115, 116, 7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109, 0];

        let nsec_rdata = NsecRdata::from_bytes(&bytes_to_test, &bytes_to_test).unwrap();

        let expected_next_domain_name = String::from("host.example.com");

        assert_eq!(nsec_rdata.get_next_domain_name().get_name(), expected_next_domain_name);

        let expected_type_bit_maps = Vec::new();

        assert_eq!(nsec_rdata.get_type_bit_maps(), expected_type_bit_maps);
    }

    #[test]
    fn to_bytes_empty_bit_map(){
        let mut nsec_rdata = NsecRdata::new(DomainName::new_from_str("."), vec![]);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("host.example.com"));
        nsec_rdata.set_next_domain_name(domain_name);

        let expected_next_domain_name = String::from("host.example.com");

        assert_eq!(nsec_rdata.get_next_domain_name().get_name(), expected_next_domain_name);

        let expected_type_bit_maps = Vec::new();

        assert_eq!(nsec_rdata.get_type_bit_maps(), expected_type_bit_maps);

        let bytes_to_test = vec![4, 104, 111, 115, 116, 7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109, 0];

        assert_eq!(nsec_rdata.to_bytes(), bytes_to_test);
    }

    #[test]
    fn from_bytes_max_value_unknown(){
        let next_domain_name_bytes = vec![4, 104, 111, 115, 116, 7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109, 0];

        //this shoud represent the 65535 value, because the windown number is 255 -> start in
        // 255*Number_bytes_for_windon*8 - 1 = 255*32*8 - 1 = 65279, need 256: 32*8 = 256, so 
        // 31 zeros and 00000001 = 1 
        let bit_map_bytes_to_test = vec![255, 32,
        0, 0, 0, 0, 0, 0, 0, 0, // 8
        0, 0, 0, 0, 0, 0, 0, 0, //16
        0, 0, 0, 0, 0, 0, 0, 0, //24
        0, 0, 0, 0, 0, 0, 0, 1]; //32

        let bytes_to_test = [next_domain_name_bytes, bit_map_bytes_to_test].concat();

        let nsec_rdata = NsecRdata::from_bytes(&bytes_to_test, &bytes_to_test).unwrap();

        let expected_next_domain_name = String::from("host.example.com");

        assert_eq!(nsec_rdata.get_next_domain_name().get_name(), expected_next_domain_name);

        let expected_type_bit_maps = vec![Rrtype::UNKNOWN(65535)];

        assert_eq!(nsec_rdata.get_type_bit_maps(), expected_type_bit_maps);

    }

    
    #[test]
    fn to_bytes_max_value_unknown(){
        let mut nsec_rdata = NsecRdata::new(DomainName::new_from_str("."), vec![]);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("host.example.com"));
        nsec_rdata.set_next_domain_name(domain_name);

        let expected_next_domain_name = String::from("host.example.com");

        assert_eq!(nsec_rdata.get_next_domain_name().get_name(), expected_next_domain_name);

        let expected_type_bit_maps = vec![Rrtype::UNKNOWN(65535)];

        nsec_rdata.set_type_bit_maps(expected_type_bit_maps.clone());

        assert_eq!(nsec_rdata.get_type_bit_maps(), expected_type_bit_maps);

        let next_domain_name_bytes = vec![4, 104, 111, 115, 116, 7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109, 0];

        let bit_map_bytes_to_test = vec![255, 32,
        0, 0, 0, 0, 0, 0, 0, 0, // 8
        0, 0, 0, 0, 0, 0, 0, 0, //16
        0, 0, 0, 0, 0, 0, 0, 0, //24
        0, 0, 0, 0, 0, 0, 0, 1]; //32

        let bytes_to_test = [next_domain_name_bytes, bit_map_bytes_to_test].concat();

        assert_eq!(nsec_rdata.to_bytes(), bytes_to_test);
    }

    #[test]
    fn from_bytes_all_standar_rtypes(){
        let next_domain_name_bytes = vec![4, 104, 111, 115, 116, 7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109, 0];

        //this shoud represent all the Rrtypes except the UNKOWNS(value), the first windown (windown 0) only is necessary, 
        let bit_map_bytes_to_test = vec![0, 32,
        102, 31, 128, 0, 1, 83, 128, 0, // 102 <-> 01100110 <-> (1, 2, 5, 6) <-> (A, NS, CNAME, SOA) and so on
        0, 0, 0, 0, 0, 0, 0, 0, //16
        0, 0, 0, 0, 0, 0, 0, 0, //24
        0, 0, 0, 0, 0, 0, 0, 32]; //31

        let bytes_to_test = [next_domain_name_bytes, bit_map_bytes_to_test].concat();

        let nsec_rdata = NsecRdata::from_bytes(&bytes_to_test, &bytes_to_test).unwrap();

        let expected_next_domain_name = String::from("host.example.com");

        assert_eq!(nsec_rdata.get_next_domain_name().get_name(), expected_next_domain_name);

        let expected_type_bit_maps = vec![Rrtype::A, Rrtype::NS, Rrtype::CNAME,Rrtype::SOA, Rrtype::WKS, Rrtype::PTR, Rrtype::HINFO, Rrtype::MINFO,
        Rrtype::MX, Rrtype::TXT, Rrtype::DNAME, Rrtype::OPT, Rrtype::DS, Rrtype::RRSIG, Rrtype::NSEC, Rrtype::DNSKEY, Rrtype::TSIG];

        assert_eq!(nsec_rdata.get_type_bit_maps(), expected_type_bit_maps);
    }

    #[test]
    fn to_bytes_all_standar_rtypes() {
        let mut nsec_rdata = NsecRdata::new(DomainName::new_from_str("."), vec![]);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("host.example.com"));
        nsec_rdata.set_next_domain_name(domain_name);

        nsec_rdata.set_type_bit_maps(vec![Rrtype::A, Rrtype::NS, Rrtype::CNAME,Rrtype::SOA, Rrtype::WKS, Rrtype::PTR, Rrtype::HINFO, Rrtype::MINFO,
            Rrtype::MX, Rrtype::TXT, Rrtype::DNAME, Rrtype::OPT, Rrtype::DS, Rrtype::RRSIG, Rrtype::NSEC, Rrtype::DNSKEY, Rrtype::TSIG]);

        let next_domain_name_bytes = vec![4, 104, 111, 115, 116, 7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109, 0];

        let bit_map_bytes_to_test = vec![0, 32,
        102, 31, 128, 0, 1, 83, 128, 0, // 102 <-> 01100110 <-> (1, 2, 5, 6) <-> (A, NS, CNAME, SOA) and so on
        0, 0, 0, 0, 0, 0, 0, 0, //16
        0, 0, 0, 0, 0, 0, 0, 0, //24
        0, 0, 0, 0, 0, 0, 0, 32]; //31;

        let bytes_to_test = [next_domain_name_bytes, bit_map_bytes_to_test].concat();

        assert_eq!(nsec_rdata.to_bytes(), bytes_to_test);
    }

    #[test]
    #[should_panic]
    fn from_bytes_wrong_map_lenght(){
        let next_domain_name_bytes = vec![4, 104, 111, 115, 116, 7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109, 0];

        //this shoud represent all the Rrtypes except the UNKOWNS(value), the first windown (windown 0) only is necessary, 
        let bit_map_bytes_to_test = vec![0, 33,
        102, 31, 128, 0, 1, 83, 128, 0, // 102 <-> 01100110 <-> (1, 2, 5, 6) <-> (A, NS, CNAME, SOA) and so on
        0, 0, 0, 0, 0, 0, 0, 0, //16
        0, 0, 0, 0, 0, 0, 0, 0, //24
        0, 0, 0, 0, 0, 0, 0, 32, 1]; //33

        let bytes_to_test = [next_domain_name_bytes, bit_map_bytes_to_test].concat();

        if let Err(error)  = NsecRdata::from_bytes(&bytes_to_test, &bytes_to_test){
            panic!("{}", error);
        }
        else{
            assert!(false, "The map length is greater than 32, must have thrown an error");
        }
    }

    #[test]
    fn to_bytes_root_domain() {
        let mut nsec_rdata = NsecRdata::new(DomainName::new_from_str("."), vec![]);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("."));
        nsec_rdata.set_next_domain_name(domain_name);

        nsec_rdata.set_type_bit_maps(vec![Rrtype::A, Rrtype::MX, Rrtype::RRSIG, Rrtype::NSEC, Rrtype::UNKNOWN(1234)]);

        let next_domain_name_bytes = vec![0];

        let bit_map_bytes_to_test = vec![0, 6, 64, 1, 0, 0, 0, 3, 
                                    4, 27, 0, 0, 0, 0, 0, 0, 0,
                                    0, 0, 0, 0, 0, 0, 0, 0, 0, 
                                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32];

        let bytes_to_test = [next_domain_name_bytes, bit_map_bytes_to_test].concat();

        assert_eq!(nsec_rdata.to_bytes(), bytes_to_test);
    }
    
    #[test]
    fn from_bytes_root_domain() {
        let next_domain_name_bytes = vec![0]; //codification for domain name = ""

        let bit_map_bytes_to_test = vec![0, 6, 64, 1, 0, 0, 0, 3, 
                                    4, 27, 0, 0, 0, 0, 0, 0, 0,
                                    0, 0, 0, 0, 0, 0, 0, 0, 0, 
                                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32];

        let bytes_to_test = [next_domain_name_bytes, bit_map_bytes_to_test].concat();

        let nsec_rdata = NsecRdata::from_bytes(&bytes_to_test, &bytes_to_test).unwrap();
        
        let expected_next_domain_name = String::from(".");
        
         assert_eq!(nsec_rdata.get_next_domain_name().get_name(), expected_next_domain_name);

        let expected_type_bit_maps = vec![Rrtype::A, Rrtype::MX, Rrtype::RRSIG, Rrtype::NSEC, Rrtype::UNKNOWN(1234)];

        assert_eq!(nsec_rdata.get_type_bit_maps(), expected_type_bit_maps);
    }
}