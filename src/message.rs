pub mod header;
pub mod question;
pub mod rdata;
pub mod resource_record;
pub mod type_rtype;
pub mod type_qtype;
pub mod class_rclass;
pub mod class_qclass;

use crate::message::class_qclass::Qclass;
use crate::message::class_rclass::Rclass;
use crate::message::type_qtype::Qtype;
use crate::message::type_rtype::Rtype;
use crate::domain_name::DomainName;
use crate::message::header::Header;
use crate::message::question::Question;
use crate::message::rdata::Rdata;
use crate::message::resource_record::ResourceRecord;
use rand::thread_rng;
use rand::Rng;
use core::fmt;
use std::vec::Vec;

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
    /// DnsMessage::new_query_message(DomainName::new_from_str("example.com".to_string()), Qtype::A, Qclass:IN, 0, false);
    ///
    /// assert_eq!(dns_query_message.header.get_rd(), false);
    /// assert_eq!(dns_query_message.question.get_qtype(), Qtype::A);
    /// assert_eq!(dns_query_message.question.get_qclass(), Qclass::IN);
    /// assert_eq!(
    ///     dns_query_message.question.get_qname().get_name(),
    ///     "example.com".to_string()
    /// );
    /// ```
    ///
    pub fn new_query_message(
        qname: DomainName,
        qtype: Qtype,
        qclass: Qclass,
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
    /// let qtype = question.get_qtype();
    /// let qclass = question.get_qclass();
    /// 
    /// assert_eq!(id, 1);
    /// assert_eq!(op_code, 1);
    /// assert!(rd);
    /// assert_eq!(qname, String::from("test.com"));
    /// assert_eq!(Rtype::from_rtype_to_int(qtype), 2);
    /// assert_eq!(Rclass::from_rclass_to_int(qclass), 1);
    /// ```
    pub fn new_response_message(
        qname: String,
        qtype: &str,
        qclass: &str,
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
        let qtype_qtype = Qtype::from_str_to_qtype(qtype);
        question.set_qtype(qtype_qtype);
        let qclass_qclass = Qclass::from_str_to_qclass(qclass);
        question.set_qclass(qclass_qclass);

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

        header.set_rcode(1);
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
    /// let qtype = question.get_qtype();
    /// let qclass = question.get_qclass();
    /// 
    /// assert_eq!(id, 1);
    /// assert!(qr);
    /// assert_eq!(opcode, 0);
    /// assert!(rd);
    /// assert_eq!(qdcount, 1);
    /// assert_eq!(qname, String::from("test.com"));
    /// assert_eq!(Rtype::from_rtype_to_int(qtype), 252);
    /// assert_eq!(Rclass::from_rclass_to_int(qclass), 1);
    /// ```
    pub fn axfr_query_message(qname: DomainName) -> Self {
        let mut rng = thread_rng();
        let msg_id = rng.gen();

        let msg = DnsMessage::new_query_message(qname, Qtype::AXFR, Qclass::IN, 0, false, msg_id);

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
        header.set_rcode(4);
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
        let dns_message = DnsMessage {
            header: header,
            question: question,
            answer: answer,
            authority: authority,
            additional: additional,
        };

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
        self.header.set_arcount(self.header.get_arcount() + 1);
        self.set_additional(msg_additionals);
    }


    /// Print the information of DNS message
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
    /// msg.print_dns_message();
    /// ```
    pub fn print_dns_message(&mut self) {
        // Get the message and print the information
        let header = self.get_header();
        let answers = self.get_answer();
        let authority = self.get_authority();
        let additional = self.get_additional();

        let answer_count = header.get_ancount();
        let authority_count = header.get_nscount();
        let additional_count = header.get_arcount();

        // Not data found error
        if answer_count == 0 && header.get_qr() == true {
            if header.get_aa() == true && header.get_rcode() == 3 {
                println!("Name Error: domain name referenced in the query does not exist.");
            } else if header.get_rcode() != 0 {
                match header.get_rcode() {
                1 => println!("Format Error: The name server was unable to interpret the query."),
                2 => println!("Server Failure: The name server was unable to process this query due to a problem with the name server."),
                4 => println!("Not implemented: The name server does not support the requested kind of query."),
                5 => println!("Refused: The name server refuses to perform the specified operation for policy reasons."),
                _ => println!("Response with error code {}", header.get_rcode()), 
            }
            } else if header.get_aa() == true && header.get_rcode() == 0 {
                println!("Data not found error: The domain name referenced in the query exists, but data of the appropiate type does not.");
            }
        } else {
            println!("-------------------------------------");
            println!(
                "Answers: {} - Authority: {} - Additional: {}",
                answer_count, authority_count, additional_count
            );
            println!("-------------------------------------");

            for answer in answers {
                match answer.get_rdata() {
                    Rdata::A(val) => {
                        println!("Ip Address: {}", val.get_string_address())
                    }
                    Rdata::ACH(val) => {
                        println!(
                            "Domain name: {} - Ch Ip address: {}",
                            val.get_domain_name().get_name(),
                            val.get_ch_address()
                        )
                    }
                    Rdata::NS(val) => {
                        println!("Name Server: {}", val.get_nsdname().get_name())
                    }
                    Rdata::CNAME(val) => {
                        println!("Cname: {}", val.get_cname().get_name())
                    }
                    Rdata::HINFO(val) => {
                        println!("CPU: {} - OS: {}", val.get_cpu(), val.get_os())
                    }
                    Rdata::MX(val) => {
                        println!(
                            "Preference: {} - Exchange: {}",
                            val.get_preference(),
                            val.get_exchange().get_name()
                        )
                    }
                    Rdata::PTR(val) => {
                        println!("Ptr name: {}", val.get_ptrdname().get_name())
                    }
                    Rdata::SOA(val) => {
                        println!("Mname: {} - Rname: {} - Serial: {} - Refresh: {} - Retry: {} - Expire: {} - Minimum: {}", val.get_mname().get_name(), val.get_rname().get_name(), val.get_serial(), val.get_refresh(), val.get_retry(), val.get_expire(), val.get_minimum())
                    }
                    Rdata::TXT(val) => {
                        println!("Txt: {:#?}", val.get_text())
                    }

                    Rdata::AAAA(val) => {
                        println!("Ip Address: {}", val.get_address_as_string())
                    }

                    Rdata::TSIG(_val) => {
                    }

                    Rdata::OPT(_val) => {
                        println!("OPT code: {} - OPT length: {} - OPT data: {:#?}", _val.get_option_code(), _val.get_option_length(), _val.get_option_data())
                    }
                    Rdata::DS(val) => {
                        println!("DS key tag: {} - DS algorithm: {} - DS digest type: {} - DS digest: {:#?}", val.get_key_tag(), val.get_algorithm(), val.get_digest_type(), val.get_digest())
                    }
                    Rdata::RRSIG(val) => {
                        println!("RRSIG type covered: {} - RRSIG algorithm: {} - RRSIG labels: {} - RRSIG original TTL: {} - RRSIG signature expiration: {} - RRSIG signature inception: {} - RRSIG key tag: {} - RRSIG signer's name: {} - RRSIG signature: {:#?}", val.get_type_covered().to_string(), val.get_algorithm(), val.get_labels(), val.get_original_ttl(), val.get_signature_expiration(), val.get_signature_inception(), val.get_key_tag(), val.get_signer_name().get_name(), val.get_signature())
                    }
                    Rdata::NSEC(val) => {
                        println!("NSEC next domain name: {} - NSEC type bit maps: {:#?}", val.get_next_domain_name().get_name(), val.get_type_bit_maps())
                    }
                    Rdata::DNSKEY(val) => {
                        println!("DNSKEY flags: {} - DNSKEY protocol: {} - DNSKEY algorithm: {} - DNSKEY public key: {:#?}", val.get_flags(), val.get_protocol(), val.get_algorithm(), val.get_public_key())
                    }

                    Rdata::NSEC3(val) => {
                        println!("NSEC3 hash algorithm: {} - NSEC3 flags: {} - NSEC3 iterations: {} - NSEC3 salt: {:#?} - NSEC3 next hash: {} - NSEC3 type bit maps: {:#?}", val.get_hash_algorithm(), val.get_flags(), val.get_iterations(), val.get_salt(), val.get_next_hashed_owner_name(), val.get_type_bit_maps())
                    }
                    Rdata::NSEC3PARAM(val) => {
                        println!("NSEC3PARAM hash algorithm: {} - NSEC3PARAM flags: {} - NSEC3PARAM iterations: {} - NSEC3PARAM salt: {:#?}", val.get_hash_algorithm(), val.get_flags(), val.get_iterations(), val.get_salt())
                    }
                }
            }

            for answer in authority {
                match answer.get_rdata() {
                    Rdata::A(val) => {
                        println!("Ip Address: {}", val.get_string_address())
                    }
                    Rdata::ACH(val) => {
                        println!(
                            "Domain name: {} - Ch Ip address: {}",
                            val.get_domain_name().get_name(),
                            val.get_ch_address()
                        )
                    }
                    Rdata::NS(val) => {
                        println!("Name Server: {}", val.get_nsdname().get_name())
                    }
                    Rdata::CNAME(val) => {
                        println!("Cname: {}", val.get_cname().get_name())
                    }
                    Rdata::HINFO(val) => {
                        println!("CPU: {} - OS: {}", val.get_cpu(), val.get_os())
                    }
                    Rdata::MX(val) => {
                        println!(
                            "Preference: {} - Exchange: {}",
                            val.get_preference(),
                            val.get_exchange().get_name()
                        )
                    }
                    Rdata::PTR(val) => {
                        println!("Ptr name: {}", val.get_ptrdname().get_name())
                    }
                    Rdata::SOA(val) => {
                        println!("Mname: {} - Rname: {} - Serial: {} - Refresh: {} - Retry: {} - Expire: {} - Minimum: {}", val.get_mname().get_name(), val.get_rname().get_name(), val.get_serial(), val.get_refresh(), val.get_retry(), val.get_expire(), val.get_minimum())
                    }
                    Rdata::TXT(val) => {
                        println!("Txt: {:#?}", val.get_text())
                    }

                    Rdata::AAAA(val) => {
                        println!("Ip Address: {}", val.get_address_as_string())
                    }

                    Rdata::TSIG(_val) => {
                    }
                    Rdata::OPT(_val) => {
                        println!("OPT code: {} - OPT length: {} - OPT data: {:#?}", _val.get_option_code(), _val.get_option_length(), _val.get_option_data())
                    }
                    Rdata::RRSIG(val) => {
                        println!("RRSIG type covered: {} - RRSIG algorithm: {} - RRSIG labels: {} - RRSIG original TTL: {} - RRSIG signature expiration: {} - RRSIG signature inception: {} - RRSIG key tag: {} - RRSIG signer's name: {} - RRSIG signature: {:#?}", val.get_type_covered().to_string(), val.get_algorithm(), val.get_labels(), val.get_original_ttl(), val.get_signature_expiration(), val.get_signature_inception(), val.get_key_tag(), val.get_signer_name().get_name(), val.get_signature())
                    }
                    Rdata::DS(val) => {
                        println!("DS key tag: {} - DS algorithm: {} - DS digest type: {} - DS digest: {:#?}", val.get_key_tag(), val.get_algorithm(), val.get_digest_type(), val.get_digest())
                    }
                    Rdata::NSEC(val) => {
                        println!("NSEC next domain name: {} - NSEC type bit maps: {:#?}", val.get_next_domain_name().get_name(), val.get_type_bit_maps())
                    }
                    Rdata::DNSKEY(val) => {
                        println!("DNSKEY flags: {} - DNSKEY protocol: {} - DNSKEY algorithm: {} - DNSKEY public key: {:#?}", val.get_flags(), val.get_protocol(), val.get_algorithm(), val.get_public_key())
                    }
                    Rdata::NSEC3(val) => {
                        println!("NSEC3 hash algorithm: {} - NSEC3 flags: {} - NSEC3 iterations: {} - NSEC3 salt: {:#?} - NSEC3 next hash: {} - NSEC3 type bit maps: {:#?}", val.get_hash_algorithm(), val.get_flags(), val.get_iterations(), val.get_salt(), val.get_next_hashed_owner_name(), val.get_type_bit_maps())
                    }
                    Rdata::NSEC3PARAM(val) => {
                        println!("NSEC3PARAM hash algorithm: {} - NSEC3PARAM flags: {} - NSEC3PARAM iterations: {} - NSEC3PARAM salt: {:#?}", val.get_hash_algorithm(), val.get_flags(), val.get_iterations(), val.get_salt())
                    }
                }
            }

            for answer in additional {
                match answer.get_rdata() {
                    Rdata::A(val) => {
                        println!("Ip Address: {}", val.get_string_address())
                    }
                    Rdata::ACH(val) => {
                        println!(
                            "Domain name: {} - Ch Ip address: {}",
                            val.get_domain_name().get_name(),
                            val.get_ch_address()
                        )
                    }
                    Rdata::NS(val) => {
                        println!("Name Server: {}", val.get_nsdname().get_name())
                    }
                    Rdata::CNAME(val) => {
                        println!("Cname: {}", val.get_cname().get_name())
                    }
                    Rdata::HINFO(val) => {
                        println!("CPU: {} - OS: {}", val.get_cpu(), val.get_os())
                    }
                    Rdata::MX(val) => {
                        println!(
                            "Preference: {} - Exchange: {}",
                            val.get_preference(),
                            val.get_exchange().get_name()
                        )
                    }
                    Rdata::PTR(val) => {
                        println!("Ptr name: {}", val.get_ptrdname().get_name())
                    }
                    Rdata::SOA(val) => {
                        println!("Mname: {} - Rname: {} - Serial: {} - Refresh: {} - Retry: {} - Expire: {} - Minimum: {}", val.get_mname().get_name(), val.get_rname().get_name(), val.get_serial(), val.get_refresh(), val.get_retry(), val.get_expire(), val.get_minimum())
                    }
                    Rdata::TXT(val) => {
                        println!("Txt: {:#?}", val.get_text())
                    }
                    Rdata::AAAA(val) => {
                        println!("Ip Address: {}", val.get_address_as_string())
                    }
                    Rdata::TSIG(_val) => {
                    }
                    Rdata::OPT(_val) => {
                        println!("OPT code: {} - OPT length: {} - OPT data: {:#?}", _val.get_option_code(), _val.get_option_length(), _val.get_option_data())
                    }
                    Rdata::DS(val) => {
                        println!("DS key tag: {} - DS algorithm: {} - DS digest type: {} - DS digest: {:#?}", val.get_key_tag(), val.get_algorithm(), val.get_digest_type(), val.get_digest())
                    }
                    Rdata::RRSIG(val) => {
                        println!("RRSIG type covered: {} - RRSIG algorithm: {} - RRSIG labels: {} - RRSIG original TTL: {} - RRSIG signature expiration: {} - RRSIG signature inception: {} - RRSIG key tag: {} - RRSIG signer's name: {} - RRSIG signature: {:#?}", val.get_type_covered().to_string(), val.get_algorithm(), val.get_labels(), val.get_original_ttl(), val.get_signature_expiration(), val.get_signature_inception(), val.get_key_tag(), val.get_signer_name().get_name(), val.get_signature())
                    }
                    Rdata::NSEC(val) => {
                        println!("NSEC next domain name: {} - NSEC type bit maps: {:#?}", val.get_next_domain_name().get_name(), val.get_type_bit_maps())
                    }
                    Rdata::DNSKEY(val) => {
                        println!("DNSKEY flags: {} - DNSKEY protocol: {} - DNSKEY algorithm: {} - DNSKEY public key: {:#?}", val.get_flags(), val.get_protocol(), val.get_algorithm(), val.get_public_key())
                    }
                    Rdata::NSEC3(val) => {
                        println!("NSEC3 hash algorithm: {} - NSEC3 flags: {} - NSEC3 iterations: {} - NSEC3 salt: {:#?} - NSEC3 next hash: {} - NSEC3 type bit maps: {:#?}", val.get_hash_algorithm(), val.get_flags(), val.get_iterations(), val.get_salt(), val.get_next_hashed_owner_name(), val.get_type_bit_maps())
                    }
                    Rdata::NSEC3PARAM(val) => {
                        println!("NSEC3PARAM hash algorithm: {} - NSEC3PARAM flags: {} - NSEC3PARAM iterations: {} - NSEC3PARAM salt: {:#?}", val.get_hash_algorithm(), val.get_flags(), val.get_iterations(), val.get_salt())
                    }
                }
            }
        }
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
        let answers = self.get_answer().into_iter();
        let authority = self.get_authority().into_iter();
        let additional = self.get_additional().into_iter();
        result.push_str(&format!("Answer\n"));
        answers.for_each(|answer| result.push_str(&format!("{}\n", answer)));
        result.push_str(&format!("Authority\n"));
        authority.for_each(|authority| result.push_str(&format!("{}\n", authority)));
        result.push_str(&format!("Additional\n"));
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

#[cfg(test)]
mod message_test {
    use crate::domain_name::DomainName;
    use crate::message::header::Header;
    use crate::message::question::Question;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::txt_rdata::TxtRdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;
    use crate::message::DnsMessage;
    use crate::message::Rclass;
    use crate::message::Qclass;
    use crate::message::Qtype;
    use crate::message::type_rtype::Rtype;

    #[test]
    fn constructor_test() {
        let dns_query_message =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::A,
                Qclass::IN,
                0,
                false,
                1);

        assert_eq!(dns_query_message.header.get_rd(), false);
        assert_eq!(Qtype::from_qtype_to_int(dns_query_message.question.get_qtype()), 1);
        assert_eq!(Qclass::from_qclass_to_int(dns_query_message.question.get_qclass()), 1);
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
                Qtype::A,
                Qclass::IN,
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
        question.set_qclass(Qclass::CS);

        let mut dns_query_message =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::A,
                Qclass::IN,
                0,
                false,
                1);

        assert_eq!(Qclass::from_qclass_to_int(dns_query_message.get_question().get_qclass()), 1);

        dns_query_message.set_question(question);

        assert_eq!(Qclass::from_qclass_to_int(dns_query_message.get_question().get_qclass()), 2);
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
                Qtype::A,
                Qclass::IN,
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
                Qtype::A,
                Qclass::IN,
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
                Qtype::A,
                Qclass::IN,
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
            0b00100100, 0b10010101, 0b10010010, 0b00000000, 0, 1, 0b00000000, 1, 0, 0, 0, 0, 4, 116,
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
        assert_eq!(header.get_rcode(), 0);
        assert_eq!(header.get_ancount(), 1);

        // Question
        assert_eq!(question.get_qname().get_name(), String::from("test.com"));
        assert_eq!(Qtype::from_qtype_to_int(question.get_qtype()), 16);
        assert_eq!(Qclass::from_qclass_to_int(question.get_qclass()), 1);

        // Answer
        assert_eq!(answer.len(), 1);

        assert_eq!(answer[0].get_name().get_name(), String::from("dcc.cl"));
        assert_eq!(Rtype::from_rtype_to_int(answer[0].get_rtype()), 16);
        assert_eq!(Rclass::from_rclass_to_int(answer[0].get_rclass()), 1);
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

        let dns_msg = DnsMessage {
            header: header,
            question: question,
            answer: answer,
            authority: Vec::new(),
            additional: Vec::new(),
        };

        let msg_bytes = &dns_msg.to_bytes();

        let real_bytes: [u8; 50] = [
            0b00100100, 0b10010101, 0b10010010, 0b00001000, 0, 1, 0b00000000, 0b00000001, 0, 0, 0,
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
                Qtype::A,
                Qclass::IN,
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
        assert_eq!(header.get_rcode(), 1);
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
        assert_eq!(Qtype::from_qtype_to_int(dns_message.get_question().get_qtype()), 252);
        assert_eq!(Qclass::from_qclass_to_int(dns_message.get_question().get_qclass()), 1);
        assert_eq!(dns_message.get_header().get_op_code(), 0);
        assert_eq!(dns_message.get_header().get_rd(), false);
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn not_implemented_msg_test() {
        let msg = DnsMessage::not_implemented_msg();

        let header = msg.get_header();

        assert_eq!(header.get_rcode(), 4);
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
                Qtype::A,
                Qclass::IN,
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
        dns_query_message.print_dns_message();

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
                Qtype::A,
                Qclass::IN,
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
                Qtype::A,
                Qclass::IN,
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
        let qtype = question.get_qtype();
        let qclass = question.get_qclass();

        assert_eq!(id, 1);
        assert_eq!(op_code, 1);
        assert!(rd);
        assert_eq!(qname, String::from("test.com"));
        assert_eq!(Qtype::from_qtype_to_int(qtype), 2);
        assert_eq!(Qclass::from_qclass_to_int(qclass), 1);
    }

    //TODO: Revisar
    #[test]
    fn get_question_qtype_a(){
        let name:DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_message = DnsMessage::new_query_message(name, Qtype::A, Qclass::IN, 1, true, 1);

        let qtype = dns_message.get_question().get_qtype().to_string();

        assert_eq!(qtype, String::from("A"));
    }

    //TODO: Revisar
    #[test]
    fn get_question_qtype_ns(){
        let name:DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_message = DnsMessage::new_query_message(name, Qtype::NS, Qclass::IN, 1, true, 1);

        let qtype = dns_message.get_question().get_qtype().to_string();

        assert_eq!(qtype, String::from("NS"));
    }

    //TODO: Revisar
    #[test]
    fn get_question_qtype_cname(){
        let name:DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_message = DnsMessage::new_query_message(name, Qtype::CNAME, Qclass::IN, 1, true, 1);

        let qtype = dns_message.get_question().get_qtype().to_string();

        assert_eq!(qtype, String::from("CNAME"));
    }

    //ToDo: Revisar
    #[test]
    fn get_question_qtype_soa(){
        let name:DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_message = DnsMessage::new_query_message(name, Qtype::SOA, Qclass::IN, 1, true, 1);

        let qtype = dns_message.get_question().get_qtype().to_string();

        assert_eq!(qtype, String::from("SOA"));
    }

    //ToDo: Revisar
    #[test]
    fn get_question_qtype_wks(){
        let name:DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_message = DnsMessage::new_query_message(name, Qtype::WKS, Qclass::IN, 1, true, 1);

        let qtype = dns_message.get_question().get_qtype().to_string();

        assert_eq!(qtype, String::from("WKS"));
    }

    //ToDo: Revisar
    #[test]
    fn get_question_qtype_ptr(){
        let name:DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_message = DnsMessage::new_query_message(name, Qtype::PTR, Qclass::IN, 1, true, 1);

        let qtype = dns_message.get_question().get_qtype().to_string();

        assert_eq!(qtype, String::from("PTR"));
    }

    //ToDo: Revisar
    #[test]
    fn get_question_qtype_hinfo(){
        let name:DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_message = DnsMessage::new_query_message(name, Qtype::HINFO, Qclass::IN, 1, true, 1);

        let qtype = dns_message.get_question().get_qtype().to_string();

        assert_eq!(qtype, String::from("HINFO"));
    }

    //ToDo: Revisar
    #[test]
    fn get_question_qtype_minfo(){
        let name:DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_message = DnsMessage::new_query_message(name, Qtype::MINFO, Qclass::IN, 1, true, 1);

        let qtype = dns_message.get_question().get_qtype().to_string();

        assert_eq!(qtype, String::from("MINFO"));
    }

    //ToDo: Revisar
    #[test]
    fn get_question_qtype_mx(){
        let name:DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_message = DnsMessage::new_query_message(name, Qtype::MX, Qclass::IN, 1, true, 1);

        let qtype = dns_message.get_question().get_qtype().to_string();

        assert_eq!(qtype, String::from("MX"));
    }

    //ToDo: Revisar
    #[test]
    fn get_question_qtype_txt(){
        let name:DomainName = DomainName::new_from_string("example.com".to_string());
        let dns_message = DnsMessage::new_query_message(name, Qtype::TXT, Qclass::IN, 1, true, 1);

        let qtype = dns_message.get_question().get_qtype().to_string();

        assert_eq!(qtype, String::from("TXT"));
    }

    #[test]
    fn check_op_code_no_error(){
        let dns_query_message =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::A,
                Qclass::IN,
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
                Qtype::AXFR,
                Qclass::IN,
                1,
                false,
                1);
        let result = dns_query_message.check_op_code().unwrap_err();
        assert_eq!(result, "IQuery not Implemented");
    }

}
