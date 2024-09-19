pub mod header;
pub mod question;
pub mod rdata;
pub mod resource_record;
pub mod rrtype;
pub mod rclass;
pub mod rcode;

use crate::message::rclass::Rclass;
use crate::message::rrtype::Rrtype;
use crate::message::rcode::Rcode;
use crate::domain_name::DomainName;
use crate::message::header::Header;
use crate::message::question::Question;
use crate::message::resource_record::ResourceRecord;
use crate::message::rdata::Rdata;
use crate::message::rdata::opt_rdata::OptRdata;
use crate::tsig;
use crate::tsig::tsig_algorithm::TsigAlgorithm;
use crate::message::rdata::opt_rdata::option_code::OptionCode;
use rand::thread_rng;
use rand::Rng;
use resource_record::ToBytes;
use core::fmt;
use std::vec::Vec;
use std::time::SystemTime;

#[derive(Clone)]
/// Structs that represents a DNS message.
/// 
/// ```text
/// +---------------------+
/// |        Header       |
/// +---------------------+
/// |       Question      | the question for the name server
/// +---------------------+
/// |        Answer       | RRs answering the question
/// +---------------------+
/// |      Authority      | RRs pointing toward an authority
/// +---------------------+
/// |      Additional     | RRs holding additional information
/// +---------------------+
/// ```
#[derive (PartialEq, Debug)]
pub struct DnsMessage {
    header: Header,
    question: Question,
    answer: Vec<ResourceRecord>,
    authority: Vec<ResourceRecord>,
    additional: Vec<ResourceRecord>,
}

impl DnsMessage {
    /// Creates a new query message.
    ///
    /// # Examples
    /// 
    /// ```
    /// let dns_query_message =
    /// DnsMessage::new_query_message(DomainName::new_from_str("example.com".to_string()), Rrtype::A, Rclass:IN, 0, false);
    ///
    /// assert_eq!(dns_query_message.header.get_rd(), false);
    /// assert_eq!(dns_query_message.question.get_qtype(), Rrtype::A);
    /// assert_eq!(dns_query_message.question.get_rclass(), Rclass::IN);
    /// assert_eq!(
    ///     dns_query_message.question.get_qname().get_name(),
    ///     "example.com".to_string()
    /// );
    /// ```
    ///
    pub fn new_query_message(
        qname: DomainName,
        rrtype: Rrtype,
        rclass: Rclass,
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
        let domain_name = qname;

        question.set_qname(domain_name);
        question.set_rrtype(rrtype);
        question.set_rclass(rclass);

        let dns_message = DnsMessage {
            header: header,
            question: question,
            answer: Vec::new(),
            authority: Vec::new(),
            additional: Vec::new(),
        };

        dns_message
    }
    /// Creates a new message.
    /// 
    /// # Example
    /// 
    /// ```
    /// let msg = DnsMessage::new();
    /// ```
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

    /// Creates a new response message.
    /// 
    /// # Example
    /// 
    /// ```
    /// let new_response = DnsMessage::new_response_message(String::from("test.com"), String::from("NS"), String::from("IN"), 1, true, 1);
    /// let header = new_response.get_header();
    /// let id = header.get_id();
    /// let op_code = header.get_op_code();
    /// let rd = header.get_rd();
    ///
    /// let question = new_response.get_question();
    /// let qname = question.get_qname().get_name();
    /// let rrtype = question.get_rrtype();
    /// let rclass = question.get_rclass();
    /// 
    /// assert_eq!(id, 1);
    /// assert_eq!(op_code, 1);
    /// assert!(rd);
    /// assert_eq!(qname, String::from("test.com"));
    /// assert_eq!(u16::from(rrtype), 2);
    /// assert_eq!(u16::from(rclass), 1);
    /// ```
    pub fn new_response_message(
        qname: String,
        rrtype: &str,
        rclass: &str,
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
        let rrtype_rrtype = Rrtype::from(rrtype);
        question.set_rrtype(rrtype_rrtype);
        let rclass_rclass = Rclass::from(rclass);
        question.set_rclass(rclass_rclass);

        let dns_message = DnsMessage {
            header: header,
            question: question,
            answer: Vec::new(),
            authority: Vec::new(),
            additional: Vec::new(),
        };

        dns_message
    }

    //function never used
    // pub fn soa_rr_query_msg(zone: NSZone) -> Self {
    //     let mut rng = thread_rng();
    //     let msg_id = rng.gen();

    //     let zone_name = zone.get_name();

    //     let msg = DnsMessage::new_query_message(zone_name, 6, 1, 0, false, msg_id);

    //     msg
    // }
    

    /// Creates a new error message.
    /// 
    /// # Example
    /// 
    /// ```
    /// let error_msg = DnsMessage::format_error_msg();
    /// let header = error_msg.get_header();
    /// let rcode = header.get_rcode();
    /// let qr = header.get_qr();
    /// 
    /// assert_eq!(rcode, 1);
    /// assert!(qr);
    /// ```
    pub fn format_error_msg() -> Self {
        let mut msg = DnsMessage::new();
        let mut header = msg.get_header();

        header.set_rcode(Rcode::FORMERR);
        header.set_qr(true);
        msg.set_header(header);

        msg
    }


    /// Creates a new not found error message.
    /// 
    /// # Example
    /// 
    /// ``` 
    /// let error_msg = DnsMessage::data_not_found_error_msg();
    /// let header = error_msg.get_header();
    /// let rcode = header.get_rcode();
    /// let qr = header.get_qr();
    /// let aa = header.get_aa();
    /// 
    /// assert_eq!(rcode, 3);
    /// assert!(qr);
    /// assert!(aa);
    /// ```
    pub fn data_not_found_error_msg() -> Self {
        let mut msg = DnsMessage::new();
        let mut header = msg.get_header();

        header.set_aa(true);
        header.set_qr(true);
        msg.set_header(header);

        msg
    }

    /// Adds ENDS0 to the message.
    /// 
    /// # Example
    /// ´´´
    /// let dns_query_message = new_query_message(DomainName::new_from_str("example.com".to_string()), Rrtype::A, Rclass:IN, 0, false);
    /// dns_query_message.add_edns0(Some(4096), 0, 0, Some(vec![12]));
    /// ´´´
    pub fn add_edns0(&mut self, max_payload: Option<u16>, version: u16, z: u16, option_codes: Option<Vec<u16>>){
        let mut opt_rdata = OptRdata::new();

        let mut option = Vec::new();

        if let Some(option_codes) = option_codes {
            for code in option_codes {
                option.push((OptionCode::from(code), 0, Vec::new()));
            }
        }
        opt_rdata.set_option(option);
        let rdata = Rdata::OPT(opt_rdata);

        let rdlength = rdata.to_bytes().len() as u16;

        let mut rr = ResourceRecord::new(rdata);

        rr.set_name(DomainName::new_from_string(".".to_string()));

        rr.set_type_code(Rrtype::OPT);

        rr.set_rclass(Rclass::UNKNOWN(max_payload.unwrap_or(512)));

        let ttl = u32::from(version) << 16 | u32::from(z);
        rr.set_ttl(ttl);

        rr.set_rdlength(rdlength);

        self.add_additionals(vec![rr]);

        self.update_header_counters();
    }

    /// Adds Tsig to the message.
    /// 
    /// # Example
    /// ```
    /// let dns_query_message = new_query_message(DomainName::new_from_str("example.com".to_string()), Rrtype::A, Rclass:IN, 0, false);
    /// let key = vec![1, 2, 3, 4, 5, 6, 7, 8];
    /// let alg_name = TsigAlgorithm::HmacSha1;
    /// let fudge = 300;
    /// let key_name = "key".to_string();
    /// let mac_request = vec![];
    /// dns_query_message.add_tsig(key, alg_name, fudge, key_name, mac_request);
    /// ```
    pub fn add_tsig(&mut self, key: Vec<u8>, alg_name: TsigAlgorithm, 
        fudge: u16, key_name: Option<String>, mac_request: Vec<u8>) {
        let message = self;
        let time_signed = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        tsig::sign_tsig(message, &key, alg_name, 
                                        fudge, time_signed, key_name.unwrap_or("".to_string()), mac_request);
    }

    /// Gets the MAC from the TSIG RR.
    /// 
    /// # Example
    /// ```
    /// let dns_query_message = new_query_message(DomainName::new_from_str("example.com".to_string()), Rrtype::A, Rclass:IN, 0, false);
    /// let key = vec![1, 2, 3, 4, 5, 6, 7, 8];
    /// let alg_name = TsigAlgorithm::HmacSha1;
    /// let fudge = 300;
    /// let key_name = "key".to_string();
    /// let mac_request = vec![];
    /// dns_query_message.add_tsig(key, alg_name, fudge, key_name, mac_request);
    /// let mac = dns_query_message.get_mac();
    /// ```
    pub fn get_mac(&self) -> Vec<u8> {
        let mut mac = Vec::new();
        let additional = self.get_additional();

        for rr in additional {
            if let Rdata::TSIG(tsig_rdata) = rr.get_rdata() {
                mac = tsig_rdata.get_mac();
            }
        }

        mac
    }

    /// Creates a new axfr query message.
    /// 
    /// # Example
    /// 
    /// ```
    /// let axfr_msg = DnsMessage::axfr_query_message(String::from("test.com"));
    /// let header = axfr_msg.get_header();
    /// let id = header.get_id();
    /// let qr = header.get_qr();
    /// let opcode = header.get_op_code();
    /// let rd = header.get_rd();
    /// let qdcount = header.get_qdcount();
    /// 
    /// let question = axfr_msg.get_question();
    /// let qname = question.get_qname().get_name();
    /// let rrtype = question.get_rrtype();
    /// let rclass = question.get_rclass();
    /// 
    /// assert_eq!(id, 1);
    /// assert!(qr);
    /// assert_eq!(opcode, 0);
    /// assert!(rd);
    /// assert_eq!(qdcount, 1);
    /// assert_eq!(qname, String::from("test.com"));
    /// assert_eq!(u16::from(rrtype), 252);
    /// assert_eq!(u16::from(rclass), 1);
    /// ```
    pub fn axfr_query_message(qname: DomainName) -> Self {
        let mut rng = thread_rng();
        let msg_id = rng.gen();

        let msg = DnsMessage::new_query_message(qname, Rrtype::AXFR, Rclass::IN, 0, false, msg_id);

        msg
    }


    /// Creates a new not implemented error message.
    /// 
    /// # Example
    /// 
    /// ```
    /// let error_msg = DnsMessage::not_implemented_msg();
    /// let header = error_msg.get_header();
    /// let rcode = header.get_rcode();
    /// let qr = header.get_qr();
    /// 
    /// assert_eq!(rcode, 4);
    /// assert!(qr);
    /// ```
    pub fn not_implemented_msg() -> Self {
        let mut msg = DnsMessage::new();
        let mut header = msg.get_header();
        header.set_rcode(Rcode::NOTIMP);
        header.set_qr(true);

        msg.set_header(header);

        msg
    }

    /// Creates a DnsMessage from an array of bytes.
    /// 
    /// # Example
    /// 
    /// ```
    /// let bytes = [0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0 ,0];
    /// let msg = DnsMessage::from_bytes(&bytes);
    /// let header = msg.get_header();
    /// let id = header.get_id();
    /// let qr = header.get_qr();
    /// let opcode = header.get_op_code();
    /// let aa = header.get_aa();
    /// let tc = header.get_tc();
    /// let rd = header.get_rd();
    /// let ra = header.get_ra();
    /// let z = header.get_z();
    /// let rcode = header.get_rcode();
    /// let qdcount = header.get_qdcount();
    /// let ancount = header.get_ancount();
    /// let nscount = header.get_nscount();
    /// let arcount = header.get_arcount();
    /// 
    /// assert_eq!(id, 1);
    /// assert!(!qr);
    /// assert_eq!(opcode, 0);
    /// assert!(!aa);
    /// assert!(!tc);
    /// assert!(!rd);
    /// assert!(!ra);
    /// assert_eq!(z, 0);
    /// assert_eq!(rcode, 0);
    /// assert_eq!(qdcount, 1);
    /// assert_eq!(ancount, 0);
    /// assert_eq!(nscount, 0);
    /// assert_eq!(arcount, 0);
    /// ```
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
                Ok(question_and_bytes) => {
                    question = question_and_bytes.0;
                    no_question_bytes = question_and_bytes.1;
                }
                Err(e) => return Err(e)
            };
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
        let mut dns_message = DnsMessage {
            header: header,
            question: question,
            answer: answer,
            authority: authority,
            additional: additional,
        };

        dns_message.update_header_counters();

        Ok(dns_message)
    }

    /// Creates a DnsMessage from an array of bytes.
    /// 
    /// # Example
    /// 
    /// ```
    /// let bytes = [0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0 ,0];
    /// let msg = DnsMessage::from_bytes(&bytes);
    /// let header = msg.get_header();
    /// let id = header.get_id();
    /// let qr = header.get_qr();
    /// let opcode = header.get_op_code();
    /// let aa = header.get_aa();
    /// let tc = header.get_tc();
    /// let rd = header.get_rd();
    /// let ra = header.get_ra();
    /// let z = header.get_z();
    /// let rcode = header.get_rcode();
    /// let qdcount = header.get_qdcount();
    /// let ancount = header.get_ancount();
    /// let nscount = header.get_nscount();
    /// let arcount = header.get_arcount();
    /// 
    /// assert_eq!(id, 1);
    /// assert!(!qr);
    /// assert_eq!(opcode, 0);
    /// assert!(!aa);
    /// assert!(!tc);
    /// assert!(!rd);
    /// assert!(!ra);
    /// assert_eq!(z, 0);
    /// assert_eq!(rcode, 0);
    /// assert_eq!(qdcount, 1);
    /// assert_eq!(ancount, 0);
    /// assert_eq!(nscount, 0);
    /// assert_eq!(arcount, 0);
    /// ```
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

    /// Updates the header counters.
    /// 
    /// # Example
    /// 
    /// ```
    /// let mut msg = DnsMessage::new();
    /// let mut header = Header::new();
    /// header.set_qdcount(1);
    /// header.set_ancount(1);
    /// header.set_nscount(1);
    /// header.set_arcount(1);
    /// msg.set_header(header);
    /// msg.update_header_counters();
    /// let header = msg.get_header();
    /// let qdcount = header.get_qdcount();
    /// let ancount = header.get_ancount();
    /// let nscount = header.get_nscount();
    /// let arcount = header.get_arcount();
    /// 
    /// assert_eq!(qdcount, 1);
    /// assert_eq!(ancount, 0);
    /// assert_eq!(nscount, 0);
    /// assert_eq!(arcount, 0);
    /// ```
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

    /// Adds a answers to the message.
    /// 
    /// # Example
    /// 
    /// ```
    /// let mut msg = DnsMessage::new();
    /// let mut rr = ResourceRecord::new();
    /// rr.set_name("www.example.com".to_string());
    /// rr.set_type(1);
    /// rr.set_class(1);
    /// rr.set_ttl(1);
    /// rr.set_rdlength(1);
    /// rr.set_rdata(vec![1]);
    /// msg.add_answers(vec![rr]);
    /// let answers = msg.get_answer();
    /// 
    /// assert_eq!(answers.len(), 1);
    /// ```
    pub fn add_answers(&mut self, mut answers: Vec<ResourceRecord>) {
        let mut msg_answers = self.get_answer();

        msg_answers.append(&mut answers);
        self.header.set_ancount(msg_answers.len() as u16);
        self.set_answer(msg_answers);
    }

    /// Adds a authorities to the message.
    /// 
    /// # Example
    /// 
    /// ```
    /// let mut msg = DnsMessage::new();
    /// let mut rr = ResourceRecord::new();
    /// rr.set_name("www.example.com".to_string());
    /// rr.set_type(1);
    /// rr.set_class(1);
    /// rr.set_ttl(1);
    /// rr.set_rdlength(1);
    /// rr.set_rdata(vec![1]);
    /// msg.add_authorities(vec![rr]);
    /// let authorities = msg.get_authority();
    /// 
    /// assert_eq!(authorities.len(), 1);
    /// ```
    pub fn add_authorities(&mut self, mut authorities: Vec<ResourceRecord>) {
        let mut msg_authorities = self.get_authority();

        msg_authorities.append(&mut authorities);
        self.header.set_nscount(msg_authorities.len() as u16);
        self.set_answer(msg_authorities);
    }

    /// Adds a additionals to the message.
    /// 
    /// # Example
    /// 
    /// ```
    /// let mut msg = DnsMessage::new();
    /// let mut rr = ResourceRecord::new();
    /// rr.set_name("www.example.com".to_string());
    /// rr.set_type(1);
    pub fn add_additionals(&mut self, mut additionals: Vec<ResourceRecord>) {
        let mut msg_additionals = self.get_additional();

        msg_additionals.append(&mut additionals);
        self.header.set_arcount(msg_additionals.len() as u16);
        self.set_additional(msg_additionals);
    }

    ///Checks the Op_code of a message
    ///
    /// # Example
    /// ```
    /// let mut msg = DnsMessage::new();
    /// let mut header = Header::new();
    /// header.set_op_code(1);
    /// msg.set_header(header);
    /// let result = msg.check_op_code();
    /// ```
    pub fn check_op_code(&self) -> Result<(), &'static str>{
        let header = self.get_header();
        let op_code = header.get_op_code();
        match op_code {
            1 => Err("IQuery not Implemented") ,
            _ => Ok(())
        }
    }

}

impl fmt::Display for DnsMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        let question = self.get_question();
        let answers = self.get_answer().into_iter();
        let authority = self.get_authority().into_iter();
        let additional = self.get_additional().into_iter();
        result.push_str(&format!("Question section\n"));
        result.push_str(&format!("{}\n", question));
        result.push_str(&format!("Answer section\n"));
        answers.for_each(|answer| result.push_str(&format!("{}\n", answer)));
        result.push_str(&format!("Authority section\n"));
        authority.for_each(|authority| result.push_str(&format!("{}\n", authority)));
        result.push_str(&format!("Additional section\n"));
        additional.for_each(|additional| result.push_str(&format!("{}\n", additional)));
        write!(f, "{}", result)
    }
}

// Getters
impl DnsMessage {
    /// Gets the header field.
    pub fn get_header(&self) -> Header {
        self.header.clone()
    }

    /// Gets the question field.
    pub fn get_question(&self) -> Question {
        self.question.clone()
    }

    /// Gets the answer field.
    pub fn get_answer(&self) -> Vec<ResourceRecord> {
        self.answer.clone()
    }

    /// Gets the authority field.
    pub fn get_authority(&self) -> Vec<ResourceRecord> {
        self.authority.clone()
    }

    /// Gets the additional field.
    pub fn get_additional(&self) -> Vec<ResourceRecord> {
        self.additional.clone()
    }

    /// Gets the id from the header.
    pub fn get_query_id(&self) -> u16 {
        self.get_header().get_id()
    }
}

// Setters
impl DnsMessage {
    /// Sets the header field with a new Header.
    pub fn set_header(&mut self, header: Header) {
        self.header = header;
    }

    /// Sets the question field with a new Question.
    pub fn set_question(&mut self, question: Question) {
        self.question = question;
    }

    /// Sets the answer field with a new `Vec<ResourceRecord>`.
    pub fn set_answer(&mut self, answer: Vec<ResourceRecord>) {
        self.answer = answer;
    }

    /// Sets the authority field with a new `Vec<ResourceRecord>`.
    pub fn set_authority(&mut self, authority: Vec<ResourceRecord>) {
        self.authority = authority;
    }

    /// Sets the additional field with a new `Vec<ResourceRecord>`.
    pub fn set_additional(&mut self, additional: Vec<ResourceRecord>) {
        self.additional = additional;
    }

    /// Sets the id from the header with new value.
    pub fn set_query_id(&mut self, id: u16) {
        let mut header = self.get_header();
        header.set_id(id);
        self.set_header(header);
    }
}

/// Constructs and returns a new `DnsMessage` that represents a recursive query message.
///
/// This function is primarily used by the `AsyncResolver` to generate a query message
/// with default parameters that are suitable for a Stub Resolver. A Stub Resolver is a type of DNS resolver
/// that is designed to query DNS servers directly, without any caching or additional logic.
///
/// Given a `name`, `record_type`, and `record_class`, this function will create a new `DnsMessage`.
/// The resulting `DnsMessage` will have a randomly generated `query_id`. This is a unique identifier for the query
/// that allows the response to be matched up with the query. The `rd` (Recursion Desired) field is set to `true`,
/// indicating to the DNS server that it should perform a recursive query if necessary to fulfill the request.
///
/// This function does not perform the DNS query itself; it merely constructs the `DnsMessage` that 
/// represents the query.
pub fn create_recursive_query(
    name: DomainName,
    record_type: Rrtype,
    record_class: Rclass,
) -> DnsMessage {
    let mut random_generator = thread_rng();
    let query_id: u16 = random_generator.gen();
    let query = DnsMessage::new_query_message(
        name.clone(),
        record_type,
        record_class,
        0,
        true,
        query_id
    );
    return query;
}

/// Constructs a `DnsMessage` that represents a server failure response.
///
/// This function is primarily used by the `Resolution` to generate a server failure response message
/// based on a given query message. This can be useful in scenarios where a default response is needed before
/// an actual response is received from the DNS server.
///
/// The `query` parameter is a reference to a `DnsMessage` that represents the original query.
/// The resulting `DnsMessage` will have the same fields as the original query, except for the header. The header
/// is modified as follows:
/// - The `rcode` (Response Code) field is set to 2, which represents a server failure. This indicates to the client
///   that the DNS server was unable to process the query due to a problem with the server.
/// - The `qr` (Query/Response) field is set to `true`, indicating that this `DnsMessage` is a response, not a query.
///
/// This function returns the modified `DnsMessage`. Note that this function does not send the response; it merely
/// constructs the `DnsMessage` that represents the response.
///
/// # Example
///
/// ```rust
/// let query = DnsMessage::new();
/// let response = create_server_failure_response_from_query(&query);
/// ```
pub fn create_server_failure_response_from_query(
    query: &DnsMessage,
) -> DnsMessage {
    let mut response = query.clone();
    let mut new_header: Header = response.get_header();
    new_header.set_rcode(Rcode::SERVFAIL);
    new_header.set_qr(true);
    response.set_header(new_header);
    return response;
}

#[cfg(test)]
mod message_test {
    use super::*;
    use crate::domain_name::DomainName;
    use crate::message::header::Header;
    use crate::message::question::Question;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::txt_rdata::TxtRdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;
    use crate::message::DnsMessage;
    use crate::message::Rclass;
    use crate::message::Rrtype;

    #[test]
    fn constructor_test() {
        let dns_query_message =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Rrtype::A,
                Rclass::IN,
                0,
                false,
                1);

        assert_eq!(dns_query_message.header.get_rd(), false);
        assert_eq!(u16::from(dns_query_message.question.get_rrtype()), 1);
        assert_eq!(u16::from(dns_query_message.question.get_rclass()), 1);
        assert_eq!(
            dns_query_message.question.get_qname().get_name(),
            "example.com".to_string()
        );
    }

    #[test]
    fn set_and_get_header() {
        let mut header = Header::new();
        header.set_rd(true);

        let mut dns_query_message =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Rrtype::A,
                Rclass::IN,
                0,
                false,
                1);

        assert_eq!(dns_query_message.get_header().get_rd(), false);

        dns_query_message.set_header(header);

        assert_eq!(dns_query_message.get_header().get_rd(), true);
    }

    #[test]
    fn set_and_get_question() {
        let mut question = Question::new();
        question.set_rclass(Rclass::CS);

        let mut dns_query_message =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Rrtype::A,
                Rclass::IN,
                0,
                false,
                1);

        assert_eq!(u16::from(dns_query_message.get_question().get_rclass()), 1);

        dns_query_message.set_question(question);

        assert_eq!(u16::from(dns_query_message.get_question().get_rclass()), 2);
    }

    #[test]
    fn set_and_get_answer() {
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        answer.push(resource_record);

        let mut dns_query_message =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Rrtype::A,
                Rclass::IN,
                0,
                false,
                1);

        assert_eq!(dns_query_message.get_answer().len(), 0);

        dns_query_message.set_answer(answer);

        assert_eq!(dns_query_message.get_answer().len(), 1);
    }

    #[test]
    fn set_and_get_authority() {
        let mut authority: Vec<ResourceRecord> = Vec::new();
        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        authority.push(resource_record);

        let mut dns_query_message =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Rrtype::A,
                Rclass::IN,
                0,
                false,
                1);

        assert_eq!(dns_query_message.get_authority().len(), 0);

        dns_query_message.set_authority(authority);

        assert_eq!(dns_query_message.get_authority().len(), 1);
    }

    #[test]
    fn set_and_get_additional() {
        let mut additional: Vec<ResourceRecord> = Vec::new();
        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        additional.push(resource_record);

        let mut dns_query_message =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Rrtype::A,
                Rclass::IN,
                0,
                false,
                1);

        assert_eq!(dns_query_message.get_additional().len(), 0);

        dns_query_message.set_additional(additional);

        assert_eq!(dns_query_message.get_additional().len(), 1);
    }

    //ToDo: Revisar Pŕactica 1
    #[test]
    fn from_bytes_test() {
        /*let bytes: [u8; 50] = [ //format error with this one
            0b00100100, 0b10010101, 0b10010010, 0b00001000, 0, 0, 0b00000000, 0b00000001, 0, 0, 0,
            0, 4, 116, 101, 115, 116, 3, 99, 111, 109, 0, 0, 5, 0, 2, 3, 100, 99, 99, 2, 99, 108,
            0, 0, 16, 0, 1, 0, 0, 0b00010110, 0b00001010, 0, 6, 5, 104, 101, 108, 108, 111,
        ];
        */
        let bytes: [u8; 50] = [
            //test passes with this one
            0b00100100, 0b10010101, 0b10010010, 0b00100000, 0, 1, 0b00000000, 1, 0, 0, 0, 0, 4, 116,
            101, 115, 116, 3, 99, 111, 109, 0, 0, 16, 0, 1, 3, 100, 99, 99, 2, 99, 108, 0, 0, 16, 0,
            1, 0, 0, 0b00010110, 0b00001010, 0, 6, 5, 104, 101, 108, 108, 111,
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

        assert_eq!(header.get_ad(), true);
        assert_eq!(header.get_rcode(), Rcode::NOERROR);

        assert_eq!(header.get_ancount(), 1);

        // Question
        assert_eq!(question.get_qname().get_name(), String::from("test.com"));
        assert_eq!(u16::from(question.get_rrtype()), 16);
        assert_eq!(u16::from(question.get_rclass()), 1);

        // Answer
        assert_eq!(answer.len(), 1);

        assert_eq!(answer[0].get_name().get_name(), String::from("dcc.cl"));
        assert_eq!(u16::from(answer[0].get_rtype()), 16);
        assert_eq!(u16::from(answer[0].get_rclass()), 1);
        assert_eq!(answer[0].get_ttl(), 5642);
        assert_eq!(answer[0].get_rdlength(), 6);
        assert_eq!(
            match answer[0].get_rdata() {
                Rdata::TXT(val) => val.get_text(),
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

        header.set_ad(true);
        header.set_rcode(Rcode::UNKNOWN(8));

        header.set_ancount(0b0000000000000001);
        header.set_qdcount(1);

        let mut question = Question::new();

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test.com"));

        question.set_qname(domain_name);
        question.set_rrtype(Rrtype::CNAME);
        question.set_rclass(Rclass::CS);

        let txt_rdata = Rdata::TXT(TxtRdata::new(vec!["hello".to_string()]));
        let mut resource_record = ResourceRecord::new(txt_rdata);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("dcc.cl"));

        resource_record.set_name(domain_name);
        resource_record.set_type_code(Rrtype::TXT);
        resource_record.set_rclass(Rclass::IN);
        resource_record.set_ttl(5642);
        resource_record.set_rdlength(6);

        let answer = vec![resource_record];

        let mut dns_msg = DnsMessage {
            header: header,
            question: question,
            answer: answer,
            authority: Vec::new(),
            additional: Vec::new(),
        };

        dns_msg.update_header_counters();

        let msg_bytes = &dns_msg.to_bytes();

        let real_bytes: [u8; 50] = [
            0b00100100, 0b10010101, 0b10010010, 0b00101000, 0, 1, 0b00000000, 0b00000001, 0, 0, 0,
            0, 4, 116, 101, 115, 116, 3, 99, 111, 109, 0, 0, 5, 0, 2, 3, 100, 99, 99, 2, 99, 108,
            0, 0, 16, 0, 1, 0, 0, 0b00010110, 0b00001010, 0, 6, 5, 104, 101, 108, 108, 111,
        ];

        let mut i = 0;

        for value in msg_bytes {
            assert_eq!(*value, real_bytes[i]);
            i += 1;
        }
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn set_and_get_query_id_test() {
        let mut dns_message = DnsMessage::new();
        assert_eq!(dns_message.get_query_id(), 0 as u16);
        dns_message.set_query_id(23 as u16);
        assert_eq!(dns_message.get_query_id(), 23 as u16);
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn add_answers_test() {
        let mut dns_query_message =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Rrtype::A,
                Rclass::IN,
                0,
                false,
                1);

        let mut new_answer = Vec::<ResourceRecord>::new();
        let a_rdata = Rdata::A(ARdata::new());
        let rr = ResourceRecord::new(a_rdata);
        new_answer.push(rr);

        let res1 = dns_query_message.get_answer().len();
        assert_eq!(res1, 0);

        dns_query_message.add_answers(new_answer);

        let res2 = dns_query_message.get_answer().len();
        assert_eq!(res2, 1);
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn format_error_msg_test() {
        let msg = DnsMessage::format_error_msg();

        let header = msg.get_header();
        //only two things are set in this fn
        assert_eq!(header.get_rcode(), Rcode::FORMERR);
        assert_eq!(header.get_qr(), true);
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn axfr_query_message_test() {
        let dns_message = DnsMessage::axfr_query_message(
            DomainName::new_from_string("example.com".to_string()));

        assert_eq!(
            dns_message.get_question().get_qname().get_name(),
            String::from("example.com")
        );
        assert_eq!(u16::from(dns_message.get_question().get_rrtype()), 252);
        assert_eq!(u16::from(dns_message.get_question().get_rclass()), 1);
        assert_eq!(dns_message.get_header().get_op_code(), 0);
        assert_eq!(dns_message.get_header().get_rd(), false);
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn not_implemented_msg_test() {
        let msg = DnsMessage::not_implemented_msg();

        let header = msg.get_header();

        assert_eq!(header.get_rcode(), Rcode::NOTIMP);
        assert_eq!(header.get_qr(), true);
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn data_not_found_error_msg_test() {
        let msg = DnsMessage::data_not_found_error_msg();

        let header = msg.get_header();

        assert_eq!(header.get_aa(), true);
        assert_eq!(header.get_qr(), true);
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn update_header_counters_test() {
        let name = DomainName::new_from_string("example.com".to_string());
        let mut dns_query_message =
            DnsMessage::new_query_message(
                name.clone(),
                Rrtype::A,
                Rclass::IN,
                0,
                false,
                1);

        assert_eq!(dns_query_message.get_header().get_ancount(), 0);
        assert_eq!(dns_query_message.get_header().get_nscount(), 0);
        assert_eq!(dns_query_message.get_header().get_arcount(), 0);

        let mut new_answer = Vec::<ResourceRecord>::new();
        let a_rdata = Rdata::A(ARdata::new());
        let mut rr = ResourceRecord::new(a_rdata);
        rr.set_name(name);
        new_answer.push(rr);

        let a_rdata1 = Rdata::A(ARdata::new());
        let rr1 = ResourceRecord::new(a_rdata1);
        new_answer.push(rr1);

        let a_rdata2 = Rdata::A(ARdata::new());
        let rr2 = ResourceRecord::new(a_rdata2);
        new_answer.push(rr2);
        dns_query_message.set_answer(new_answer);

        let mut new_authority = Vec::<ResourceRecord>::new();
        let a_rdata3 = Rdata::A(ARdata::new());
        let rr3 = ResourceRecord::new(a_rdata3);
        new_authority.push(rr3);

        let a_rdata4 = Rdata::A(ARdata::new());
        let rr4 = ResourceRecord::new(a_rdata4);
        new_authority.push(rr4);
        dns_query_message.set_authority(new_authority);

        let mut new_additional = Vec::<ResourceRecord>::new();
        let a_rdata5 = Rdata::A(ARdata::new());
        let rr5 = ResourceRecord::new(a_rdata5);
        new_additional.push(rr5);
        dns_query_message.set_additional(new_additional);

        dns_query_message.update_header_counters();

        assert_eq!(dns_query_message.get_header().get_ancount(), 3);
        assert_eq!(dns_query_message.get_header().get_nscount(), 2);
        assert_eq!(dns_query_message.get_header().get_arcount(), 1);
    }
    //ToDo: Revisar Práctica 1
    #[test]
    fn add_authorities_test() {
        let mut dns_query_message =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Rrtype::A,
                Rclass::IN,
                0,
                false,
                1);

        let mut new_authority = Vec::<ResourceRecord>::new();
        let a_rdata3 = Rdata::A(ARdata::new());
        let rr3 = ResourceRecord::new(a_rdata3);
        new_authority.push(rr3);

        assert_eq!(dns_query_message.get_answer().len(), 0);

        dns_query_message.add_authorities(new_authority);
        //since the new authority is added to the answer lets check if something was added
        assert_eq!(dns_query_message.get_answer().len(), 1);
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn add_additionals_test() {
        let mut dns_query_message =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Rrtype::A,
                Rclass::IN,
                0,
                false,
                1);
                
        let mut new_additional = Vec::<ResourceRecord>::new();
        let a_rdata5 = Rdata::A(ARdata::new());
        let rr5 = ResourceRecord::new(a_rdata5);
        new_additional.push(rr5);

        assert_eq!(dns_query_message.get_answer().len(), 0);

        dns_query_message.add_additionals(new_additional);
        //since the new additional is added to the answer lets check if something was added
        assert_eq!(dns_query_message.get_additional().len(), 1);
    }

    //ToDo: Revisar
    #[test]
    fn new_response_message(){
        let new_response = DnsMessage::new_response_message(String::from("test.com"), "NS", "IN", 1, true, 1);

        let header = new_response.get_header();
        let id = header.get_id();
        let op_code = header.get_op_code();
        let rd = header.get_rd();

        let question = new_response.get_question();
        let qname = question.get_qname().get_name();
        let rrtype = question.get_rrtype();
        let rclass = question.get_rclass();

        assert_eq!(id, 1);
        assert_eq!(op_code, 1);
        assert!(rd);
        assert_eq!(qname, String::from("test.com"));
        assert_eq!(u16::from(rrtype), 2);
        assert_eq!(u16::from(rclass), 1);
    }

    //TODO: Revisar
    #[test]
    fn get_question_rrtype_a(){
        let name:DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_message = DnsMessage::new_query_message(name, Rrtype::A, Rclass::IN, 1, true, 1);

        let rrtype = dns_message.get_question().get_rrtype().to_string();

        assert_eq!(rrtype, String::from("A"));
    }

    //TODO: Revisar
    #[test]
    fn get_question_rrtype_ns(){
        let name:DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_message = DnsMessage::new_query_message(name, Rrtype::NS, Rclass::IN, 1, true, 1);

        let rrtype = dns_message.get_question().get_rrtype().to_string();

        assert_eq!(rrtype, String::from("NS"));
    }

    //TODO: Revisar
    #[test]
    fn get_question_rrtype_cname(){
        let name:DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_message = DnsMessage::new_query_message(name, Rrtype::CNAME, Rclass::IN, 1, true, 1);

        let rrtype = dns_message.get_question().get_rrtype().to_string();

        assert_eq!(rrtype, String::from("CNAME"));
    }

    //ToDo: Revisar
    #[test]
    fn get_question_rrtype_soa(){
        let name:DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_message = DnsMessage::new_query_message(name, Rrtype::SOA, Rclass::IN, 1, true, 1);

        let rrtype = dns_message.get_question().get_rrtype().to_string();

        assert_eq!(rrtype, String::from("SOA"));
    }

    //ToDo: Revisar
    #[test]
    fn get_question_rrtype_wks(){
        let name:DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_message = DnsMessage::new_query_message(name, Rrtype::WKS, Rclass::IN, 1, true, 1);

        let rrtype = dns_message.get_question().get_rrtype().to_string();

        assert_eq!(rrtype, String::from("WKS"));
    }

    //ToDo: Revisar
    #[test]
    fn get_question_rrtype_ptr(){
        let name:DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_message = DnsMessage::new_query_message(name, Rrtype::PTR, Rclass::IN, 1, true, 1);

        let rrtype = dns_message.get_question().get_rrtype().to_string();

        assert_eq!(rrtype, String::from("PTR"));
    }

    //ToDo: Revisar
    #[test]
    fn get_question_rrtype_hinfo(){
        let name:DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_message = DnsMessage::new_query_message(name, Rrtype::HINFO, Rclass::IN, 1, true, 1);

        let rrtype = dns_message.get_question().get_rrtype().to_string();

        assert_eq!(rrtype, String::from("HINFO"));
    }

    //ToDo: Revisar
    #[test]
    fn get_question_rrtype_minfo(){
        let name:DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_message = DnsMessage::new_query_message(name, Rrtype::MINFO, Rclass::IN, 1, true, 1);

        let rrtype = dns_message.get_question().get_rrtype().to_string();

        assert_eq!(rrtype, String::from("MINFO"));
    }

    //ToDo: Revisar
    #[test]
    fn get_question_rrtype_mx(){
        let name:DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_message = DnsMessage::new_query_message(name, Rrtype::MX, Rclass::IN, 1, true, 1);

        let rrtype = dns_message.get_question().get_rrtype().to_string();

        assert_eq!(rrtype, String::from("MX"));
    }

    //ToDo: Revisar
    #[test]
    fn get_question_rrtype_txt(){
        let name:DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_message = DnsMessage::new_query_message(name, Rrtype::TXT, Rclass::IN, 1, true, 1);

        let rrtype = dns_message.get_question().get_rrtype().to_string();

        assert_eq!(rrtype, String::from("TXT"));
    }

    #[test]
    fn check_op_code_no_error(){
        let dns_query_message =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Rrtype::A,
                Rclass::IN,
                0,
                false,
                1);
        let result = dns_query_message.check_op_code().unwrap();
        assert_eq!(result, ());
    }

    #[test]
    fn check_op_code_error(){
        let dns_query_message =
            DnsMessage::new_query_message(
                DomainName::new_from_string(" ".to_string()),
                Rrtype::AXFR,
                Rclass::IN,
                1,
                false,
                1);
        let result = dns_query_message.check_op_code().unwrap_err();
        assert_eq!(result, "IQuery not Implemented");
    }

    #[test]
    fn create_recursive_query_with_rd() {
        let name = DomainName::new_from_str("www.example.com.");
        let record_type = Rrtype::A;
        let record_class = Rclass::IN;

        let query = create_recursive_query(name.clone(), record_type, record_class);

        assert_eq!(query.get_question().get_qname(), name);
        assert_eq!(query.get_question().get_rrtype(), record_type);
        assert_eq!(query.get_question().get_rclass(), record_class);
        assert!(query.get_header().get_rd());
        assert_eq!(query.get_header().get_qr(), false);
    }

    #[test]
    fn server_failure_response_from_query_construction() {
        let name = DomainName::new_from_str("www.example.com.");
        let record_type = Rrtype::A;
        let record_class = Rclass::IN;

        let query = create_recursive_query(name.clone(), record_type, record_class);

        let response = create_server_failure_response_from_query(&query);

        assert_eq!(response.get_question().get_qname(), name);
        assert_eq!(response.get_question().get_rrtype(), record_type);
        assert_eq!(response.get_question().get_rclass(), record_class);    
        assert_eq!(response.get_header().get_rcode(), Rcode::SERVFAIL);
        assert!(response.get_header().get_qr());
    }

    #[test]
    fn add_edns0(){
        let mut dns_query_message =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Rrtype::A,
                Rclass::IN,
                0,
                false,
                1);

        dns_query_message.add_edns0(None, 0, 32768, Some(vec![12]));

        let additional = dns_query_message.get_additional();

        assert_eq!(additional.len(), 1);

        let rr = &additional[0];

        assert_eq!(rr.get_name().get_name(), String::from("."));

        assert_eq!(rr.get_rtype(), Rrtype::OPT);

        assert_eq!(rr.get_rclass(), Rclass::UNKNOWN(512));

        assert_eq!(rr.get_ttl(), 32768);

        assert_eq!(rr.get_rdlength(), 4);

        let rdata = rr.get_rdata();

        match rdata {
            Rdata::OPT(opt) => {
                let options = opt.get_option();
                for option in options {
                    assert_eq!(option, (OptionCode::PADDING, 0, Vec::new()));
                }
            },
            _ => {}

        }
    }
}
