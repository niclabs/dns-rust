use crate::domain_name::DomainName;
use crate::message::rdata::Rdata;
use crate::message::Rtype;
use crate::message::Rclass;
use crate::message::resource_record::{FromBytes, ResourceRecord, ToBytes};
use std::net::IpAddr;

use std::str::SplitWhitespace;

/// Struct for the AAAA Rdata
/// 2.2 AAAA data format
///
/// A 128 bit IPv6 address is encoded in the data portion of an AAAA
/// resource record in network byte order (high-order byte first).

#[derive(Clone, PartialEq, Debug)]
pub struct AAAARdata {
    /// A 128 bit Internet address.
    address: IpAddr,
}
/// Getter for the struct AAAARdata
impl AAAARdata{
    /// Function to get the address of the AAAA Rdata
    pub fn get_address(&self) -> IpAddr{
        self.address
    }
}

/// Setter for the struct AAAARdata
impl AAAARdata{
    /// Function to set the address of the AAAA Rdata
    pub fn set_address(&mut self, address: IpAddr){
        self.address = address;
    }
}