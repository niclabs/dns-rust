use crate::resource_record::{FromBytes, ToBytes};

#[derive(Clone)]
/// An struct that represents the rdata for a type
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    ADDRESS                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///
pub struct ARdata {
    // A 32 bit Internet address.
    address: [u8; 4],
}

impl ToBytes for ARdata {
    /// Return a vec of bytes that represents the a rdata
    fn to_bytes(&self) -> Vec<u8> {
        let bytes: Vec<u8> = self.get_address().to_vec();

        bytes
    }
}

impl FromBytes<ARdata> for ARdata {
    /// Creates a new ARdata from an array of bytes
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut a_rdata = ARdata::new();

        let array_bytes = [bytes[0], bytes[1], bytes[2], bytes[3]];

        a_rdata.set_address(array_bytes);

        a_rdata
    }
}

impl ARdata {
    /// Creates a new ARdata with default values.
    ///
    /// # Examples
    /// ```
    /// let a_rdata = ARdata::new();
    /// assert_eq!(a_rdata.address[0], 0);
    /// assert_eq!(a_rdata.address[1], 0);
    /// assert_eq!(a_rdata.address[2], 0);
    /// assert_eq!(a_rdata.address[3], 0);
    /// ```
    ///
    pub fn new() -> Self {
        let a_rdata = ARdata {
            address: [0 as u8, 0 as u8, 0 as u8, 0 as u8],
        };

        a_rdata
    }
}

// Getters
impl ARdata {
    // Gets the address attribute from ARdata
    pub fn get_address(&self) -> [u8; 4] {
        self.address
    }
}

// Setters
impl ARdata {
    // Sets the address attibute with a value
    pub fn set_address(&mut self, address: [u8; 4]) {
        self.address = address;
    }
}

mod test {
    use super::ARdata;
    use super::{FromBytes, ToBytes};

    #[test]
    fn constructor_test() {
        let a_rdata = ARdata::new();
        assert_eq!(a_rdata.address[0], 0);
        assert_eq!(a_rdata.address[1], 0);
        assert_eq!(a_rdata.address[2], 0);
        assert_eq!(a_rdata.address[3], 0);
    }

    #[test]
    fn set_and_get_address_test() {
        let mut a_rdata = ARdata::new();

        assert_eq!(a_rdata.get_address()[0], 0);
        assert_eq!(a_rdata.get_address()[1], 0);
        assert_eq!(a_rdata.get_address()[2], 0);
        assert_eq!(a_rdata.get_address()[3], 0);

        a_rdata.set_address([127, 0, 0, 1]);

        assert_eq!(a_rdata.get_address()[0], 127);
        assert_eq!(a_rdata.get_address()[1], 0);
        assert_eq!(a_rdata.get_address()[2], 0);
        assert_eq!(a_rdata.get_address()[3], 1);
    }

    #[test]
    fn to_bytes_test() {
        let mut a_rdata = ARdata::new();

        a_rdata.set_address([127, 0, 0, 1]);

        let a_rdata_to_bytes = a_rdata.to_bytes();

        assert_eq!(a_rdata_to_bytes[0], 127);
        assert_eq!(a_rdata_to_bytes[1], 0);
        assert_eq!(a_rdata_to_bytes[2], 0);
        assert_eq!(a_rdata_to_bytes[3], 1);
    }

    #[test]
    fn from_bytes_test() {
        let bytes: [u8; 4] = [128, 0, 0, 1];
        let a_rdata = ARdata::from_bytes(&bytes);

        assert_eq!(a_rdata.get_address()[0], 128);
        assert_eq!(a_rdata.get_address()[1], 0);
        assert_eq!(a_rdata.get_address()[2], 0);
        assert_eq!(a_rdata.get_address()[3], 1);
    }
}
