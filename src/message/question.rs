use crate::domain_name::DomainName;

use crate::message::rclass::Rclass;

use super::rrtype::Rrtype;

#[derive(Default, Clone)]
/// An struct that represents the question section from a dns message
/// ```text
///                                1  1  1  1  1  1
///  0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    QNAME                      |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    QTYPE                      |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    rclass                     |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///
///
/// DNS question format of a query.
/// ```
#[derive (PartialEq, Debug)]
pub struct Question {
    qname: DomainName,
    // type of query
    rrtype: Rrtype,
    // class of query
    rclass: Rclass,
}

// Methods
impl Question {
    /// Creates a new Question with default values
    /// # Example
    /// ```text
    /// let mut question = Question::new();
    /// assert_eq!(question.qname.get_name(), String::from(""));
    /// assert_eq!(question.qtype, 0);
    /// assert_eq!(question.rclass, 0);
    /// ```
    pub fn new() -> Self {
        let question: Question = Question {
            qname: DomainName::new(),
            rrtype: Rrtype::A,
            rclass: Rclass::IN,
        };
        question
    }

    /// Given an array of bytes, creates a new Question.
    /// # Example
    /// ```text
    /// let bytes: [u8; 14] = [4, 116, 101, 115, 116, 3, 99, 111, 109, 0, 0, 5, 0, 1];
    /// 
    /// let mut question = Question::new();
    /// let (question, _others_msg_bytes) = Question::from_bytes(&bytes, &bytes).unwrap();
    /// let qname = question.get_qname().get_name();
    /// assert_eq!(qname, String::from("test.com"));
    /// let qtype = question.get_qtype();
    /// assert_eq!(u16::from(qtype), 5);
    /// let rclass = question.get_rclass();
    /// assert_eq!(Rclass::from_rclass_to_int(rclass), 1);
    /// ```
    pub fn from_bytes<'a>(
        bytes: &'a [u8],
        full_msg: &'a [u8],
    ) -> Result<(Question, &'a [u8]), &'static str> {
        let domain_name_result = DomainName::from_bytes(bytes, full_msg);

        match domain_name_result {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        let (qname, bytes_without_name) = domain_name_result.unwrap();

        if bytes_without_name.len() < 4 {
            return Err("Format Error");
        }

        let rrtype_int = ((bytes_without_name[0] as u16) << 8) | bytes_without_name[1] as u16;
        let rrtype = Rrtype::from(rrtype_int);
        let rclass_int = ((bytes_without_name[2] as u16) << 8) | bytes_without_name[3] as u16;
        let rclass = Rclass::from(rclass_int);

        let mut question = Question::new();
        question.set_qname(qname);
        question.set_rrtype(rrtype);
        question.set_rclass(rclass);

        Ok((question, &bytes_without_name[4..]))
    }

    /// Returns a byte that represents the first byte from qtype.
    /// # Example
    /// ```text
    /// let mut question = Question::new();
    /// question.set_qtype(Rtype::A);
    /// let first_byte = question.get_first_qtype_byte();
    /// assert_eq!(first_byte, 1);
    /// ```
    fn get_first_rrtype_byte(&self) -> u8 {
        let rrtype = self.get_rrtype();
        let first_byte = (u16::from(rrtype) >> 8) as u8;

        first_byte
    }

    // Returns a byte that represents the second byte from qtype.
    fn get_second_rrtype_byte(&self) -> u8 {
        let rrtype = self.get_rrtype();
        let second_byte = u16::from(rrtype) as u8;

        second_byte
    }

    // Returns a byte that represents the first byte from rclass.
    fn get_first_rclass_byte(&self) -> u8 {
        let rclass: Rclass = self.get_rclass();
        let first_byte = (u16::from(rclass) >> 8) as u8;

        first_byte
    }

    // Returns a byte that represents the second byte from rclass.
    fn get_second_rclass_byte(&self) -> u8 {
        let rclass = self.get_rclass();
        let second_byte = u16::from(rclass) as u8;

        second_byte
    }

    // Returns a vec of bytes that represents the Question.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut question_bytes: Vec<u8> = Vec::new();

        let qname = self.get_qname().get_name();

        if qname != "".to_string() {
            let qname_bytes = self.get_qname().to_bytes();
            for byte in qname_bytes.as_slice() {
                question_bytes.push(*byte);
            }

            question_bytes.push(self.get_first_rrtype_byte());
            question_bytes.push(self.get_second_rrtype_byte());
            question_bytes.push(self.get_first_rclass_byte());
            question_bytes.push(self.get_second_rclass_byte());
        }
        return question_bytes;
    }
}

// Setters
impl Question {
    pub fn set_qname(&mut self, qname: DomainName) {
        self.qname = qname;
    }

    pub fn set_rrtype(&mut self, rrtype: Rrtype) {
        self.rrtype = rrtype;
    }

    pub fn set_rclass(&mut self, rclass: Rclass) {
        self.rclass = rclass;
    }
}

// Getters
impl Question {
    pub fn get_qname(&self) -> DomainName {
        self.qname.clone()
    }

    pub fn get_rrtype(&self) -> Rrtype {
        self.rrtype.clone()
    }

    pub fn get_rclass(&self) -> Rclass {
        self.rclass.clone()
    }
}

#[cfg(test)]
mod question_test {

    use super::Question;
    use crate::domain_name::DomainName;
    use crate::message::rrtype::Rrtype;
    use crate::message::rclass::Rclass;

    #[test]
    fn constructor_test() {
        let question = Question::new();

        assert_eq!(question.qname.get_name(), String::from(""));
        assert_eq!(question.rrtype.to_string(), String::from("A"));
        assert_eq!(question.rclass.to_string(), String::from("IN"));
    }

    #[test]
    fn set_and_get_qname() {
        let mut question = Question::new();

        let mut qname = question.get_qname().get_name();
        assert_eq!(qname, String::from(""));

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("Test"));
        question.set_qname(domain_name);

        qname = question.get_qname().get_name();
        assert_eq!(qname, String::from("Test"));
    }

    #[test]
    fn set_and_get_rrtype() {
        let mut question = Question::new();

        let mut rrtype = question.get_rrtype();
        assert_eq!(rrtype.to_string(), String::from("A"));

        question.set_rrtype(Rrtype::CNAME);
        rrtype = question.get_rrtype();
        assert_eq!(rrtype.to_string(), String::from("CNAME"));
    }

    #[test]
    fn set_and_get_rclass() {
        let mut question = Question::new();

        let mut rclass = question.get_rclass();
        assert_eq!(rclass.to_string(), String::from("IN"));

        question.set_rclass(Rclass::CS);
        rclass = question.get_rclass();
        assert_eq!(rclass.to_string(), String::from("CS"));
    }

    #[test]
    fn to_bytes_correct_val() {
        let mut domain_name = DomainName::new();
        let mut question = Question::new();

        domain_name.set_name(String::from("test.com"));
        question.set_qname(domain_name);
        question.set_rrtype(Rrtype::CNAME);
        question.set_rclass(Rclass::IN);

        let bytes_to_test: [u8; 14] = [4, 116, 101, 115, 116, 3, 99, 111, 109, 0, 0, 5, 0, 1];
        let question_to_bytes = question.to_bytes();

        for (index, value) in question_to_bytes.iter().enumerate() {
            assert_eq!(*value, bytes_to_test[index]);
        }
    }

    #[test]
    fn to_bytes_empty_qname() {
        let question = Question::new();
        let expected_question_to_bytes: Vec<u8> = Vec::new();

        let real_question_to_bytes = question.to_bytes();
        assert_eq!(real_question_to_bytes, expected_question_to_bytes);
    }

    #[test]
    fn from_bytes_correct_val() {
        let bytes: [u8; 14] = [4, 116, 101, 115, 116, 3, 99, 111, 109, 0, 0, 5, 0, 1];

        let (question, _others_msg_bytes) = Question::from_bytes(&bytes, &bytes).unwrap();

        let qname = question.get_qname().get_name();
        assert_eq!(qname, String::from("test.com"));
        let rrtype = question.get_rrtype();
        assert_eq!(u16::from(rrtype), 5);
        let rclass = question.get_rclass();
        assert_eq!(u16::from(rclass), 1);
    }

    #[test]
    #[should_panic(expected = "Format Error")]
    fn from_bytes_handling_err() {
        let bytes: [u8; 14] = [38, 55, 101, 115, 116, 3, 99, 111, 109, 0, 0, 5, 0, 1];

        Question::from_bytes(&bytes, &bytes).unwrap();
    }

    #[test]
    #[should_panic(expected = "Format Error")]
    fn from_bytes_less_bytes_than_expected() {
        let bytes: [u8; 12] = [4, 116, 101, 115, 116, 3, 99, 111, 109, 0, 0, 5];

        Question::from_bytes(&bytes, &bytes).unwrap();
    }
}
