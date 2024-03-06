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

impl ToBytes for AAAARdata {
    /// Returns a `Vec<u8>` of bytes that represents the AAAA RDATA.
    fn to_bytes(&self) -> Vec<u8> {
        let address = self.get_address();
        match address {
            IpAddr::V4(_val) => panic!("This is not an IPv6 address"),
            IpAddr::V6(val) => val.octets().to_vec(),
        }
    }
}

impl AAAARdata{
    /// Creates a new `AAAARdata` with default values.
    ///
    /// # Examples
    /// ```
    /// let aaaa_rdata = AAAARdata::new();
    /// assert_eq!(aaaa_rdata.address[0], 0);
    /// ```
    pub fn new() -> AAAARdata {
        let array = [0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16, 0 as u16];
        let ip_address = IpAddr::from(array);
        AAAARdata {
            address: ip_address,
        }
    }

    /// Creates a new `AAAARdata` with a specified address
    /// 
    /// # Examples
    /// ```
    /// let aaaa_rdata = AAAARdata::new_from_addr(IpAddr::from([1,1,1,1,1,1,1,1]));
    /// ```
    pub fn new_from_addr(address: IpAddr) -> AAAARdata {
        AAAARdata {
            address: address,
        }
    }

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