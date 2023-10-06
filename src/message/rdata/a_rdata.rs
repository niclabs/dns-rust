use crate::domain_name::DomainName;
use crate::message::rdata::Rdata;
use crate::message::Rclass;
use crate::message::Rtype;
use std::net::IpAddr;
use crate::message::resource_record::{FromBytes, ResourceRecord, ToBytes};

use std::str::SplitWhitespace;

#[derive(Clone, PartialEq, Debug)]
/// An struct that represents the `Rdata` for a type.
/// 
/// ```text
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    ADDRESS                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
pub struct ARdata {
    /// A 32 bit Internet address.
    address: IpAddr,
}

impl ToBytes for ARdata {
    /// Returns a `Vec<u8>` of bytes that represents the A RDATA.
    fn to_bytes(&self) -> Vec<u8> {
        let address = self.get_address();
        match address {
            IpAddr::V4(val) => val.octets().to_vec(),
            IpAddr::V6(val) => val.octets().to_vec(),
        }
    }
}

impl FromBytes<Result<Self, &'static str>> for ARdata {
    /// Creates a new `ARdata` from an array of bytes.
    fn from_bytes(bytes: &[u8], _full_msg: &[u8]) -> Result<Self, &'static str> {
        let bytes_len = bytes.len();

        if bytes_len < 4 {
            return Err("Format Error");
        }

        let mut a_rdata = ARdata::new();

        let array_bytes = [bytes[0], bytes[1], bytes[2], bytes[3]];
        let ip_address = IpAddr::from(array_bytes);

        a_rdata.set_address(ip_address);

        Ok(a_rdata)
    }
}

impl ARdata {
    /// Creates a new `ARdata` with default values.
    ///
    /// # Examples
    /// ```
    /// let a_rdata = ARdata::new();
    /// assert_eq!(a_rdata.address[0], 0);
    /// assert_eq!(a_rdata.address[1], 0);
    /// assert_eq!(a_rdata.address[2], 0);
    /// assert_eq!(a_rdata.address[3], 0);
    /// ```
    pub fn new() -> Self {
        let array = [0 as u8, 0 as u8, 0 as u8, 0 as u8];
        let ip_address = IpAddr::from(array);
        let a_rdata = ARdata {
            address: ip_address,
        };

        a_rdata
    }
    /// Returns a `ResourceRecord` from the given values.
    /// 
    /// # Examples
    /// ```
    /// let a_rr = ARdata::rr_from_master_file(
    ///     "204.13.100.3".split_whitespace(),
    ///     0,
    ///     String::from("IN"),
    ///     "admin1.googleplex.edu".to_string(),
    /// );
    /// 
    /// assert_eq!(a_rr.get_class(), Rclass::IN);
    /// assert_eq!(
    ///     a_rr.get_name().get_name(),
    ///     String::from("admin1.googleplex.edu")
    /// );
    /// assert_eq!(a_rr.get_rtype(), Rtype::A);
    /// assert_eq!(a_rr.get_ttl(), 0);
    /// assert_eq!(a_rr.get_rdlength(), 4);
    /// let a_rdata = a_rr.get_rdata();
    /// match a_rdata {
    ///     Rdata::SomeARdata(val) => assert_eq!(val.get_address(), [204, 13, 100, 3]),
    ///     _ => {}
    /// }
    /// ```
    pub fn rr_from_master_file(
        mut values: SplitWhitespace,
        ttl: u32,
        class: &str,
        host_name: String,
    ) -> ResourceRecord {
        let mut a_rdata = ARdata::new();
        let mut address: [u8; 4] = [0; 4];
        let str_ip = values.next().unwrap();
        let bytes_str: Vec<&str> = str_ip.split(".").collect();
        let mut index = 0;

        for byte in bytes_str {
            let numb_byte = byte.parse::<u8>().unwrap();
            address[index] = numb_byte;
            index = index + 1;
        }
        let ip_address = IpAddr::from(address);
        a_rdata.set_address(ip_address);

        let rdata = Rdata::SomeARdata(a_rdata);

        let mut resource_record = ResourceRecord::new(rdata);

        let mut domain_name = DomainName::new();
        domain_name.set_name(host_name);

        resource_record.set_name(domain_name);
        resource_record.set_type_code(Rtype::A);
        let rclass = Rclass::from_str_to_rclass(class);
        resource_record.set_rclass(rclass);
        resource_record.set_ttl(ttl);
        resource_record.set_rdlength(4);

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

// Getters
impl ARdata {
    /// Gets the `address` attribute from ARdata.
    pub fn get_address(&self) -> IpAddr {
        self.address
    }
}

// Setters
impl ARdata {
    /// Sets the `address` attibute with the given value.
    pub fn set_address(&mut self, address: IpAddr) {
        self.address = address;
    }
}

#[cfg(test)]
mod a_rdata_test {
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::Rdata;
    use crate::message::Rclass;
    use crate::message::Rtype;
    use std::net::IpAddr;
    use crate::message::resource_record::{FromBytes, ToBytes};

    #[test]
    fn constructor_test() {
        let a_rdata = ARdata::new();
        assert_eq!(a_rdata.address, IpAddr::from([0, 0, 0, 0]));
    }

    #[test]
    fn set_and_get_address_test() {
        let mut a_rdata = ARdata::new();

        assert_eq!(a_rdata.get_address(), IpAddr::from([0, 0, 0, 0]));

        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));

        assert_eq!(a_rdata.get_address(), IpAddr::from([127, 0, 0, 1]));
    }

    #[test]
    fn to_bytes_test() {
        let mut a_rdata = ARdata::new();

        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));

        let a_rdata_to_bytes = a_rdata.to_bytes();

        assert_eq!(a_rdata_to_bytes[0], 127);
        assert_eq!(a_rdata_to_bytes[1], 0);
        assert_eq!(a_rdata_to_bytes[2], 0);
        assert_eq!(a_rdata_to_bytes[3], 1);
    }

    #[test]
    fn from_bytes_test() {
        let bytes: [u8; 4] = [128, 0, 0, 1];
        let a_rdata = ARdata::from_bytes(&bytes, &bytes).unwrap();

        assert_eq!(a_rdata.get_address(), IpAddr::from([128, 0, 0, 1]));
    }

    #[test]
    fn get_string_address_test() {
        let mut a_rdata = ARdata::new();

        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));

        assert_eq!(a_rdata.get_string_address(), "127.0.0.1");
    }

    #[test]
    fn rr_from_master_file_test() {
        let a_rr = ARdata::rr_from_master_file(
            "204.13.100.3".split_whitespace(),
            0,
            "IN",
            "admin1.googleplex.edu".to_string(),
        );

        assert_eq!(a_rr.get_rclass(), Rclass::IN);
        assert_eq!(
            a_rr.get_name().get_name(),
            String::from("admin1.googleplex.edu")
        );
        assert_eq!(a_rr.get_rtype(), Rtype::A);
        assert_eq!(a_rr.get_ttl(), 0);
        assert_eq!(a_rr.get_rdlength(), 4);

        let a_rdata = a_rr.get_rdata();
        match a_rdata {
            Rdata::SomeARdata(val) => assert_eq!(val.get_address(), IpAddr::from([204, 13, 100, 3])),
            _ => {}
        }
    }
}
