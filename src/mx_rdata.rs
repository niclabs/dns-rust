use crate::domain_name::DomainName;
use crate::resource_record::{FromBytes, ToBytes};

#[derive(Clone)]
/// An struct that represents the rdata for mx type
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                  PREFERENCE                   |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// /                   EXCHANGE                    /
/// /                                               /
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///
pub struct MxRdata {
    preference: u16,
    // A domain name
    exchange: DomainName,
}

impl ToBytes for MxRdata {
    /// Return a vec of bytes that represents the mx rdata
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        let first_byte_preference = self.get_first_preference_byte();
        let second_byte_preference = self.get_second_preference_byte();

        let exchange_bytes = self.get_exchange().to_bytes();

        bytes.push(first_byte_preference);
        bytes.push(second_byte_preference);

        for byte in exchange_bytes.as_slice() {
            bytes.push(*byte);
        }

        bytes
    }
}

impl FromBytes<MxRdata> for MxRdata {
    /// Creates a new MxRdata from an array of bytes
    fn from_bytes(bytes: &[u8]) -> Self {
        let preference = (bytes[0] as u16) << 8 | bytes[1] as u16;

        // This must be replace for a DomainName struct
        let (exchange, _) = DomainName::from_bytes(&bytes[2..]);

        let mut mx_rdata = MxRdata::new();

        mx_rdata.set_preference(preference);
        mx_rdata.set_exchange(exchange);

        mx_rdata
    }
}

impl MxRdata {
    /// Creates a new MxRdata with default values.
    ///
    /// # Examples
    /// ```
    /// let mx_rdata = MxRdata::new();
    /// assert_eq!(mx_rdata.preference, 0);
    /// assert_eq!(mx_rdata.exchange, String::from(""));
    /// ```
    ///
    pub fn new() -> Self {
        let mx_rdata: MxRdata = MxRdata {
            preference: 0 as u16,
            exchange: DomainName::new(),
        };
        mx_rdata
    }

    // Gets the first byte from the preference attribute
    pub fn get_first_preference_byte(&self) -> u8 {
        (self.get_preference() >> 8) as u8
    }

    // Gets the second byte from the preference attribute
    pub fn get_second_preference_byte(&self) -> u8 {
        self.get_preference() as u8
    }
}

// Getters
impl MxRdata {
    // Gets the preference attribute from MxRdata
    pub fn get_preference(&self) -> u16 {
        self.preference
    }

    // Gets the exchange attribute from MxRdata
    pub fn get_exchange(&self) -> DomainName {
        self.exchange.clone()
    }
}

// Setters
impl MxRdata {
    // Sets the preference attibute with a value
    pub fn set_preference(&mut self, preference: u16) {
        self.preference = preference;
    }

    // Sets the exchange attibute with a value
    pub fn set_exchange(&mut self, exchange: DomainName) {
        self.exchange = exchange;
    }
}

mod test {
    use super::DomainName;
    use super::MxRdata;
    use crate::resource_record::{FromBytes, ToBytes};

    #[test]
    fn constructor_test() {
        let mx_rdata = MxRdata::new();

        assert_eq!(mx_rdata.preference, 0);
        assert_eq!(mx_rdata.exchange.get_name(), String::from(""));
    }

    #[test]
    fn set_and_get_preference_test() {
        let mut mx_rdata = MxRdata::new();

        assert_eq!(mx_rdata.get_preference(), 0);

        mx_rdata.set_preference(16);

        assert_eq!(mx_rdata.get_preference(), 16);
    }

    #[test]
    fn set_and_get_exchange_test() {
        let mut mx_rdata = MxRdata::new();

        assert_eq!(mx_rdata.get_exchange().get_name(), String::from(""));

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test"));

        mx_rdata.set_exchange(domain_name);

        assert_eq!(mx_rdata.get_exchange().get_name(), String::from("test"));
    }

    #[test]
    fn to_bytes_test() {
        let mut mx_rdata = MxRdata::new();

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test.com"));

        mx_rdata.set_exchange(domain_name);
        mx_rdata.set_preference(128);

        let bytes_to_test: [u8; 12] = [0, 128, 4, 116, 101, 115, 116, 3, 99, 111, 109, 0];
        let mx_rdata_to_bytes = mx_rdata.to_bytes();

        for (index, value) in mx_rdata_to_bytes.iter().enumerate() {
            assert_eq!(*value, bytes_to_test[index]);
        }
    }

    #[test]
    fn from_bytes_test() {
        let bytes: [u8; 12] = [0, 128, 4, 116, 101, 115, 116, 3, 99, 111, 109, 0];

        let mx_rdata = MxRdata::from_bytes(&bytes);

        assert_eq!(mx_rdata.get_preference(), 128);
        assert_eq!(mx_rdata.get_exchange().get_name(), String::from("test.com"));
    }
}
