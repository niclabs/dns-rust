use crate::message::{resource_record::ResourceRecord, DnsMessage};
use std::fmt;

/// This struct represents the response of a DNS lookup.
/// 
/// This response corresponds to the return value of the `lookup` method of 
/// the `AsyncResolver` struct. It is a wrapper around a `DnsMessage` struct.
/// With the methods of this struct, you can convert the response to a string,
/// a byte vector, a `DnsMessage` struct or a vector of `ResourceRecord` structs,
/// depending on your needs.
#[derive(Clone, Debug, PartialEq)]
pub struct LookupResponse {
    // The DNS message response.
    dns_msg_response: DnsMessage,
}

impl LookupResponse {
    /// Create a new `LookupResponse` instance.
    pub fn new(dns_msg_response: DnsMessage) -> LookupResponse {
        LookupResponse { dns_msg_response }
    }

    /// Convert the response to a byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.dns_msg_response.to_bytes()
    }

    /// Convert the response to a `DnsMessage` struct.
    /// 
    /// This method returns a clone of the `DnsMessage` struct that is 
    /// wrapped. This is useful if you want to access the `DnsMessage`
    /// using this library's API.
    pub fn to_dns_msg(&self) -> DnsMessage {
        self.dns_msg_response.clone()
    }

    /// Convert the response to a vector of `ResourceRecord` structs.
    /// 
    /// This method returns a vector of `ResourceRecord` structs that are
    /// contained in the `DnsMessage` struct that is wrapped. This is useful
    /// if you want to access the `ResourceRecord` structs using this library's
    /// API.
    pub fn to_vec_of_rr(&self) -> Vec<ResourceRecord> {
        self.dns_msg_response.get_answer()
    }
}

impl fmt::Display for LookupResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for address in &self.dns_msg_response.get_answer() {
            result.push_str(&format!("{} \n", address));
        }
        for address in &self.dns_msg_response.get_authority() {
            result.push_str(&format!("{} \n", address));
        }
        for address in &self.dns_msg_response.get_additional() {
            result.push_str(&format!("{} \n", address));
        }
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod lookup_response_tests {
    use std::net::IpAddr;
    use crate::{
        domain_name::DomainName,  
        message::{
            class_qclass::Qclass, 
            class_rclass::Rclass, 
            header::Header, 
            question::Question, 
            rdata::{a_rdata::ARdata, txt_rdata::TxtRdata, Rdata}, 
            resource_record::ResourceRecord, 
            type_qtype::Qtype, 
            type_rtype::Rtype, 
            DnsMessage
        }
    };
    use super::LookupResponse;

    #[test]
    fn new_lookup_response() {
        let dns_response = DnsMessage::new();
        let lookup_response = LookupResponse::new(dns_response);
        assert_eq!(lookup_response.to_string(), "");
    }
    
    #[test]
    fn to_string() {
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let a_rdata = ARdata::new_from_addr(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_name(DomainName::new_from_string("example.com".to_string()));
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

        println!("{}", lookup_response.to_string());
        assert_eq!(
            lookup_response.to_string(), 
            "example.com  IN  A  0  127.0.0.1".to_string()
        );
    }

    #[test]
    fn to_bytes() {
        let mut header = Header::new();

        header.set_id(0b0010010010010101);
        header.set_qr(true);
        header.set_op_code(2);
        header.set_tc(true);
        header.set_rcode(8);
        header.set_ancount(0b0000000000000001);
        header.set_qdcount(1);

        let mut question = Question::new();

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test.com"));

        question.set_qname(domain_name);
        question.set_qtype(Qtype::CNAME);
        question.set_qclass(Qclass::CS);

        let txt_rdata = Rdata::TXT(TxtRdata::new(vec!["hello".to_string()]));
        let mut resource_record = ResourceRecord::new(txt_rdata);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("dcc.cl"));

        resource_record.set_name(domain_name);
        resource_record.set_type_code(Rtype::TXT);
        resource_record.set_rclass(Rclass::IN);
        resource_record.set_ttl(5642);
        resource_record.set_rdlength(6);

        let answer = vec![resource_record];

        let mut dns_msg = DnsMessage::new();
        dns_msg.set_header(header);
        dns_msg.set_question(question);
        dns_msg.set_answer(answer);
      

        let real_bytes: [u8; 50] = [
            0b00100100, 0b10010101, 0b10010010, 0b00001000, 0, 1, 0b00000000, 0b00000001, 0, 0, 0,
            0, 4, 116, 101, 115, 116, 3, 99, 111, 109, 0, 0, 5, 0, 2, 3, 100, 99, 99, 2, 99, 108,
            0, 0, 16, 0, 1, 0, 0, 0b00010110, 0b00001010, 0, 6, 5, 104, 101, 108, 108, 111,
        ];

        assert_eq!(dns_msg.to_bytes(), real_bytes.to_vec());

    }

    #[test]
    fn to_dns_msg() {
        let mut header = Header::new();

        header.set_id(0b0010010010010101);
        header.set_qr(true);
        header.set_op_code(2);
        header.set_tc(true);
        header.set_rcode(8);
        header.set_ancount(0b0000000000000001);
        header.set_qdcount(1);

        let mut question = Question::new();

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test.com"));

        question.set_qname(domain_name);
        question.set_qtype(Qtype::CNAME);
        question.set_qclass(Qclass::CS);

        let txt_rdata = Rdata::TXT(TxtRdata::new(vec!["hello".to_string()]));
        let mut resource_record = ResourceRecord::new(txt_rdata);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("dcc.cl"));

        resource_record.set_name(domain_name);
        resource_record.set_type_code(Rtype::TXT);
        resource_record.set_rclass(Rclass::IN);
        resource_record.set_ttl(5642);
        resource_record.set_rdlength(6);

        let answer = vec![resource_record];

        let mut dns_msg = DnsMessage::new();
        dns_msg.set_header(header);
        dns_msg.set_question(question);
        dns_msg.set_answer(answer);
      

        let lookup_response = LookupResponse::new(dns_msg);
        let dns_from_lookup = lookup_response.to_dns_msg();
        assert_eq!(dns_from_lookup.get_header().get_id(), 0b0010010010010101);
        assert_eq!(dns_from_lookup.get_header().get_qr(), true);
        assert_eq!(dns_from_lookup.get_header().get_op_code(), 2);
        assert_eq!(dns_from_lookup.get_header().get_tc(), true);
        assert_eq!(dns_from_lookup.get_header().get_rcode(), 8);
        assert_eq!(dns_from_lookup.get_header().get_ancount(), 0b0000000000000001);
        assert_eq!(dns_from_lookup.get_header().get_qdcount(), 1);
        assert_eq!(dns_from_lookup.get_question().get_qname().get_name(), "test.com");
        assert_eq!(dns_from_lookup.get_question().get_qtype(), Qtype::CNAME);
        assert_eq!(dns_from_lookup.get_question().get_qclass(), Qclass::CS);
        assert_eq!(dns_from_lookup.get_answer()[0].get_name().get_name(), "dcc.cl");
    }

    #[test]
    fn to_vec_of_rr() {
        let mut header = Header::new();

        header.set_id(0b0010010010010101);
        header.set_qr(true);
        header.set_op_code(2);
        header.set_tc(true);
        header.set_rcode(8);
        header.set_ancount(0b0000000000000001);
        header.set_qdcount(1);

        let mut question = Question::new();

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test.com"));

        question.set_qname(domain_name);
        question.set_qtype(Qtype::CNAME);
        question.set_qclass(Qclass::CS);

        let txt_rdata = Rdata::TXT(TxtRdata::new(vec!["hello".to_string()]));
        let mut resource_record = ResourceRecord::new(txt_rdata);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("dcc.cl"));

        resource_record.set_name(domain_name);
        resource_record.set_type_code(Rtype::TXT);
        resource_record.set_rclass(Rclass::IN);
        resource_record.set_ttl(5642);
        resource_record.set_rdlength(6);

        let answer = vec![resource_record];

        let mut dns_msg = DnsMessage::new();
        dns_msg.set_header(header);
        dns_msg.set_question(question);
        dns_msg.set_answer(answer);
      
        let lookup_response = LookupResponse::new(dns_msg);
        let vec_of_rr = lookup_response.to_vec_of_rr();
        assert_eq!(vec_of_rr[0].get_name().get_name(), "dcc.cl");
    }
}