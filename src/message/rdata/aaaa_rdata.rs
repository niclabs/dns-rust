use crate::message::resource_record::{FromBytes, ToBytes};
use std::net::IpAddr;


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

impl FromBytes<Result<Self, &'static str>> for AAAARdata {
    /// Creates a new `AAAARdata` from an array of bytes.
    fn from_bytes(bytes: &[u8], _full_msg: &[u8]) -> Result<Self, &'static str> {
        let bytes_len = bytes.len();

        if bytes_len < 16 {
            return Err("Format Error");
        }

        let mut aaaa_rdata = AAAARdata::new();

        let array_bytes = [bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7], bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]];
        let ip_address = IpAddr::from(array_bytes);

        aaaa_rdata.set_address(ip_address);

        Ok(aaaa_rdata)
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

#[cfg(test)]
mod aaaa_rdata_test{
    use super::*;

    #[test]
    fn constructor_test(){
        let aaaa_rdata = AAAARdata::new();
        let array = [0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(aaaa_rdata.address, IpAddr::from(array));
    }

    #[test]
    fn from_addr_constructor_test(){
        let array = [1,1,1,1,1,1,1,1];
        let aaaa_rdata = AAAARdata::new_from_addr(IpAddr::from(array.clone()));
        assert_eq!(aaaa_rdata.address, IpAddr::from(array));
    }

    #[test]
    fn set_and_get_address_test(){
        let mut aaaa_rdata = AAAARdata::new();

        let array = [1,1,1,1,1,1,1,1];
        assert_eq!(aaaa_rdata.get_address(), IpAddr::from([0, 0, 0, 0, 0, 0, 0, 0]));

        aaaa_rdata.set_address(IpAddr::from(array.clone()));

        assert_eq!(aaaa_rdata.get_address(), IpAddr::from(array));
    }
}
