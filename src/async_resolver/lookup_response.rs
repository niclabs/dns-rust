use crate::message::{resource_record::ResourceRecord, DnsMessage};

/// This struct represents the response of a DNS lookup.
/// 
/// It contains the list of IP addresses associated with the domain name. Depending
/// on the resquested format of the response, the IP addresses can be represented
/// as strings, structs or bytes.
#[derive(Clone, Debug)]
pub struct LookupResponse {
    dns_msg_response: DnsMessage,
}

impl LookupResponse {
    /// Create a new LookupResponse instance.
    pub fn new(dns_msg_response: DnsMessage) -> LookupResponse {
        LookupResponse { dns_msg_response }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for address in &self.dns_msg_response.get_answer() {
            result.push_str(&format!("{}\n", address));
        }
        result
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.dns_msg_response.to_bytes()
    }

    pub fn to_dns_msg(&self) -> DnsMessage {
        self.dns_msg_response.clone()
    }

    pub fn to_vec_of_rr(&self) -> Vec<ResourceRecord> {
        self.dns_msg_response.get_answer()
    }
}



#[cfg(test)]
mod lookup_response_tests {
    use crate::message::DnsMessage;
    use super::LookupResponse;

    // use tokio::runtime::Runtime;
    #[test]
    fn new_lookup_response() {
        let dns_response = DnsMessage::new();
        let lookup_response = LookupResponse::new(dns_response);
        assert_eq!(lookup_response.to_string(), "");
    }
    







}