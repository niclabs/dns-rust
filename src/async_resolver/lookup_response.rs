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
    use crate::{domain_name::DomainName,  message::{class_qclass::Qclass, rdata::{a_rdata::ARdata, Rdata}, resource_record::ResourceRecord, type_qtype::Qtype, DnsMessage}};
    use super::LookupResponse;

    // use tokio::runtime::Runtime;
    #[test]
    fn new_lookup_response() {
        let dns_response = DnsMessage::new();
        let lookup_response = LookupResponse::new(dns_response);
        assert_eq!(lookup_response.to_string(), "");
    }
    
    #[test]
    fn to_string() {
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        answer.push(resource_record);

        let mut dns_query_message =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::A,
                Qclass::IN,
                0,
                false,
                1);
        dns_query_message.set_answer(answer);
        let lookup_response = LookupResponse::new(dns_query_message);
        assert_eq!(lookup_response.to_string(), "RR: - type:1 - class:1");
        
        
    }
    







}