use core::fmt;

use crate::domain_name::DomainName;
use crate::message::resource_record::{FromBytes, ToBytes};


/// RFC 2782: https://datatracker.ietf.org/doc/html/rfc2782
/// An struct that represents the `Rdata` for srv type.

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SrvRdata {
    /// The priority of this target host. A client MUST attempt to contact the target host with the lowest-numbered priority it can reach; target hosts with the same priority SHOULD be tried in an order defined by the weight field.
    priority: u16,
    /// A server selection mechanism. The weight field specifies a relative weight for entries with the same priority. Larger weights SHOULD be given a proportionately higher probability of being selected.
    weight: u16,
    /// The port on this target host of this service.
    port: u16,
    /// The domain name of the target host.
    target: DomainName,
}

impl ToBytes for SrvRdata {
    /// Return a `Vec<u8>` of bytes that represents the srv rdata.
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        let priority_bytes = self.get_priority().to_be_bytes();
        let weight_bytes = self.get_weight().to_be_bytes();
        let port_bytes = self.get_port().to_be_bytes();
        let target_bytes = self.get_target().to_bytes();

        for byte in priority_bytes.as_slice() {
            bytes.push(*byte);
        }

        for byte in weight_bytes.as_slice() {
            bytes.push(*byte);
        }

        for byte in port_bytes.as_slice() {
            bytes.push(*byte);
        }

        for byte in target_bytes.as_slice() {
            bytes.push(*byte);
        }

        bytes
    
    }
}

impl FromBytes<Result<Self, &'static str>> for SrvRdata {
    /// Return a `SrvRdata` from an array of bytes.
    fn from_bytes(bytes: &[u8], full_msg: &[u8]) -> Result<Self, &'static str> {
        let bytes_len = bytes.len();

        if bytes_len < 6 {
            return Err("The bytes slice is too short to parse the SRV Rdata");
        }

        let priority = u16::from_be_bytes([bytes[0], bytes[1]]);
        let weight = u16::from_be_bytes([bytes[2], bytes[3]]);
        let port = u16::from_be_bytes([bytes[4], bytes[5]]);
        let (target, _) = DomainName::from_bytes(&bytes[6..], full_msg).unwrap();

        let srv_rdata = SrvRdata::new_with_values(priority, weight, port, target);

        Ok(srv_rdata)
    }
}

impl SrvRdata {

    /// Creates a new `SrvRdata` with default values.
    /// 
    /// # Example
    /// ```
    /// let srv_rdata = SrvRdata::new();
    /// ```
    pub fn new() -> SrvRdata {
        SrvRdata {
            priority: 0,
            weight: 0,
            port: 0,
            target: DomainName::new(),
        }
    }

    /// Creates a new `SrvRdata` with the specified values.
    /// # Example
    /// ```
    /// let srv_rdata = SrvRdata::new_with_values(1, 2, 3, DomainName::new_from_str("www.example.com"));
    /// ```
    pub fn new_with_values(priority: u16, weight: u16, port: u16, target: DomainName) -> SrvRdata {
        SrvRdata {
            priority,
            weight,
            port,
            target,
        }
    }
}

impl SrvRdata {
    /// Gets the priority atrribute of the SrvRdata.
    pub fn get_priority(&self) -> u16 {
        self.priority
    }

    /// Gets the weight atrribute of the SrvRdata.
    pub fn get_weight(&self) -> u16 {
        self.weight
    }

    /// Gets the port atrribute of the SrvRdata.
    pub fn get_port(&self) -> u16 {
        self.port
    }

    /// Gets the target atrribute of the SrvRdata.
    pub fn get_target(&self) -> DomainName {
        self.target.clone()
    }
}

impl SrvRdata {
    /// Sets the priority atrribute of the SrvRdata.
    pub fn set_priority(&mut self, priority: u16) {
        self.priority = priority;
    }

    /// Sets the weight atrribute of the SrvRdata.
    pub fn set_weight(&mut self, weight: u16) {
        self.weight = weight;
    }

    /// Sets the port atrribute of the SrvRdata.
    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    /// Sets the target atrribute of the SrvRdata.
    pub fn set_target(&mut self, target: DomainName) {
        self.target = target;
    }
}

impl fmt::Display for SrvRdata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Priority: {}, 
        Weight: {}, 
        Port: {}, 
        Target: {}", 
        self.get_priority(), 
        self.get_weight(), 
        self.get_port(), 
        self.get_target())
    
    }
}

#[cfg(test)]
mod srv_rdata_tests {
    use super::*;

    #[test]
    fn construct_srv_rdata() {
        let srv_rdata = SrvRdata::new();
        assert_eq!(srv_rdata.get_priority(), 0);
        assert_eq!(srv_rdata.get_weight(), 0);
        assert_eq!(srv_rdata.get_port(), 0);
        assert_eq!(srv_rdata.get_target(), DomainName::new());
    }

    #[test]
    fn construct_srv_rdata_with_values() {
        let srv_rdata = SrvRdata::new_with_values(1, 2, 3, DomainName::new_from_str("www.example.com"));
        assert_eq!(srv_rdata.get_priority(), 1);
        assert_eq!(srv_rdata.get_weight(), 2);
        assert_eq!(srv_rdata.get_port(), 3);
        assert_eq!(srv_rdata.get_target(), DomainName::new_from_str("www.example.com"));
    }

    #[test]
    fn get_and_set_srv_rdata_values() {
        let mut srv_rdata = SrvRdata::new();
        srv_rdata.set_priority(1);
        srv_rdata.set_weight(2);
        srv_rdata.set_port(3);
        srv_rdata.set_target(DomainName::new_from_str("www.example.com"));

        assert_eq!(srv_rdata.get_priority(), 1);
        assert_eq!(srv_rdata.get_weight(), 2);
        assert_eq!(srv_rdata.get_port(), 3);
        assert_eq!(srv_rdata.get_target(), DomainName::new_from_str("www.example.com"));
    }

    #[test]
    fn to_bytes() {
        let srv_rdata = SrvRdata::new_with_values(1, 2, 3, DomainName::new_from_str("www.example.com"));
        let bytes = srv_rdata.to_bytes();
        let expected_bytes = vec![0, 1,
         0, 2, 
         0, 3, 
         3, 119, 119, 119, 
         7, 101, 120, 97, 109, 112, 108, 101, 
         3, 99, 111, 109,
         0];
        assert_eq!(bytes, expected_bytes);
    }

    #[test]
    fn from_bytes() {
        let bytes = vec![0, 1,
         0, 2, 
         0, 3, 
         3, 119, 119, 119, 
         7, 101, 120, 97, 109, 112, 108, 101, 
         3, 99, 111, 109,
         0];

        let srv_rdata = SrvRdata::from_bytes(&bytes, &bytes).unwrap();

        assert_eq!(srv_rdata.get_priority(), 1);
        assert_eq!(srv_rdata.get_weight(), 2);
        assert_eq!(srv_rdata.get_port(), 3);
        assert_eq!(srv_rdata.get_target(), DomainName::new_from_str("www.example.com"));
    }
}