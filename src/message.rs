pub mod header;
pub mod question;
pub mod rdata;
pub mod resource_record;

use crate::domain_name::DomainName;
use crate::message::header::Header;
use crate::message::question::Question;
use crate::message::rdata::Rdata;
use crate::message::resource_record::ResourceRecord;
use rand::thread_rng;
use rand::Rng;
use std::vec::Vec;

#[derive(Clone)]
// Structs that represents a dns message
pub struct DnsMessage {
    header: Header,
    question: Question,
    answer: Vec<ResourceRecord>,
    authority: Vec<ResourceRecord>,
    additional: Vec<ResourceRecord>,
}

#[derive(Clone)]
//Enum for the Class of a RR in a DnsMessage
pub enum Rclass {
    IN,
    CS,
    CH,
    HS,
    ANY,
    UNKNOWN(u16),
}

#[derive(Clone)]
//Enum For the Type of a RR in a DnsMessage with an Rdata implementation
pub enum Rtype {
    A,
    NS,
    CNAME,
    SOA,
    PTR,
    HINFO,
    MINFO,
    WKS,
    MX,
    TXT,
    ANY,
    AXFR,
    MAILB,
    MAILA,
    UNKNOWN(u16),
}

//Functions for the RType Enum
impl Rtype{
    //Function to get the int equivalent of a type
    pub fn from_rtype_to_int(rtype: Rtype) -> u16{
        match rtype {
            Rtype::A => 1,
            Rtype::NS => 2,
            Rtype::CNAME => 5,
            Rtype::SOA => 6,
            Rtype::WKS => 11,
            Rtype::PTR => 12,
            Rtype::HINFO => 13,
            Rtype::MINFO => 14,
            Rtype::MX => 15,
            Rtype::TXT => 16,
            Rtype::AXFR => 252,
            Rtype::MAILB => 253,
            Rtype::MAILA => 254,
            Rtype::ANY => 255,
            Rtype::UNKNOWN(val) => val
        }
    }
    //Function to get the String equivalent of a type
    pub fn from_rtype_to_str(rtype: Rtype) -> String {
        match rtype {
            Rtype::A => String::from("A"),
            Rtype::NS => String::from("NS"),
            Rtype::CNAME => String::from("CNAME"),
            Rtype::SOA => String::from("SOA"),
            Rtype::WKS => String::from("WKS"),
            Rtype::PTR => String::from("PTR"),
            Rtype::HINFO => String::from("HINFO"),
            Rtype::MINFO => String::from("MINFO"),
            Rtype::MX => String::from("MX"),
            Rtype::TXT => String::from("TXT"),
            Rtype::AXFR => String::from("AXFR"),
            Rtype::MAILB => String::from("MAILB"),
            Rtype::MAILA => String::from("MAILA"),
            Rtype::ANY => String::from("ANy"),
            Rtype::UNKNOWN(_val) => String::from("UNKNOWN TYPE") 
        }
    }

    //Function to get the String equivalent of a type
    pub fn from_int_to_rtype(val: u16) -> Rtype{
        match val {
            1 => Rtype::A,
            2 => Rtype::NS,
            5 => Rtype::CNAME,
            6 => Rtype::SOA,
            11 => Rtype::WKS,
            12 => Rtype::PTR,
            13 => Rtype::HINFO,
            14 => Rtype::MINFO,
            15 => Rtype::MX,
            16 => Rtype::TXT,
            252 => Rtype::AXFR,
            253 => Rtype::MAILB,
            254 => Rtype::MAILA,
            255 => Rtype::ANY,
            _ => Rtype::UNKNOWN(val),
        }
    }

    //Function to get the Rtype from a String
    pub fn from_string_to_rtype(string: String) -> Rtype{
        let to_be_matched = string.as_str();
        match to_be_matched {
            "A" => Rtype::A,
            "NS" => Rtype::NS,
            "CNAME" => Rtype::CNAME,
            "SOA" => Rtype::SOA,
            "WKS" => Rtype::WKS,
            "PTR" => Rtype::PTR,
            "HINFO" => Rtype::HINFO,
            "MINFO" => Rtype::MINFO,
            "MX" => Rtype::MX,
            "TXT" => Rtype::TXT,
            "AXFR" => Rtype::AXFR,
            "MAILB" => Rtype::MAILB,
            "MAILA" => Rtype::MAILA,
            "ANY" => Rtype::ANY,
            _ => Rtype::UNKNOWN(99),
        }
    }
}
impl Default for Rtype {
    fn default() -> Self { Rtype::A }
}


//Functions for the Rclass Enum
impl Rclass {
    //Function to get the int equivalent of a class
    pub fn from_rclass_to_int(class: Rclass) -> u16{
        match class {
            Rclass::IN => 1,
            Rclass::CS => 2,
            Rclass::CH => 3,
            Rclass::HS => 4,
            Rclass::ANY => 255,
            Rclass::UNKNOWN(val) => val,
        }
    }

    //Function to get an string representing the class
    pub fn from_rclass_to_str(class: Rclass) -> String{
        match class {
            Rclass::IN => String::from("IN"),
            Rclass::CS => String::from("CS"),
            Rclass::CH => String::from("CH"),
            Rclass::HS => String::from("HS"),
            Rclass::ANY => String::from("ANY"),
            Rclass::UNKNOWN(_val) => String::from("UNKNOWN CLASS")
        }
    }

    //Function to get the Rclass from a value
    pub fn from_int_to_rclass(val:u16) -> Rclass{
        match val {
            1 => Rclass::IN,
            2 => Rclass::CS,
            3 => Rclass::CH,
            4 => Rclass::HS,
            255 => Rclass::ANY,
            _ => Rclass::UNKNOWN(val)
        }
    }

    //Function to get the Rclass from a String
    pub fn from_string_to_rclass(string: String) -> Rclass{
        let to_be_macthed = string.as_str();
        match to_be_macthed {
            "IN" => Rclass::IN,
            "CS" => Rclass::CS,
            "CH" => Rclass::CH,
            "HS" => Rclass::HS,
            "ANY" => Rclass::ANY,
            _ => Rclass::UNKNOWN(99)
        }
    }  
}

impl Default for Rclass {
    fn default() -> Self { Rclass::IN }
}


impl DnsMessage {
    // Creates a new query message
    //
    // # Examples
    // '''
    // let dns_query_message =
    // DnsMessage::new_query_message("test.com".to_string(), 1, 1, 0, false);
    //
    // assert_eq!(dns_query_message.header.get_rd(), false);
    // assert_eq!(dns_query_message.question.get_qtype(), 1);
    // assert_eq!(dns_query_message.question.get_qclass(), 1);
    // assert_eq!(
    //     dns_query_message.question.get_qname().get_name(),
    //     "test.com".to_string()
    // );
    // '''
    //
    pub fn new_query_message(
        qname: String,
        qtype: String,
        qclass: String,
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
        let qtype_rtype = Rtype::from_string_to_rtype(qtype);
        question.set_qtype(qtype_rtype);
        let qclass_rclass = Rclass::from_string_to_rclass(qclass);
        question.set_qclass(qclass_rclass);

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
        qtype: String,
        qclass: String,
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
        let qtype_rtype = Rtype::from_string_to_rtype(qtype);
        question.set_qtype(qtype_rtype);
        let qclass_rclass = Rclass::from_string_to_rclass(qclass);
        question.set_qclass(qclass_rclass);

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

    pub fn format_error_msg() -> Self {
        let mut msg = DnsMessage::new();
        let mut header = msg.get_header();

        header.set_rcode(1);
        header.set_qr(true);
        msg.set_header(header);

        msg
    }

    pub fn data_not_found_error_msg() -> Self {
        let mut msg = DnsMessage::new();
        let mut header = msg.get_header();

        header.set_aa(true);
        header.set_qr(true);
        msg.set_header(header);

        msg
    }

    pub fn axfr_query_message(qname: String) -> Self {
        let mut rng = thread_rng();
        let msg_id = rng.gen();

        let msg = DnsMessage::new_query_message(qname, String::from("AXFR"), String::from("IN"), 0, false, msg_id);

        msg
    }

    pub fn not_implemented_msg() -> Self {
        let mut msg = DnsMessage::new();
        let mut header = msg.get_header();
        header.set_rcode(4);
        header.set_qr(true);

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

    // Print the information of DNS message
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
                    Rdata::SomeARdata(val) => {
                        println!("Ip Address: {}", val.get_string_address())
                    }
                    Rdata::SomeAChRdata(val) => {
                        println!(
                            "Domain name: {} - Ch Ip address: {}",
                            val.get_domain_name().get_name(),
                            val.get_ch_address()
                        )
                    }
                    Rdata::SomeNsRdata(val) => {
                        println!("Name Server: {}", val.get_nsdname().get_name())
                    }
                    Rdata::SomeCnameRdata(val) => {
                        println!("Cname: {}", val.get_cname().get_name())
                    }
                    Rdata::SomeHinfoRdata(val) => {
                        println!("CPU: {} - OS: {}", val.get_cpu(), val.get_os())
                    }
                    Rdata::SomeMxRdata(val) => {
                        println!(
                            "Preference: {} - Exchange: {}",
                            val.get_preference(),
                            val.get_exchange().get_name()
                        )
                    }
                    Rdata::SomePtrRdata(val) => {
                        println!("Ptr name: {}", val.get_ptrdname().get_name())
                    }
                    Rdata::SomeSoaRdata(val) => {
                        println!("Mname: {} - Rname: {} - Serial: {} - Refresh: {} - Retry: {} - Expire: {} - Minimum: {}", val.get_mname().get_name(), val.get_rname().get_name(), val.get_serial(), val.get_refresh(), val.get_retry(), val.get_expire(), val.get_minimum())
                    }
                    Rdata::SomeTxtRdata(val) => {
                        println!("Txt: {:#?}", val.get_text())
                    }
                }
            }

            for answer in authority {
                match answer.get_rdata() {
                    Rdata::SomeARdata(val) => {
                        println!("Ip Address: {}", val.get_string_address())
                    }
                    Rdata::SomeAChRdata(val) => {
                        println!(
                            "Domain name: {} - Ch Ip address: {}",
                            val.get_domain_name().get_name(),
                            val.get_ch_address()
                        )
                    }
                    Rdata::SomeNsRdata(val) => {
                        println!("Name Server: {}", val.get_nsdname().get_name())
                    }
                    Rdata::SomeCnameRdata(val) => {
                        println!("Cname: {}", val.get_cname().get_name())
                    }
                    Rdata::SomeHinfoRdata(val) => {
                        println!("CPU: {} - OS: {}", val.get_cpu(), val.get_os())
                    }
                    Rdata::SomeMxRdata(val) => {
                        println!(
                            "Preference: {} - Exchange: {}",
                            val.get_preference(),
                            val.get_exchange().get_name()
                        )
                    }
                    Rdata::SomePtrRdata(val) => {
                        println!("Ptr name: {}", val.get_ptrdname().get_name())
                    }
                    Rdata::SomeSoaRdata(val) => {
                        println!("Mname: {} - Rname: {} - Serial: {} - Refresh: {} - Retry: {} - Expire: {} - Minimum: {}", val.get_mname().get_name(), val.get_rname().get_name(), val.get_serial(), val.get_refresh(), val.get_retry(), val.get_expire(), val.get_minimum())
                    }
                    Rdata::SomeTxtRdata(val) => {
                        println!("Txt: {:#?}", val.get_text())
                    }
                }
            }

            for answer in additional {
                match answer.get_rdata() {
                    Rdata::SomeARdata(val) => {
                        println!("Ip Address: {}", val.get_string_address())
                    }
                    Rdata::SomeAChRdata(val) => {
                        println!(
                            "Domain name: {} - Ch Ip address: {}",
                            val.get_domain_name().get_name(),
                            val.get_ch_address()
                        )
                    }
                    Rdata::SomeNsRdata(val) => {
                        println!("Name Server: {}", val.get_nsdname().get_name())
                    }
                    Rdata::SomeCnameRdata(val) => {
                        println!("Cname: {}", val.get_cname().get_name())
                    }
                    Rdata::SomeHinfoRdata(val) => {
                        println!("CPU: {} - OS: {}", val.get_cpu(), val.get_os())
                    }
                    Rdata::SomeMxRdata(val) => {
                        println!(
                            "Preference: {} - Exchange: {}",
                            val.get_preference(),
                            val.get_exchange().get_name()
                        )
                    }
                    Rdata::SomePtrRdata(val) => {
                        println!("Ptr name: {}", val.get_ptrdname().get_name())
                    }
                    Rdata::SomeSoaRdata(val) => {
                        println!("Mname: {} - Rname: {} - Serial: {} - Refresh: {} - Retry: {} - Expire: {} - Minimum: {}", val.get_mname().get_name(), val.get_rname().get_name(), val.get_serial(), val.get_refresh(), val.get_retry(), val.get_expire(), val.get_minimum())
                    }
                    Rdata::SomeTxtRdata(val) => {
                        println!("Txt: {:#?}", val.get_text())
                    }
                }
            }
        }
    }
}

// Getters
impl DnsMessage {
    // Gets the header field
    pub fn get_header(&self) -> Header {
        self.header.clone()
    }

    // Gets the question field
    pub fn get_question(&self) -> Question {
        self.question.clone()
    }

    // Gets the answer field
    pub fn get_answer(&self) -> Vec<ResourceRecord> {
        self.answer.clone()
    }

    // Gets the authority field
    pub fn get_authority(&self) -> Vec<ResourceRecord> {
        self.authority.clone()
    }

    // Gets the additional field
    pub fn get_additional(&self) -> Vec<ResourceRecord> {
        self.additional.clone()
    }

    // Gets the id from the header
    pub fn get_query_id(&self) -> u16 {
        self.get_header().get_id()
    }
}

// Setters
impl DnsMessage {
    // Sets the header field with a new Header
    pub fn set_header(&mut self, header: Header) {
        self.header = header;
    }

    // Sets the question field with a new Question
    pub fn set_question(&mut self, question: Question) {
        self.question = question;
    }

    // Sets the answer field with a new Vec<ResourceRecord>
    pub fn set_answer(&mut self, answer: Vec<ResourceRecord>) {
        self.answer = answer;
    }

    // Sets the authority field with a new Vec<ResourceRecord>
    pub fn set_authority(&mut self, authority: Vec<ResourceRecord>) {
        self.authority = authority;
    }

    // Sets the additional field with a new Vec<ResourceRecord>
    pub fn set_additional(&mut self, additional: Vec<ResourceRecord>) {
        self.additional = additional;
    }

    // Sets the id from the header with new value
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
    use crate::message::Rtype;

    #[test]
    fn constructor_test() {
        let dns_query_message =
            DnsMessage::new_query_message("test.com".to_string(), String::from("A"), String::from("IN"), 0, false, 1);

        assert_eq!(dns_query_message.header.get_rd(), false);
        assert_eq!(Rtype::from_rtype_to_int(dns_query_message.question.get_qtype()), 1);
        assert_eq!(Rclass::from_rclass_to_int(dns_query_message.question.get_qclass()), 1);
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
            DnsMessage::new_query_message("test.com".to_string(), String::from("A"), String::from("IN"), 0, false, 1);

        assert_eq!(dns_query_message.get_header().get_rd(), false);

        dns_query_message.set_header(header);

        assert_eq!(dns_query_message.get_header().get_rd(), true);
    }

    #[test]
    fn set_and_get_question() {
        let mut question = Question::new();
        question.set_qclass(Rclass::CS);

        let mut dns_query_message =
            DnsMessage::new_query_message("test.com".to_string(), String::from("A"), String::from("IN"), 0, false, 1);

        assert_eq!(Rclass::from_rclass_to_int(dns_query_message.get_question().get_qclass()), 1);

        dns_query_message.set_question(question);

        assert_eq!(Rclass::from_rclass_to_int(dns_query_message.get_question().get_qclass()), 2);
    }

    #[test]
    fn set_and_get_answer() {
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        answer.push(resource_record);

        let mut dns_query_message =
            DnsMessage::new_query_message("test.com".to_string(), String::from("A"), String::from("IN"), 0, false, 1);

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
            DnsMessage::new_query_message("test.com".to_string(), String::from("A"), String::from("IN"), 0, false, 1);

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
            DnsMessage::new_query_message("test.com".to_string(), String::from("A"), String::from("IN"), 0, false, 1);

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
        assert_eq!(Rtype::from_rtype_to_int(question.get_qtype()), 16);
        assert_eq!(Rclass::from_rclass_to_int(question.get_qclass()), 1);

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
        header.set_qdcount(1);

        let mut question = Question::new();

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test.com"));

        question.set_qname(domain_name);
        question.set_qtype(Rtype::CNAME);
        question.set_qclass(Rclass::CS);

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
            DnsMessage::new_query_message(String::from("test.com"), String::from("A"), String::from("IN"), 0, false, 1);

        let mut new_answer = Vec::<ResourceRecord>::new();
        let a_rdata = Rdata::SomeARdata(ARdata::new());
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
        let dns_message = DnsMessage::axfr_query_message(String::from("example.com"));

        assert_eq!(
            dns_message.get_question().get_qname().get_name(),
            String::from("example.com")
        );
        assert_eq!(Rtype::from_rtype_to_int(dns_message.get_question().get_qtype()), 252);
        assert_eq!(Rclass::from_rclass_to_int(dns_message.get_question().get_qclass()), 1);
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
        let mut dns_query_message =
            DnsMessage::new_query_message(String::from("test.com"), String::from("A"), String::from("IN"), 0, false, 1);

        assert_eq!(dns_query_message.get_header().get_ancount(), 0);
        assert_eq!(dns_query_message.get_header().get_nscount(), 0);
        assert_eq!(dns_query_message.get_header().get_arcount(), 0);

        let mut new_answer = Vec::<ResourceRecord>::new();
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let rr = ResourceRecord::new(a_rdata);
        new_answer.push(rr);

        let a_rdata1 = Rdata::SomeARdata(ARdata::new());
        let rr1 = ResourceRecord::new(a_rdata1);
        new_answer.push(rr1);

        let a_rdata2 = Rdata::SomeARdata(ARdata::new());
        let rr2 = ResourceRecord::new(a_rdata2);
        new_answer.push(rr2);
        dns_query_message.set_answer(new_answer);

        let mut new_authority = Vec::<ResourceRecord>::new();
        let a_rdata3 = Rdata::SomeARdata(ARdata::new());
        let rr3 = ResourceRecord::new(a_rdata3);
        new_authority.push(rr3);

        let a_rdata4 = Rdata::SomeARdata(ARdata::new());
        let rr4 = ResourceRecord::new(a_rdata4);
        new_authority.push(rr4);
        dns_query_message.set_authority(new_authority);

        let mut new_additional = Vec::<ResourceRecord>::new();
        let a_rdata5 = Rdata::SomeARdata(ARdata::new());
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
            DnsMessage::new_query_message(String::from("test.com"), String::from("A"), String::from("IN"), 0, false, 1);

        let mut new_authority = Vec::<ResourceRecord>::new();
        let a_rdata3 = Rdata::SomeARdata(ARdata::new());
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
            DnsMessage::new_query_message(String::from("test.com"), String::from("A"), String::from("IN"), 0, false, 1);

        let mut new_additional = Vec::<ResourceRecord>::new();
        let a_rdata5 = Rdata::SomeARdata(ARdata::new());
        let rr5 = ResourceRecord::new(a_rdata5);
        new_additional.push(rr5);

        assert_eq!(dns_query_message.get_answer().len(), 0);

        dns_query_message.add_additionals(new_additional);
        //since the new additional is added to the answer lets check if something was added
        assert_eq!(dns_query_message.get_answer().len(), 1);
    }

    //ToDo: Revisar
    #[test]
    fn new_response_message(){
        let new_response = DnsMessage::new_response_message(String::from("test.com"), String::from("NS"), String::from("IN"), 1, true, 1);

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
        assert_eq!(Rtype::from_rtype_to_int(qtype), 2);
        assert_eq!(Rclass::from_rclass_to_int(qclass), 1);
    }

    //ToDo: Revisar
    #[test]
    fn get_question_qtype_a(){
        let name = String::from("value");
        let dns_message = DnsMessage::new_query_message(name, String::from("A"), String::from("IN"), 1, true, 1);

        let qtype = Rtype::from_rtype_to_str(dns_message.get_question().get_qtype());

        assert_eq!(qtype, String::from("A"));
    }

    //ToDo: Revisar
    #[test]
    fn get_question_qtype_ns(){
        let name = String::from("value");
        let dns_message = DnsMessage::new_query_message(name, String::from("NS"), String::from("IN"), 1, true, 1);

        let qtype = Rtype::from_rtype_to_str(dns_message.get_question().get_qtype());

        assert_eq!(qtype, String::from("NS"));
    }

    //ToDo: Revisar
    #[test]
    fn get_question_qtype_cname(){
        let name = String::from("value");
        let dns_message = DnsMessage::new_query_message(name, String::from("CNAME"), String::from("IN"), 1, true, 1);

        let qtype = Rtype::from_rtype_to_str(dns_message.get_question().get_qtype());

        assert_eq!(qtype, String::from("CNAME"));
    }

    //ToDo: Revisar
    #[test]
    fn get_question_qtype_soa(){
        let name = String::from("value");
        let dns_message = DnsMessage::new_query_message(name, String::from("SOA"), String::from("IN"), 1, true, 1);

        let qtype = Rtype::from_rtype_to_str(dns_message.get_question().get_qtype());

        assert_eq!(qtype, String::from("SOA"));
    }

    //ToDo: Revisar
    #[test]
    fn get_question_qtype_wks(){
        let name = String::from("value");
        let dns_message = DnsMessage::new_query_message(name, String::from("WKS"), String::from("IN"), 1, true, 1);

        let qtype = Rtype::from_rtype_to_str(dns_message.get_question().get_qtype());

        assert_eq!(qtype, String::from("WKS"));
    }

    //ToDo: Revisar
    #[test]
    fn get_question_qtype_ptr(){
        let name = String::from("value");
        let dns_message = DnsMessage::new_query_message(name, String::from("PTR"), String::from("IN"), 1, true, 1);

        let qtype = Rtype::from_rtype_to_str(dns_message.get_question().get_qtype());

        assert_eq!(qtype, String::from("PTR"));
    }

    //ToDo: Revisar
    #[test]
    fn get_question_qtype_hinfo(){
        let name = String::from("value");
        let dns_message = DnsMessage::new_query_message(name, String::from("HINFO"), String::from("IN"), 1, true, 1);

        let qtype = Rtype::from_rtype_to_str(dns_message.get_question().get_qtype());

        assert_eq!(qtype, String::from("HINFO"));
    }

    //ToDo: Revisar
    #[test]
    fn get_question_qtype_minfo(){
        let name = String::from("value");
        let dns_message = DnsMessage::new_query_message(name, String::from("MINFO"), String::from("IN"), 1, true, 1);

        let qtype = Rtype::from_rtype_to_str(dns_message.get_question().get_qtype());

        assert_eq!(qtype, String::from("MINFO"));
    }

    //ToDo: Revisar
    #[test]
    fn get_question_qtype_mx(){
        let name = String::from("value");
        let dns_message = DnsMessage::new_query_message(name, String::from("MX"), String::from("IN"), 1, true, 1);

        let qtype = Rtype::from_rtype_to_str(dns_message.get_question().get_qtype());

        assert_eq!(qtype, String::from("MX"));
    }

    //ToDo: Revisar
    #[test]
    fn get_question_qtype_txt(){
        let name = String::from("value");
        let dns_message = DnsMessage::new_query_message(name, String::from("TXT"), String::from("IN"), 1, true, 1);

        let qtype = Rtype::from_rtype_to_str(dns_message.get_question().get_qtype());

        assert_eq!(qtype, String::from("TXT"));
    }
}
