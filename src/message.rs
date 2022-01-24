pub mod header;
pub mod question;
pub mod rdata;
pub mod resource_record;

use crate::domain_name::DomainName;
use crate::message::header::Header;
use crate::message::question::Question;
use crate::message::resource_record::ResourceRecord;
use crate::name_server::zone::NSZone;

use rand::thread_rng;
use rand::Rng;
use std::vec::Vec;

#[derive(Clone)]
/// Structs that represents a dns message
pub struct DnsMessage {
    header: Header,
    question: Question,
    answer: Vec<ResourceRecord>,
    authority: Vec<ResourceRecord>,
    additional: Vec<ResourceRecord>,
}

impl DnsMessage {
    /// Creates a new query message
    ///
    /// # Examples
    /// '''
    /// let dns_query_message =
    /// DnsMessage::new_query_message("test.com".to_string(), 1, 1, 0, false);
    ///
    /// assert_eq!(dns_query_message.header.get_rd(), false);
    /// assert_eq!(dns_query_message.question.get_qtype(), 1);
    /// assert_eq!(dns_query_message.question.get_qclass(), 1);
    /// assert_eq!(
    ///     dns_query_message.question.get_qname().get_name(),
    ///     "test.com".to_string()
    /// );
    /// '''
    ///
    pub fn new_query_message(
        qname: String,
        qtype: u16,
        qclass: u16,
        op_code: u8,
        rd: bool,
        id: u16,
    ) -> Self {
        let qr = false;
        let qdcount = 1;
        let mut header = Header::new();

        header.set_id(id);
        header.set_qr(qr);
        header.set_op_code(op_code);
        header.set_rd(rd);
        header.set_qdcount(qdcount);

        let mut question = Question::new();
        let mut domain_name = DomainName::new();

        domain_name.set_name(qname);

        question.set_qname(domain_name);
        question.set_qtype(qtype);
        question.set_qclass(qclass);

        let dns_message = DnsMessage {
            header: header,
            question: question,
            answer: Vec::new(),
            authority: Vec::new(),
            additional: Vec::new(),
        };

        dns_message
    }

    pub fn new() -> Self {
        let msg = DnsMessage {
            header: Header::new(),
            question: Question::new(),
            answer: Vec::new(),
            authority: Vec::new(),
            additional: Vec::new(),
        };

        msg
    }

    pub fn new_response_message(
        qname: String,
        qtype: u16,
        qclass: u16,
        op_code: u8,
        rd: bool,
        id: u16,
    ) -> Self {
        let qr = true;
        let qdcount = 1;
        let mut header = Header::new();

        header.set_id(id);
        header.set_qr(qr);
        header.set_op_code(op_code);
        header.set_rd(rd);
        header.set_qdcount(qdcount);

        let mut question = Question::new();
        let mut domain_name = DomainName::new();

        domain_name.set_name(qname);

        question.set_qname(domain_name);
        question.set_qtype(qtype);
        question.set_qclass(qclass);

        let dns_message = DnsMessage {
            header: header,
            question: question,
            answer: Vec::new(),
            authority: Vec::new(),
            additional: Vec::new(),
        };

        dns_message
    }

    pub fn soa_rr_query_msg(zone: NSZone) -> Self {
        let mut rng = thread_rng();
        let msg_id = rng.gen();

        let zone_name = zone.get_name();

        let msg = DnsMessage::new_query_message(zone_name, 6, 1, 0, false, msg_id);

        msg
    }

    pub fn format_error_msg() -> Self {
        let mut msg = DnsMessage::new();
        let mut header = msg.get_header();

        header.set_rcode(1);
        msg.set_header(header);

        msg
    }

    pub fn axfr_query_message(qname: String) -> Self {
        let mut rng = thread_rng();
        let msg_id = rng.gen();

        let msg = DnsMessage::new_query_message(qname, 252, 1, 0, false, msg_id);

        msg
    }

    pub fn not_implemented_msg(mut msg: DnsMessage) -> Self {
        let mut header = msg.get_header();
        header.set_rcode(4);

        msg.set_header(header);

        msg
    }

    // Creates a DnsMessage from an array of bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        let bytes_len = bytes.len();

        if bytes_len < 12 {
            return Err("Format Error");
        }

        // Header
        let header = Header::from_bytes(&bytes[0..12]);

        // Question
        let q_count = header.get_qdcount();

        if bytes_len < 13 {
            return Err("Format Error");
        }

        let (mut question, mut no_question_bytes) = (Question::new(), &bytes[12..]);

        if q_count > 0 {
            let question_result = Question::from_bytes(&bytes[12..], bytes);

            match question_result {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            }

            let question_and_bytes = question_result.unwrap();
            question = question_and_bytes.0;
            no_question_bytes = question_and_bytes.1;
        }

        // ResourceRecords

        let mut answer = Vec::<ResourceRecord>::new();
        let mut authority = Vec::<ResourceRecord>::new();
        let mut additional = Vec::<ResourceRecord>::new();

        let answer_rr_size = header.get_ancount();
        let authority_rr_size = header.get_nscount();
        let additional_rr_size = header.get_arcount();

        // Answers
        for _i in 0..answer_rr_size {
            let rr_result = ResourceRecord::from_bytes(no_question_bytes, bytes);

            match rr_result {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            }

            let (resource_record, other_rr_bytes) = rr_result.unwrap();

            answer.push(resource_record);
            no_question_bytes = other_rr_bytes;
        }

        // Authorities
        for _i in 0..authority_rr_size {
            let rr_result = ResourceRecord::from_bytes(no_question_bytes, bytes);

            match rr_result {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            }

            let (resource_record, other_rr_bytes) = rr_result.unwrap();

            authority.push(resource_record);
            no_question_bytes = other_rr_bytes;
        }

        // Additional
        for _i in 0..additional_rr_size {
            let rr_result = ResourceRecord::from_bytes(no_question_bytes, bytes);

            match rr_result {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            }

            let (resource_record, other_rr_bytes) = rr_result.unwrap();

            additional.push(resource_record);
            no_question_bytes = other_rr_bytes;
        }

        // Create message
        let dns_message = DnsMessage {
            header: header,
            question: question,
            answer: answer,
            authority: authority,
            additional: additional,
        };

        Ok(dns_message)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut header_bytes = self.get_header().to_bytes().to_vec();
        let mut question_bytes = self.get_question().to_bytes();
        let mut answer_bytes: Vec<u8> = Vec::new();
        let mut authority_bytes: Vec<u8> = Vec::new();
        let mut additional_bytes: Vec<u8> = Vec::new();

        for answer in self.get_answer() {
            answer_bytes.append(&mut answer.to_bytes());
        }

        for authority in self.get_authority() {
            authority_bytes.append(&mut authority.to_bytes());
        }

        for additional in self.get_additional() {
            additional_bytes.append(&mut additional.to_bytes());
        }

        let mut dns_msg_bytes = Vec::<u8>::new();

        dns_msg_bytes.append(&mut header_bytes);
        dns_msg_bytes.append(&mut question_bytes);
        dns_msg_bytes.append(&mut answer_bytes);
        dns_msg_bytes.append(&mut authority_bytes);
        dns_msg_bytes.append(&mut additional_bytes);

        dns_msg_bytes
    }

    pub fn update_header_counters(&mut self) {
        let answer = self.get_answer();
        let authority = self.get_authority();
        let additional = self.get_additional();

        let mut header = self.get_header();
        header.set_ancount(answer.len() as u16);
        header.set_nscount(authority.len() as u16);
        header.set_arcount(additional.len() as u16);

        self.set_header(header);
    }

    pub fn add_answers(&mut self, mut answers: Vec<ResourceRecord>) {
        let mut msg_answers = self.get_answer();

        msg_answers.append(&mut answers);
        self.set_answer(msg_answers);
    }

    pub fn add_authorities(&mut self, mut authorities: Vec<ResourceRecord>) {
        let mut msg_authorities = self.get_authority();

        msg_authorities.append(&mut authorities);
        self.set_answer(msg_authorities);
    }

    pub fn add_additionals(&mut self, mut additionals: Vec<ResourceRecord>) {
        let mut msg_additionals = self.get_additional();

        msg_additionals.append(&mut additionals);
        self.set_answer(msg_additionals);
    }
}

// Getters
impl DnsMessage {
    /// Gets the header field
    pub fn get_header(&self) -> Header {
        self.header.clone()
    }

    /// Gets the question field
    pub fn get_question(&self) -> Question {
        self.question.clone()
    }

    /// Gets the answer field
    pub fn get_answer(&self) -> Vec<ResourceRecord> {
        self.answer.clone()
    }

    /// Gets the authority field
    pub fn get_authority(&self) -> Vec<ResourceRecord> {
        self.authority.clone()
    }

    /// Gets the additional field
    pub fn get_additional(&self) -> Vec<ResourceRecord> {
        self.additional.clone()
    }

    /// Gets the id from the header
    pub fn get_query_id(&self) -> u16 {
        self.get_header().get_id()
    }

    pub fn get_question_qtype(&self) -> String {
        let qtype = match self.get_question().get_qtype() {
            1 => "A".to_string(),
            2 => "NS".to_string(),
            5 => "CNAME".to_string(),
            6 => "SOA".to_string(),
            11 => "WKS".to_string(),
            12 => "PTR".to_string(),
            13 => "HINFO".to_string(),
            14 => "MINFO".to_string(),
            15 => "MX".to_string(),
            16 => "TXT".to_string(),
            _ => unreachable!(),
        };

        qtype
    }
}

// Setters
impl DnsMessage {
    /// Sets the header field with a new Header
    pub fn set_header(&mut self, header: Header) {
        self.header = header;
    }

    /// Sets the question field with a new Question
    pub fn set_question(&mut self, question: Question) {
        self.question = question;
    }

    /// Sets the answer field with a new Vec<ResourceRecord>
    pub fn set_answer(&mut self, answer: Vec<ResourceRecord>) {
        self.answer = answer;
    }

    /// Sets the authority field with a new Vec<ResourceRecord>
    pub fn set_authority(&mut self, authority: Vec<ResourceRecord>) {
        self.authority = authority;
    }

    /// Sets the additional field with a new Vec<ResourceRecord>
    pub fn set_additional(&mut self, additional: Vec<ResourceRecord>) {
        self.additional = additional;
    }

    /// Sets the id from the header with new value
    pub fn set_query_id(&mut self, id: u16) {
        let mut header = self.get_header();
        header.set_id(id);
        self.set_header(header);
    }
}

mod test {
    use crate::domain_name::DomainName;
    use crate::message::header::Header;
    use crate::message::question::Question;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::txt_rdata::TxtRdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;
    use crate::message::DnsMessage;

    #[test]
    fn constructor_test() {
        let dns_query_message =
            DnsMessage::new_query_message("test.com".to_string(), 1, 1, 0, false, 1);

        assert_eq!(dns_query_message.header.get_rd(), false);
        assert_eq!(dns_query_message.question.get_qtype(), 1);
        assert_eq!(dns_query_message.question.get_qclass(), 1);
        assert_eq!(
            dns_query_message.question.get_qname().get_name(),
            "test.com".to_string()
        );
    }

    #[test]
    fn set_and_get_header() {
        let mut header = Header::new();
        header.set_rd(true);

        let mut dns_query_message =
            DnsMessage::new_query_message("test.com".to_string(), 1, 1, 0, false, 1);

        assert_eq!(dns_query_message.get_header().get_rd(), false);

        dns_query_message.set_header(header);

        assert_eq!(dns_query_message.get_header().get_rd(), true);
    }

    #[test]
    fn set_and_get_question() {
        let mut question = Question::new();
        question.set_qclass(2);

        let mut dns_query_message =
            DnsMessage::new_query_message("test.com".to_string(), 1, 1, 0, false, 1);

        assert_eq!(dns_query_message.get_question().get_qclass(), 1);

        dns_query_message.set_question(question);

        assert_eq!(dns_query_message.get_question().get_qclass(), 2);
    }

    #[test]
    fn set_and_get_answer() {
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        answer.push(resource_record);

        let mut dns_query_message =
            DnsMessage::new_query_message("test.com".to_string(), 1, 1, 0, false, 1);

        assert_eq!(dns_query_message.get_answer().len(), 0);

        dns_query_message.set_answer(answer);

        assert_eq!(dns_query_message.get_answer().len(), 1);
    }

    #[test]
    fn set_and_get_authority() {
        let mut authority: Vec<ResourceRecord> = Vec::new();
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        authority.push(resource_record);

        let mut dns_query_message =
            DnsMessage::new_query_message("test.com".to_string(), 1, 1, 0, false, 1);

        assert_eq!(dns_query_message.get_authority().len(), 0);

        dns_query_message.set_authority(authority);

        assert_eq!(dns_query_message.get_authority().len(), 1);
    }

    #[test]
    fn set_and_get_additional() {
        let mut additional: Vec<ResourceRecord> = Vec::new();
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        additional.push(resource_record);

        let mut dns_query_message =
            DnsMessage::new_query_message("test.com".to_string(), 1, 1, 0, false, 1);

        assert_eq!(dns_query_message.get_additional().len(), 0);

        dns_query_message.set_additional(additional);

        assert_eq!(dns_query_message.get_additional().len(), 1);
    }

    #[test]
    fn from_bytes_test() {
        let bytes: [u8; 50] = [
            0b00100100, 0b10010101, 0b10010010, 0b00000000, 0, 1, 0b00000000, 1, 0, 0, 0, 0, 4,
            116, 101, 115, 116, 3, 99, 111, 109, 0, 0, 1, 0, 1, 3, 100, 99, 99, 2, 99, 108, 0, 0,
            16, 0, 1, 0, 0, 0b00010110, 0b00001010, 0, 6, 5, 104, 101, 108, 108, 111,
        ];

        let dns_message = DnsMessage::from_bytes(&bytes).unwrap();

        let header = dns_message.get_header();
        let question = dns_message.get_question();
        let answer = dns_message.get_answer();
        let authority = dns_message.get_authority();
        let additional = dns_message.get_additional();

        // Header
        assert_eq!(header.get_id(), 0b0010010010010101);
        assert_eq!(header.get_qr(), true);
        assert_eq!(header.get_op_code(), 2);
        assert_eq!(header.get_tc(), true);
        assert_eq!(header.get_rcode(), 0);
        assert_eq!(header.get_ancount(), 1);

        // Question
        assert_eq!(question.get_qname().get_name(), String::from("test.com"));
        assert_eq!(question.get_qtype(), 1);
        assert_eq!(question.get_qclass(), 1);

        // Answer
        assert_eq!(answer.len(), 1);

        assert_eq!(answer[0].get_name().get_name(), String::from("dcc.cl"));
        assert_eq!(answer[0].get_type_code(), 16);
        assert_eq!(answer[0].get_class(), 1);
        assert_eq!(answer[0].get_ttl(), 5642);
        assert_eq!(answer[0].get_rdlength(), 6);
        assert_eq!(
            match answer[0].get_rdata() {
                Rdata::SomeTxtRdata(val) => val.get_text(),
                _ => unreachable!(),
            },
            vec!["hello".to_string()]
        );

        // Authority
        assert_eq!(authority.len(), 0);

        // Additional
        assert_eq!(additional.len(), 0);
    }

    #[test]
    fn to_bytes_test() {
        let mut header = Header::new();

        header.set_id(0b0010010010010101);
        header.set_qr(true);
        header.set_op_code(2);
        header.set_tc(true);
        header.set_rcode(8);
        header.set_ancount(0b0000000000000001);

        let mut question = Question::new();

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test.com"));

        question.set_qname(domain_name);
        question.set_qtype(5);
        question.set_qclass(2);

        let txt_rdata = Rdata::SomeTxtRdata(TxtRdata::new(vec!["hello".to_string()]));
        let mut resource_record = ResourceRecord::new(txt_rdata);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("dcc.cl"));

        resource_record.set_name(domain_name);
        resource_record.set_type_code(16);
        resource_record.set_class(1);
        resource_record.set_ttl(5642);
        resource_record.set_rdlength(6);

        let answer = vec![resource_record];

        let dns_msg = DnsMessage {
            header: header,
            question: question,
            answer: answer,
            authority: Vec::new(),
            additional: Vec::new(),
        };

        let msg_bytes = &dns_msg.to_bytes();

        let real_bytes: [u8; 50] = [
            0b00100100, 0b10010101, 0b10010010, 0b00001000, 0, 0, 0b00000000, 0b00000001, 0, 0, 0,
            0, 4, 116, 101, 115, 116, 3, 99, 111, 109, 0, 0, 5, 0, 2, 3, 100, 99, 99, 2, 99, 108,
            0, 0, 16, 0, 1, 0, 0, 0b00010110, 0b00001010, 0, 6, 5, 104, 101, 108, 108, 111,
        ];

        let mut i = 0;

        for value in msg_bytes {
            assert_eq!(*value, real_bytes[i]);
            i += 1;
        }
    }
}
