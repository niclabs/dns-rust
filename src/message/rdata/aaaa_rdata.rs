use crate::message::resource_record::{FromBytes, ToBytes, ResourceRecord};
use crate::message::rrtype::Rrtype;
use crate::domain_name::DomainName;
use crate::message::rdata::Rdata;
use crate::message::Rclass;
use std::fmt;
use std::net::IpAddr;
use std::net::Ipv6Addr;
use std::str::SplitWhitespace;

// Define a trait that abstracts setting the address
pub trait SetAddress {
    fn set_address(&self) -> Option<IpAddr>;
}

impl SetAddress for &str {
    fn set_address(&self) -> Option<IpAddr> {
        self.parse::<IpAddr>().ok()
    }
}

impl SetAddress for IpAddr {
    fn set_address(&self) -> Option<IpAddr> {
        Some(*self)
    }
}

/// Struct for the AAAA Rdata
/// 2.2 AAAA data format
///
/// A 128 bit IPv6 address is encoded in the data portion of an AAAA
/// resource record in network byte order (high-order byte first).

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
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

     /// Returns a `ResourceRecord` from the given values.
    /// 
    /// # Examples
    /// ```
    /// let aaaa_rr = AAAARdata::rr_from_master_file(
    ///     "2001:1398:16:4:100::2".split_whitespace(),
    ///     0,
    ///     String::from("IN"),
    ///     "admin1.googleplex.edu".to_string(),
    /// );
    /// 
    /// assert_eq!(aaaa_rr.get_class(), Rclass::IN);
    /// assert_eq!(
    ///     aaaa_rr.get_name().get_name(),
    ///     String::from("admin1.googleplex.edu")
    /// );
    /// assert_eq!(aaaa_rr.get_rtype(), Rrtype::AAAA);
    /// assert_eq!(aaaa_rr.get_rdlength(), 16);
    /// assert_eq!(aaaa_rr.get_ttl(), 0);
    /// let aaaa_rdata = aaaa_rr.get_rdata();
    /// match aaaa_rdata {
    ///     Rdata::AAAA(val) => assert_eq!(val.get_address(), [0x2001, 0x1398, 0x16, 0x4, 0x100, 0x0, 0x0, 0x2]),
    ///     _ => {}
    /// }
    /// ```
    pub fn rr_from_master_file(
        mut values: SplitWhitespace,
        ttl: u32,
        class: &str,
        host_name: String,
    ) -> ResourceRecord {
        let mut aaaa_rdata = AAAARdata::new();
        let mut address: [u16; 8] = [0; 8];
        let str_ip = values.next().unwrap();
        // println!("the str ip is {}", str_ip);
            match str_ip.parse::<Ipv6Addr>() {
        Ok(addr) => {
            address = addr.segments();
            // println!("the ip bytes are {:#x?}", address);
        }
        Err(e) => {
            eprintln!("Failed to parse IP: {}", str_ip);
            std::process::exit(1); // Clean exit with error code
        }
    }
        // println!("the ip bytes are {:#?}", address);
        let ip_address = IpAddr::from(address);
        aaaa_rdata.set_address(ip_address);

        let rdata = Rdata::AAAA(aaaa_rdata);

        let mut resource_record = ResourceRecord::new(rdata);

        let mut domain_name = DomainName::new();
        domain_name.set_name(host_name);

        resource_record.set_name(domain_name);
        resource_record.set_type_code(Rrtype::AAAA);
        let rclass = Rclass::from(class);
        resource_record.set_rclass(rclass);
        resource_record.set_ttl(ttl);
        resource_record.set_rdlength(16);

        resource_record
    }

    /// Returns a `String` that represents the `ARdata`.
    pub fn get_string_address(&self) -> String {
        let ip = self.get_address();

        let mut ip_address = "".to_string();

        let ip_vec = match ip {
            IpAddr::V4(val) => val.octets().to_vec(),
            IpAddr::V6(val) => val.octets().to_vec(),
        };

        for num in ip_vec.iter(){
            ip_address.push_str(num.to_string().as_str());
            ip_address.push_str(".");
        }
        ip_address.pop();

        ip_address
    }

}
/// Getter for the struct AAAARdata
impl AAAARdata{
    /// Function to get the address of the AAAA Rdata
    pub fn get_address(&self) -> IpAddr{
        self.address
    }

    /// Function to get the address of the AAAA Rdata as a string
    pub fn get_address_as_string(&self) -> String{
        let ip = self.get_address();
        match ip {
            IpAddr::V4(_val) => panic!("This is not an IPv6 address"),
            IpAddr::V6(val) => val.to_string(),
        }
    }
}

/// Setter for the struct AAAARdata
impl AAAARdata{
    /// Function to set the address of the AAAA Rdata
    pub fn set_address<T: SetAddress>(&mut self, address: T) {
        if let Some(ip_addr) = address.set_address() {
            self.address = ip_addr;
        } else {
            // Handle the IP address parsing error here
            println!("Error: invalid IP address");
        }
    }
}

impl fmt::Display for AAAARdata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_address_as_string())
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

    #[test]
    fn to_bytes_test(){
        let mut aaaa_rdata = AAAARdata::new();

        let array = [1,1,1,1,1,1,1,1];
        aaaa_rdata.set_address(IpAddr::from(array.clone()));

        let aaaa_rdata_to_bytes = aaaa_rdata.to_bytes();

        for i in 0..16{
            
            if i % 2 == 0 {
                assert_eq!(aaaa_rdata_to_bytes[i], 0);
            }

            else {
                assert_eq!(aaaa_rdata_to_bytes[i], 1);
            }
        }
    }

    #[test]
    fn from_bytes_test(){
        let bytes: [u8; 16] = [0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1];

        let aaaa_rdata = AAAARdata::from_bytes(&bytes, &bytes).unwrap();

        let array = [0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1];
        assert_eq!(aaaa_rdata.get_address(), IpAddr::from(array));
    }

    #[test]
    fn from_bytes_error_test(){
        let bytes: [u8; 15] = [0,1,0,1,0,1,0,1,0,1,0,1,0,1,0];

        let aaaa_rdata = AAAARdata::from_bytes(&bytes, &bytes);

        assert_eq!(aaaa_rdata, Err("Format Error"));
    }

    #[test]
    fn get_address_as_string_test(){
        let mut aaaa_rdata = AAAARdata::new();

        let array = [1,1,1,1,1,1,1,1];
        aaaa_rdata.set_address(IpAddr::from(array.clone()));

        let string_address = aaaa_rdata.get_address_as_string();

        assert_eq!(string_address, "1:1:1:1:1:1:1:1");
    }
}
