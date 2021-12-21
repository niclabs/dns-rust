use crate::domain_name::DomainName;

#[derive(Default, Clone)]
/// An struct that represents the question section from a dns message
///
///                                1  1  1  1  1  1
///  0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    QNAME                      |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    QTYPE                      |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    QCLASS                     |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///

// DNS question format of a query.
pub struct Question {
    qname: DomainName,
    // type of query
    qtype: u16,
    // class of query
    qclass: u16,
}

// Methods
impl Question {
    // Creates a new Question with default values
    ///
    /// # Examples
    /// ```
    ///
    /// let mut question = Question::new();
    /// assert_eq!(question.qname.get_name(), String::from(""));
    /// assert_eq!(question.qtype, 0);
    /// assert_eq!(question.qclass, 0);
    /// ```
    ///
    pub fn new() -> Self {
        let question: Question = Question {
            qname: DomainName::new(),
            qtype: 0 as u16,
            qclass: 0 as u16,
        };
        question
    }

    /// Given an array of bytes, creates a new Question.
    pub fn from_bytes<'a>(bytes: &'a [u8], full_msg: &'a [u8]) -> (Question, &'a [u8]) {
        let (qname, bytes_without_name) = DomainName::from_bytes(bytes, full_msg);

        let qtype = ((bytes_without_name[0] as u16) << 8) | bytes_without_name[1] as u16;
        let qclass = ((bytes_without_name[2] as u16) << 8) | bytes_without_name[3] as u16;

        let mut question = Question::new();
        question.set_qname(qname);
        question.set_qtype(qtype);
        question.set_qclass(qclass);

        (question, &bytes_without_name[4..])
    }

    /// Returns a byte that represents the first byte from qtype.
    fn get_first_qtype_byte(&self) -> u8 {
        let qtype = self.get_qtype();
        let first_byte = (qtype >> 8) as u8;

        first_byte
    }

    /// Returns a byte that represents the second byte from qtype.
    fn get_second_qtype_byte(&self) -> u8 {
        let qtype = self.get_qtype();
        let second_byte = qtype as u8;

        second_byte
    }

    /// Returns a byte that represents the first byte from qclass.
    fn get_first_qclass_byte(&self) -> u8 {
        let qclass = self.get_qclass();
        let first_byte = (qclass >> 8) as u8;

        first_byte
    }

    /// Returns a byte that represents the second byte from qclass.
    fn get_second_qclass_byte(&self) -> u8 {
        let qclass = self.get_qclass();
        let second_byte = qclass as u8;

        second_byte
    }

    /// Returns a vec of bytes that represents the Question.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut question_bytes: Vec<u8> = Vec::new();

        let qname_bytes = self.get_qname().to_bytes();
        for byte in qname_bytes.as_slice() {
            question_bytes.push(*byte);
        }

        question_bytes.push(self.get_first_qtype_byte());
        question_bytes.push(self.get_second_qtype_byte());
        question_bytes.push(self.get_first_qclass_byte());
        question_bytes.push(self.get_second_qclass_byte());

        question_bytes
    }
}

// Setters
impl Question {
    pub fn set_qname(&mut self, qname: DomainName) {
        self.qname = qname;
    }

    pub fn set_qtype(&mut self, qtype: u16) {
        self.qtype = qtype;
    }

    pub fn set_qclass(&mut self, qclass: u16) {
        self.qclass = qclass;
    }
}

// Getters
impl Question {
    pub fn get_qname(&self) -> DomainName {
        self.qname.clone()
    }

    pub fn get_qtype(&self) -> u16 {
        self.qtype
    }

    pub fn get_qclass(&self) -> u16 {
        self.qclass
    }
}

// Tests
mod test {
    use super::Question;
    use crate::domain_name::DomainName;

    #[test]
    fn constructor_test() {
        let question = Question::new();

        assert_eq!(question.qname.get_name(), String::from(""));
        assert_eq!(question.qtype, 0);
        assert_eq!(question.qclass, 0);
    }

    #[test]
    fn set_and_get_qname_test() {
        let mut question = Question::new();

        assert_eq!(question.get_qname().get_name(), String::from(""));

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("Test"));
        question.set_qname(domain_name);

        let qname = question.get_qname().get_name();
        assert_eq!(qname, String::from("Test"));
    }

    #[test]
    fn set_and_get_qtype_test() {
        let mut question = Question::new();

        assert_eq!(question.get_qtype(), 0);

        question.set_qtype(1 as u16);
        let qtype = question.get_qtype();

        assert_eq!(qtype, 1 as u16);
    }

    #[test]
    fn set_and_get_qclass_test() {
        let mut question = Question::new();

        assert_eq!(question.get_qclass(), 0);

        question.set_qclass(1 as u16);
        let qclass = question.get_qclass();

        assert_eq!(qclass, 1 as u16);
    }

    #[test]
    fn to_bytes_test() {
        let mut question = Question::new();

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test.com"));

        question.set_qname(domain_name);
        question.set_qtype(5);
        question.set_qclass(2);

        let bytes_to_test: [u8; 14] = [4, 116, 101, 115, 116, 3, 99, 111, 109, 0, 0, 5, 0, 2];
        let question_to_bytes = question.to_bytes();

        for (index, value) in question_to_bytes.iter().enumerate() {
            assert_eq!(*value, bytes_to_test[index]);
        }
    }

    #[test]
    fn from_bytes_test() {
        let bytes: [u8; 14] = [4, 116, 101, 115, 116, 3, 99, 111, 109, 0, 0, 5, 0, 2];

        let (question, _others_msg_bytes) = Question::from_bytes(&bytes);

        assert_eq!(question.get_qname().get_name(), String::from("test.com"));
        assert_eq!(question.get_qtype(), 5);
        assert_eq!(question.get_qclass(), 2);
    }
}
