
/// This struct represents the response of a DNS lookup.
/// 
/// It contains the list of IP addresses associated with the domain name. Depending
/// on the resquested format of the response, the IP addresses can be represented
/// as strings, structs or bytes.
pub struct LookupResponse {
    pub addresses: Vec<SocketAddr>,
}


impl LookupResponse {
    /// Create a new LookupResponse instance.
    pub fn new(addresses: Vec<SocketAddr>) -> LookupResponse {
        LookupResponse { addresses }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for address in &self.addresses {
            result.push_str(&format!("{}\n", address));
        }
        result
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        for address in &self.addresses {
            result.extend_from_slice(&address.ip().octets());
            result.extend_from_slice(&address.port().to_be_bytes());
        }
        result
    }

    pub fn to_struct(&self) -> Vec<SocketAddr> {
        self.addresses.clone()
    }

    pub fn to_vec(&self) -> Vec<String> {
        self.addresses.iter().map(|addr| addr.to_string()).collect()
    }
}