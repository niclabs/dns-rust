use std::string::String;

#[derive(Clone, Default)]

// DNS domain name represented as a sequence of labels, where each label consists of
// a length octet followed by that number of octets.
// The domain name terminates with the zero length octet for the null label of the root.
pub struct DomainName {
    name: String,
}

// Methods
impl DomainName {
    /// Creates a new DomainName with default name
    ///
    /// # Examples
    /// ```
    /// let domain_name = DomainName::new();
    ///
    /// assert_eq!(domain_name.name, String::from(""));
    /// ```
    ///
    pub fn new() -> Self {
        let domain_name: DomainName = DomainName {
            name: String::from(""),
        };
        domain_name
    }

    /// Given an array of bytes, creates a new DomainName and returns the unused bytes
    pub fn from_bytes_no_offset(bytes: &[u8]) -> (Self, &[u8]) {
        let mut name = String::from("");
        let mut index = 0;

        for byte in bytes {
            if *byte <= 9 && *byte >= 1 {
                name.push('.');
            } else if *byte == 0 {
                break;
            } else {
                name.push(*byte as char);
            }
            index += 1;
        }

        name.remove(0);

        let mut domain_name = DomainName::new();
        domain_name.set_name(name);

        (domain_name, &bytes[index + 1..])
    }

    pub fn from_bytes<'a>(bytes: &'a [u8], full_msg: &'a [u8]) -> (Self, &'a [u8]) {
        let msg_compresion = bytes[0].clone() >> 6;

        if msg_compresion == 3 {
            let offset: usize = (((bytes[0].clone() as u16) << 8 | bytes[1].clone() as u16)
                & 0b0011111111111111) as usize;
            let (question, mut no_question_bytes) =
                DomainName::from_bytes_no_offset(&full_msg[offset..]);
            no_question_bytes = &bytes[2..];

            return (question, no_question_bytes);
        } else {
            return DomainName::from_bytes_no_offset(&bytes[0..]);
        }
    }

    /// Returns an array of bytes that represents the domain name
    pub fn to_bytes(&self) -> Vec<u8> {
        let name = self.get_name();
        let mut bytes: Vec<u8> = Vec::new();

        for word in name.split(".") {
            let word_length = word.len();
            bytes.push(word_length as u8);

            for character in word.chars() {
                bytes.push(character as u8);
            }
        }

        bytes.push(0 as u8);

        bytes
    }
}

// Setters Domain Name
impl DomainName {
    // Sets the name attribute with a value
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

// Getters Domain Name
impl DomainName {
    // Gets the name attribute from a DomainName struct
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

mod test {
    use super::DomainName;

    #[test]
    fn constructor_test() {
        let domain_name = DomainName::new();

        assert_eq!(domain_name.name, String::from(""));
    }

    #[test]
    fn set_and_get_name_test() {
        let mut domain_name = DomainName::new();

        assert_eq!(domain_name.name, String::from(""));

        domain_name.set_name(String::from("test.test2.com"));

        assert_eq!(domain_name.get_name(), String::from("test.test2.com"))
    }

    #[test]
    fn to_bytes_test() {
        let mut domain_name = DomainName::new();
        let bytes_test: Vec<u8> = vec![
            4, 116, 101, 115, 116, 5, 116, 101, 115, 116, 50, 3, 99, 111, 109, 0,
        ];
        domain_name.set_name(String::from("test.test2.com"));

        let bytes = domain_name.to_bytes();

        for (index, byte) in bytes.iter().enumerate() {
            assert_eq!(*byte, bytes_test[index]);
        }
    }

    #[test]
    fn from_bytes_test() {
        let bytes_test: Vec<u8> = vec![
            4, 116, 101, 115, 116, 5, 116, 101, 115, 116, 50, 3, 99, 111, 109, 0,
        ];
        let (domain_name, _) = DomainName::from_bytes(&bytes_test, &bytes_test);

        assert_eq!(domain_name.get_name(), String::from("test.test2.com"));
    }
}
