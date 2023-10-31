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