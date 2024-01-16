use crate::message::resource_record::{FromBytes, ToBytes};
use crate::domain_name::DomainName;
use crate::message::type_rtype::Rtype;

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
    pub type_bit_maps: Vec<Rtype>,
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
                Rtype::UNKNOWN(rr_type) => (rr_type / 256) as u8,
                _ => (Rtype::from_rtype_to_int(rtype) / 256) as u8,
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

        let (next_domain_name, rest_bytes) = domain_result.unwrap();

        let mut decoded_types = Vec::new();
        let mut offset = 0;

        while offset < rest_bytes.len() {
            let window_number = rest_bytes[offset];
            let bitmap_length = rest_bytes[offset + 1] as usize;
            let bitmap = &rest_bytes[offset + 2..offset + 2 + bitmap_length];
            for i in 0..bitmap.len() {
                let byte = bitmap[i];
                for j in 0..8 {
                    let rr_type = window_number as u16 * 256 + i as u16 * 8 + j as u16;
                    let bit_mask = 1 << (7 - j);
                    if byte & bit_mask != 0 {
                        decoded_types.push(Rtype::from_int_to_rtype(rr_type));
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
    pub fn new(next_domain_name: DomainName, type_bit_maps: Vec<Rtype>) -> Self {
        NsecRdata {
            next_domain_name,
            type_bit_maps,
        }
    }

    /// Returns the next_domain_name of the `NsecRdata`.
    /// # Example
    /// ```
    /// let nsec_rdata = NsecRdata::new(DomainName::new_from_str("example.com"), vec![Rtype::A, Rtype::NS]);
    /// assert_eq!(nsec_rdata.get_next_domain_name().get_name(), String::from("www.example.com"));
    /// ```
    pub fn get_next_domain_name(&self) -> DomainName {
        self.next_domain_name.clone()
    }

    /// Returns the type_bit_maps of the `NsecRdata`.
    /// # Example
    /// ```
    /// let nsec_rdata = NsecRdata::new(DomainName::new_from_str("example.com"), vec![Rtype::A, Rtype::NS]);
    /// assert_eq!(nsec_rdata.get_type_bit_maps(), vec![Rtype::A, Rtype::NS]);
    /// ```
    pub fn get_type_bit_maps(&self) -> Vec<Rtype> {
        self.type_bit_maps.clone()
    }
}

impl NsecRdata{
    /// Setters
    
    /// Set the next_domain_name of the `NsecRdata`.
    /// # Example
    /// ```
    /// let mut nsec_rdata = NsecRdata::new(DomainName::new_from_str("example.com"), vec![Rtype::A, Rtype::NS]);
    /// nsec_rdata.set_next_domain_name(DomainName::new_from_str("www.example2.com"));
    /// assert_eq!(nsec_rdata.get_next_domain_name().get_name(), String::from("www.example2.com"));
    /// ```
    pub fn set_next_domain_name(&mut self, next_domain_name: DomainName) {
        self.next_domain_name = next_domain_name;
    }

    /// Set the type_bit_maps of the `NsecRdata`.
    /// # Example   
    /// ```
    /// let mut nsec_rdata = NsecRdata::new(DomainName::new_from_str("example.com"), vec![Rtype::A, Rtype::NS]);
    /// nsec_rdata.set_type_bit_maps(vec![Rtype::A, Rtype::NS, Rtype::CNAME]);
    /// assert_eq!(nsec_rdata.get_type_bit_maps(), vec![Rtype::A, Rtype::NS, Rtype::CNAME]);
    /// ```
    pub fn set_type_bit_maps(&mut self, type_bit_maps: Vec<Rtype>) {
        self.type_bit_maps = type_bit_maps;
    }
}

impl NsecRdata{
    /// Complementary functions for to_bytes
    fn add_rtype_to_bitmap(rtype: &Rtype, bitmap: &mut Vec<u8>) {
        // Calculate the offset and bit for the specific Qtype
        let rr_type = Rtype::from_rtype_to_int(*rtype);
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

#[cfg(test)]
mod nsec_rdata_test{
    use super::*;

    #[test]
    fn constructor_test() {
        let nsec_rdata = NsecRdata::new(DomainName::new(), vec![]);

        assert_eq!(nsec_rdata.next_domain_name.get_name(), String::from(""));
        assert_eq!(nsec_rdata.type_bit_maps, vec![]);
    }

    #[test]
    fn set_and_get_next_domain_name_test() {
        let mut nsec_rdata = NsecRdata::new(DomainName::new(), vec![]);

        assert_eq!(nsec_rdata.get_next_domain_name().get_name(), String::from(""));

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test"));
        nsec_rdata.set_next_domain_name(domain_name);

        assert_eq!(nsec_rdata.get_next_domain_name().get_name(), String::from("test"));
    }

    #[test]
    fn set_and_get_type_bit_maps_test() {
        let mut nsec_rdata = NsecRdata::new(DomainName::new(), vec![]);

        assert_eq!(nsec_rdata.get_type_bit_maps(), vec![]);

        nsec_rdata.set_type_bit_maps(vec![Rtype::A, Rtype::NS]);

        assert_eq!(nsec_rdata.get_type_bit_maps(), vec![Rtype::A, Rtype::NS]);
    }

    #[test]
    fn to_bytes_test() {
        let mut nsec_rdata = NsecRdata::new(DomainName::new(), vec![]);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("host.example.com"));
        nsec_rdata.set_next_domain_name(domain_name);

        nsec_rdata.set_type_bit_maps(vec![Rtype::A, Rtype::MX, Rtype::RRSIG, Rtype::NSEC, Rtype::UNKNOWN(1234)]);

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

        let expected_type_bit_maps = vec![Rtype::A, Rtype::MX, Rtype::RRSIG, Rtype::NSEC, Rtype::UNKNOWN(1234)];

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
}