use crate::domain_name::DomainName;
use crate::message::rdata::Rdata;
use crate::message::Rtype;
use crate::message::Rclass;
use crate::message::resource_record::{FromBytes, ResourceRecord, ToBytes};

/// Struct for the TSIG RData
/// [RFC 2845](https://tools.ietf.org/html/rfc2845#section-3.5)
/// [RFC 8945](https://tools.ietf.org/html/rfc8945#section-3.5)

pub struct TSigRdata {
    algorithm_name: DomainName,
    time_signed: u64,
    fudge: u16,
    mac_size: u16,
    mac: Vec<u8>,
    original_id: u16,
    error: u16,
    other_len: u16,
    other_data: Vec<u8>,
}

impl ToBytes for TSigRdata{
    /// Returns a `Vec<u8>` of bytes that represents the TSig RDATA.
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        let algorithm_name_bytes = self.get_algorithm_name().to_bytes();

        for byte in algorithm_name_bytes.as_slice() {
            bytes.push(*byte);
        }
        
        let time_signed = self.get_time_signed();

        bytes.push((time_signed >> 56) as u8);

        bytes.push((time_signed >> 48) as u8);

        bytes.push((time_signed >> 40) as u8);

        bytes.push((time_signed >> 32) as u8);

        bytes.push((time_signed >> 24) as u8);

        bytes.push((time_signed >> 16) as u8);

        bytes.push((time_signed >> 8) as u8);

        bytes.push(time_signed as u8);
        
        let fudge = self.get_fudge();

        bytes.push((fudge >> 8) as u8);

        bytes.push(fudge as u8);

        let mac_size = self.get_mac_size();

        bytes.push((mac_size >> 8) as u8);

        bytes.push(mac_size as u8);

        let mac = self.get_mac();

        for byte in mac.as_slice() {
            bytes.push(*byte);
        }

        let original_id = self.get_original_id();

        bytes.push((original_id >> 8) as u8);

        bytes.push(original_id as u8);

        let error = self.get_error();

        bytes.push((error >> 8) as u8);

        bytes.push(error as u8);

        let other_len = self.get_other_len();

        bytes.push((other_len >> 8) as u8);

        bytes.push(other_len as u8);    

        let other_data = self.get_other_data();

        for byte in other_data.as_slice() {
            bytes.push(*byte);
        }

        bytes
    }
}

impl TSigRdata {
    /// Creates a new TSigRdata with default values
    ///
    /// # Examples
    /// ```
    /// let tsig_rdata = TSigRdata::new();
    /// ```
    pub fn new() -> TSigRdata {
        TSigRdata {
            algorithm_name: DomainName::new(),
            time_signed: 0,
            fudge: 0,
            mac_size: 0,
            mac: Vec::new(),
            original_id: 0,
            error: 0,
            other_len: 0,
            other_data: Vec::new(),
        }
    }
}

/// Getters
impl TSigRdata {

    /// Gets the algorithm_name attribute from TSigRdata
    fn get_algorithm_name(&self) -> DomainName {
        self.algorithm_name.clone()
    }

    /// Gets the time_signed attribute from TSigRdata
    fn get_time_signed(&self) -> u64 {
        self.time_signed
    }

    /// Gets the fudge attribute from TSigRdata
    fn get_fudge(&self) -> u16 {
        self.fudge
    }
    
    /// Gets the mac_size attribute from TSigRdata
    fn get_mac_size(&self) -> u16 {
        self.mac_size
    }

    /// Gets the mac attribute from TSigRdata
    fn get_mac(&self) -> Vec<u8> {
        self.mac.clone()
    }

    /// Gets the original_id attribute from TSigRdata
    fn get_original_id(&self) -> u16 {
        self.original_id
    }

    /// Gets the error attribute from TSigRdata
    fn get_error(&self) -> u16 {
        self.error
    }

    /// Gets the other_len attribute from TSigRdata
    fn get_other_len(&self) -> u16 {
        self.other_len
    }

    /// Gets the other_data attribute from TSigRdata
    fn get_other_data(&self) -> Vec<u8> {
        self.other_data.clone()
    }
}

/// Setters
impl TSigRdata{

    /// Sets the algorithm_name attibute with a value
    fn set_algorithm_name(&mut self, algorithm_name: DomainName) {
        self.algorithm_name = algorithm_name;
    }

    /// Sets the time_signed attibute with a value
    fn set_time_signed(&mut self, time_signed: u64) {
        self.time_signed = time_signed;
    }

    /// Sets the fudge attibute with a value
    fn set_fudge(&mut self, fudge: u16) {
        self.fudge = fudge;
    }

    /// Sets the mac_size attibute with a value
    fn set_mac_size(&mut self, mac_size: u16) {
        self.mac_size = mac_size;
    }

    /// Sets the mac attibute with a value
    fn set_mac(&mut self, mac: Vec<u8>) {
        self.mac = mac;
    }

    /// Sets the original_id attibute with a value
    fn set_original_id(&mut self, original_id: u16) {
        self.original_id = original_id;
    }

    /// Sets the error attibute with a value
    fn set_error(&mut self, error: u16) {
        self.error = error;
    }

    /// Sets the other_len attibute with a value
    fn set_other_len(&mut self, other_len: u16) {
        self.other_len = other_len;
    }

    /// Sets the other_data attibute with a value
    fn set_other_data(&mut self, other_data: Vec<u8>) {
        self.other_data = other_data;
    }
}