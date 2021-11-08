pub mod header;
pub mod question;
pub mod rdata;
pub mod resource_record;

use crate::domain_name::DomainName;
use crate::message::header::Header;
use crate::message::question::Question;
use crate::message::resource_record::ResourceRecord;
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
    ) -> Self {
        let mut rng = rand::thread_rng();
        let id: u16 = rng.gen();
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
}

mod test {
    use crate::message::header::Header;
    use crate::message::question::Question;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;
    use crate::message::DnsMessage;

    #[test]
    fn constructor_test() {
        let dns_query_message =
            DnsMessage::new_query_message("test.com".to_string(), 1, 1, 0, false);

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
            DnsMessage::new_query_message("test.com".to_string(), 1, 1, 0, false);

        assert_eq!(dns_query_message.get_header().get_rd(), false);

        dns_query_message.set_header(header);

        assert_eq!(dns_query_message.get_header().get_rd(), true);
    }

    #[test]
    fn set_and_get_question() {
        let mut question = Question::new();
        question.set_qclass(2);

        let mut dns_query_message =
            DnsMessage::new_query_message("test.com".to_string(), 1, 1, 0, false);

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
            DnsMessage::new_query_message("test.com".to_string(), 1, 1, 0, false);

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
            DnsMessage::new_query_message("test.com".to_string(), 1, 1, 0, false);

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
            DnsMessage::new_query_message("test.com".to_string(), 1, 1, 0, false);

        assert_eq!(dns_query_message.get_additional().len(), 0);

        dns_query_message.set_additional(additional);

        assert_eq!(dns_query_message.get_additional().len(), 1);
    }
}
