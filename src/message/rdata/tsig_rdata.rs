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