use crate::domain_name::DomainName;
use crate::message::resource_record::{FromBytes, ToBytes};

#[derive(Clone)]
/// An struct that represents the rdata for soa type
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// /                     MNAME                     /
/// /                                               /
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// /                     RNAME                     /
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    SERIAL                     |
/// |                                               |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    REFRESH                    |
/// |                                               |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                     RETRY                     |
/// |                                               |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    EXPIRE                     |
/// |                                               |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    MINIMUM                    |
/// |                                               |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///
pub struct SoaRdata {
    // DomainName of the name server that was the
    // original or primary source of data for this zone.
    mname: DomainName,

    // A DomainName which specifies the mailbox of the
    // person responsible for this zone.
    rname: DomainName,

    // The unsigned 32 bit version number of the original copy
    // of the zone.  Zone transfers preserve this value.  This
    // value wraps and should be compared using sequence space
    // arithmetic.
    serial: u32,

    // A 32 bit time interval before the zone should be
    // refreshed.
    refresh: u32,

    // A 32 bit time interval that should elapse before a
    // failed refresh should be retried.
    retry: u32,

    // A 32 bit time value that specifies the upper limit on
    // the time interval that can elapse before the zone is no
    // longer authoritative.
    expire: u32,

    // The unsigned 32 bit minimum TTL field that should be
    // exported with any RR from this zone.
    minimum: u32,
}

impl ToBytes for SoaRdata {
    /// Return a vec of bytes that represents the soa rdata
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        let mname_bytes = self.get_mname().to_bytes();

        for byte in mname_bytes.as_slice() {
            bytes.push(*byte);
        }

        let rname_bytes = self.get_rname().to_bytes();

        for byte in rname_bytes.as_slice() {
            bytes.push(*byte);
        }

        // Serial
        let serial_first_byte = self.get_first_serial_byte();
        bytes.push(serial_first_byte);

        let serial_second_byte = self.get_second_serial_byte();
        bytes.push(serial_second_byte);

        let serial_third_byte = self.get_third_serial_byte();
        bytes.push(serial_third_byte);

        let serial_fourth_byte = self.get_fourth_serial_byte();
        bytes.push(serial_fourth_byte);

        // Refresh
        let refresh_first_byte = self.get_first_refresh_byte();
        bytes.push(refresh_first_byte);

        let refresh_second_byte = self.get_second_refresh_byte();
        bytes.push(refresh_second_byte);

        let refresh_third_byte = self.get_third_refresh_byte();
        bytes.push(refresh_third_byte);

        let refresh_fourth_byte = self.get_fourth_refresh_byte();
        bytes.push(refresh_fourth_byte);

        // Retry
        let retry_first_byte = self.get_first_retry_byte();
        bytes.push(retry_first_byte);

        let retry_second_byte = self.get_second_retry_byte();
        bytes.push(retry_second_byte);

        let retry_third_byte = self.get_third_retry_byte();
        bytes.push(retry_third_byte);

        let retry_fourth_byte = self.get_fourth_retry_byte();
        bytes.push(retry_fourth_byte);

        // Expire
        let expire_first_byte = self.get_first_expire_byte();
        bytes.push(expire_first_byte);

        let expire_second_byte = self.get_second_expire_byte();
        bytes.push(expire_second_byte);

        let expire_third_byte = self.get_third_expire_byte();
        bytes.push(expire_third_byte);

        let expire_fourth_byte = self.get_fourth_expire_byte();
        bytes.push(expire_fourth_byte);

        // Minimum
        let minimum_first_byte = self.get_first_minimum_byte();
        bytes.push(minimum_first_byte);

        let minimum_second_byte = self.get_second_minimum_byte();
        bytes.push(minimum_second_byte);

        let minimum_third_byte = self.get_third_minimum_byte();
        bytes.push(minimum_third_byte);

        let minimum_fourth_byte = self.get_fourth_minimum_byte();
        bytes.push(minimum_fourth_byte);

        bytes
    }
}

impl FromBytes<SoaRdata> for SoaRdata {
    /// Creates a new SoaRdata from an array of bytes
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut soa_rdata = SoaRdata::new();

        let (mname, bytes_without_mname) = DomainName::from_bytes(bytes);
        let (rname, bytes_without_rname) = DomainName::from_bytes(bytes_without_mname);

        soa_rdata.set_mname(mname);
        soa_rdata.set_rname(rname);

        soa_rdata.set_serial_from_bytes(&bytes_without_rname[0..4]);
        soa_rdata.set_refresh_from_bytes(&bytes_without_rname[4..8]);
        soa_rdata.set_retry_from_bytes(&bytes_without_rname[8..12]);
        soa_rdata.set_expire_from_bytes(&bytes_without_rname[12..16]);
        soa_rdata.set_minimum_from_bytes(&bytes_without_rname[16..20]);

        soa_rdata
    }
}

impl SoaRdata {
    /// Creates a new SoaRdata with default values.
    ///
    /// # Examples
    /// ```
    /// let soa_rdata = SoaRdata::new();
    /// assert_eq!(soa_rdata.serial, 0);
    /// assert_eq!(soa_rdata.refresh, 0);
    /// assert_eq!(soa_rdata.retry, 0);
    /// assert_eq!(soa_rdata.expire, 0);
    /// assert_eq!(soa_rdata.minimum, 0);
    /// ```
    ///
    pub fn new() -> Self {
        let soa_rdata = SoaRdata {
            mname: DomainName::new(),
            rname: DomainName::new(),
            serial: 0 as u32,
            refresh: 0 as u32,
            retry: 0 as u32,
            expire: 0 as u32,
            minimum: 0 as u32,
        };

        soa_rdata
    }

    /// Gets the first byte from the serial value
    fn get_first_serial_byte(&self) -> u8 {
        let serial = self.get_serial();

        (serial >> 24) as u8
    }

    /// Gets the second byte from the serial value
    fn get_second_serial_byte(&self) -> u8 {
        let serial = self.get_serial();

        (serial >> 16) as u8
    }

    /// Gets the third byte from the serial value
    fn get_third_serial_byte(&self) -> u8 {
        let serial = self.get_serial();

        (serial >> 8) as u8
    }

    /// Gets the fourth byte from the serial value
    fn get_fourth_serial_byte(&self) -> u8 {
        let serial = self.get_serial();

        serial as u8
    }

    /// Gets the first byte from the refresh value
    fn get_first_refresh_byte(&self) -> u8 {
        let refresh = self.get_refresh();

        (refresh >> 24) as u8
    }

    /// Gets the second byte from the refresh value
    fn get_second_refresh_byte(&self) -> u8 {
        let refresh = self.get_refresh();

        (refresh >> 16) as u8
    }

    /// Gets the third byte from the refresh value
    fn get_third_refresh_byte(&self) -> u8 {
        let refresh = self.get_refresh();

        (refresh >> 8) as u8
    }

    /// Gets the fourth byte from the refresh value
    fn get_fourth_refresh_byte(&self) -> u8 {
        let refresh = self.get_refresh();

        refresh as u8
    }

    /// Gets the first byte from the retry value
    fn get_first_retry_byte(&self) -> u8 {
        let retry = self.get_retry();

        (retry >> 24) as u8
    }

    /// Gets the second byte from the retry value
    fn get_second_retry_byte(&self) -> u8 {
        let retry = self.get_retry();

        (retry >> 16) as u8
    }

    /// Gets the third byte from the retry value
    fn get_third_retry_byte(&self) -> u8 {
        let retry = self.get_retry();

        (retry >> 8) as u8
    }

    /// Gets the fourth byte from the retry value
    fn get_fourth_retry_byte(&self) -> u8 {
        let retry = self.get_retry();

        retry as u8
    }

    /// Gets the first byte from the expire value
    fn get_first_expire_byte(&self) -> u8 {
        let expire = self.get_expire();

        (expire >> 24) as u8
    }

    /// Gets the second byte from the expire value
    fn get_second_expire_byte(&self) -> u8 {
        let expire = self.get_expire();

        (expire >> 16) as u8
    }

    /// Gets the third byte from the expire value
    fn get_third_expire_byte(&self) -> u8 {
        let expire = self.get_expire();

        (expire >> 8) as u8
    }

    /// Gets the fourth byte from the expire value
    fn get_fourth_expire_byte(&self) -> u8 {
        let expire = self.get_expire();

        expire as u8
    }

    /// Gets the first byte from the minimum value
    fn get_first_minimum_byte(&self) -> u8 {
        let minimum = self.get_minimum();

        (minimum >> 24) as u8
    }

    /// Gets the second byte from the minimum value
    fn get_second_minimum_byte(&self) -> u8 {
        let minimum = self.get_minimum();

        (minimum >> 16) as u8
    }

    /// Gets the third byte from the minimum value
    fn get_third_minimum_byte(&self) -> u8 {
        let minimum = self.get_minimum();

        (minimum >> 8) as u8
    }

    /// Gets the fourth byte from the minimum value
    fn get_fourth_minimum_byte(&self) -> u8 {
        let minimum = self.get_minimum();

        minimum as u8
    }

    /// Set the serial attribute from an array of bytes
    fn set_serial_from_bytes(&mut self, bytes: &[u8]) {
        let first_byte = (bytes[0] as u32) << 24;
        let second_byte = (bytes[1] as u32) << 16;
        let third_byte = (bytes[2] as u32) << 8;
        let fourth_byte = bytes[3] as u32;

        self.set_serial(first_byte | second_byte | third_byte | fourth_byte);
    }

    /// Set the refresh attribute from an array of bytes
    fn set_refresh_from_bytes(&mut self, bytes: &[u8]) {
        let first_byte = (bytes[0] as u32) << 24;
        let second_byte = (bytes[1] as u32) << 16;
        let third_byte = (bytes[2] as u32) << 8;
        let fourth_byte = bytes[3] as u32;

        self.set_refresh(first_byte | second_byte | third_byte | fourth_byte);
    }

    /// Set the retry attribute from an array of bytes
    fn set_retry_from_bytes(&mut self, bytes: &[u8]) {
        let first_byte = (bytes[0] as u32) << 24;
        let second_byte = (bytes[1] as u32) << 16;
        let third_byte = (bytes[2] as u32) << 8;
        let fourth_byte = bytes[3] as u32;

        self.set_retry(first_byte | second_byte | third_byte | fourth_byte);
    }

    /// Set the expire attribute from an array of bytes
    fn set_expire_from_bytes(&mut self, bytes: &[u8]) {
        let first_byte = (bytes[0] as u32) << 24;
        let second_byte = (bytes[1] as u32) << 16;
        let third_byte = (bytes[2] as u32) << 8;
        let fourth_byte = bytes[3] as u32;

        self.set_expire(first_byte | second_byte | third_byte | fourth_byte);
    }

    /// Set the minimum attribute from an array of bytes
    fn set_minimum_from_bytes(&mut self, bytes: &[u8]) {
        let first_byte = (bytes[0] as u32) << 24;
        let second_byte = (bytes[1] as u32) << 16;
        let third_byte = (bytes[2] as u32) << 8;
        let fourth_byte = bytes[3] as u32;

        self.set_minimum(first_byte | second_byte | third_byte | fourth_byte);
    }
}

// Getters
impl SoaRdata {
    // Gets the mname attribute from SoaRdata
    pub fn get_mname(&self) -> DomainName {
        self.mname.clone()
    }

    // Gets the rname attribute from SoaRdata
    pub fn get_rname(&self) -> DomainName {
        self.rname.clone()
    }

    // Gets the serial attribute from SoaRdata
    pub fn get_serial(&self) -> u32 {
        self.serial
    }

    // Gets the refresh attribute from SoaRdata
    pub fn get_refresh(&self) -> u32 {
        self.refresh
    }

    // Gets the retry attribute from SoaRdata
    pub fn get_retry(&self) -> u32 {
        self.retry
    }

    // Gets the expire attribute from SoaRdata
    pub fn get_expire(&self) -> u32 {
        self.expire
    }

    // Gets the minimum attribute from SoaRdata
    pub fn get_minimum(&self) -> u32 {
        self.minimum
    }
}

// Setters
impl SoaRdata {
    // Sets the mname attibute with a DomainName
    pub fn set_mname(&mut self, mname: DomainName) {
        self.mname = mname;
    }

    // Sets the rname attibute with a DomainName
    pub fn set_rname(&mut self, rname: DomainName) {
        self.rname = rname;
    }

    // Sets the serial attibute with a value
    pub fn set_serial(&mut self, serial: u32) {
        self.serial = serial;
    }

    // Sets the refresh attibute with a value
    pub fn set_refresh(&mut self, refresh: u32) {
        self.refresh = refresh;
    }

    // Sets the retry attibute with a value
    pub fn set_retry(&mut self, retry: u32) {
        self.retry = retry;
    }

    // Sets the expire attibute with a value
    pub fn set_expire(&mut self, expire: u32) {
        self.expire = expire;
    }

    // Sets the minimum attibute with a value
    pub fn set_minimum(&mut self, minimum: u32) {
        self.minimum = minimum;
    }
}

mod test {
    use crate::domain_name::DomainName;
    use crate::message::rdata::soa_rdata::SoaRdata;
    use crate::message::resource_record::{FromBytes, ToBytes};

    #[test]
    fn constructor_test() {
        let soa_rdata = SoaRdata::new();

        assert_eq!(soa_rdata.mname.get_name(), String::from(""));
        let soa_rdata = SoaRdata::new();
        assert_eq!(soa_rdata.refresh, 0);
        assert_eq!(soa_rdata.retry, 0);
        assert_eq!(soa_rdata.expire, 0);
        assert_eq!(soa_rdata.minimum, 0);
    }

    #[test]
    fn set_and_get_mname_test() {
        let mut soa_rdata = SoaRdata::new();

        assert_eq!(soa_rdata.get_mname().get_name(), String::from(""));

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test.com"));
        soa_rdata.set_mname(domain_name);

        assert_eq!(soa_rdata.get_mname().get_name(), String::from("test.com"));
    }

    #[test]
    fn set_and_get_rname_test() {
        let mut soa_rdata = SoaRdata::new();

        assert_eq!(soa_rdata.get_rname().get_name(), String::from(""));

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test.com"));
        soa_rdata.set_rname(domain_name);

        assert_eq!(soa_rdata.get_rname().get_name(), String::from("test.com"));
    }

    #[test]
    fn set_and_get_serial_test() {
        let mut soa_rdata = SoaRdata::new();

        assert_eq!(soa_rdata.get_serial(), 0);

        soa_rdata.set_serial(4654);

        assert_eq!(soa_rdata.get_serial(), 4654);
    }

    #[test]
    fn set_and_get_refresh_test() {
        let mut soa_rdata = SoaRdata::new();

        assert_eq!(soa_rdata.get_refresh(), 0);

        soa_rdata.set_refresh(4654);

        assert_eq!(soa_rdata.get_refresh(), 4654);
    }

    #[test]
    fn set_and_get_retry_test() {
        let mut soa_rdata = SoaRdata::new();

        assert_eq!(soa_rdata.get_retry(), 0);

        soa_rdata.set_retry(4654);

        assert_eq!(soa_rdata.get_retry(), 4654);
    }

    #[test]
    fn set_and_get_expire_test() {
        let mut soa_rdata = SoaRdata::new();

        assert_eq!(soa_rdata.get_expire(), 0);

        soa_rdata.set_expire(4654);

        assert_eq!(soa_rdata.get_expire(), 4654);
    }

    #[test]
    fn set_and_get_minimum_test() {
        let mut soa_rdata = SoaRdata::new();

        assert_eq!(soa_rdata.get_minimum(), 0);

        soa_rdata.set_minimum(4654);

        assert_eq!(soa_rdata.get_minimum(), 4654);
    }

    #[test]
    fn to_bytes_test() {
        let mut soa_rdata = SoaRdata::new();

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test.com"));

        soa_rdata.set_mname(domain_name.clone());
        soa_rdata.set_rname(domain_name);
        soa_rdata.set_serial(512);
        soa_rdata.set_refresh(8);
        soa_rdata.set_retry(4);
        soa_rdata.set_expire(2);
        soa_rdata.set_minimum(1);

        let bytes_to_test: [u8; 40] = [
            4, 116, 101, 115, 116, 3, 99, 111, 109, 0, 4, 116, 101, 115, 116, 3, 99, 111, 109, 0,
            0, 0, 2, 0, 0, 0, 0, 8, 0, 0, 0, 4, 0, 0, 0, 2, 0, 0, 0, 1,
        ];
        let soa_rdata_to_bytes = soa_rdata.to_bytes();

        for (index, value) in soa_rdata_to_bytes.iter().enumerate() {
            assert_eq!(*value, bytes_to_test[index]);
        }
    }

    #[test]
    fn from_bytes_test() {
        let bytes: [u8; 40] = [
            4, 116, 101, 115, 116, 3, 99, 111, 109, 0, 4, 116, 101, 115, 116, 3, 99, 111, 109, 0,
            0, 0, 2, 0, 0, 0, 0, 8, 0, 0, 0, 4, 0, 0, 0, 2, 0, 0, 0, 1,
        ];

        let soa_rdata = SoaRdata::from_bytes(&bytes);

        assert_eq!(soa_rdata.get_mname().get_name(), String::from("test.com"));
        assert_eq!(soa_rdata.get_rname().get_name(), String::from("test.com"));
        assert_eq!(soa_rdata.get_serial(), 512);
        assert_eq!(soa_rdata.get_refresh(), 8);
        assert_eq!(soa_rdata.get_retry(), 4);
        assert_eq!(soa_rdata.get_expire(), 2);
        assert_eq!(soa_rdata.get_minimum(), 1);
    }
}
