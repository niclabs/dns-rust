use crate::message::resource_record::{FromBytes, ToBytes};

#[derive(Clone, Debug, PartialEq)]
/// Struct for DNSKEY Rdata
///                       1 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 3 3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |              Flags            |    Protocol   |   Algorithm   |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// /                                                               /
/// /                            Public Key                         /
/// /                                                               /
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// 
/// [RFC 4034](https://tools.ietf.org/html/rfc4034#section-2.1.)

pub struct DnskeyRdata {
    pub flags: u16,
    pub protocol: u8,
    pub algorithm: u8,
    pub public_key: Vec<u8>,
}

impl ToBytes for DnskeyRdata {
    /// Returns a `Vec<u8>` of bytes that represents the DNSKEY RDATA.
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.flags.to_be_bytes());
        bytes.push(self.protocol);
        bytes.push(self.algorithm);
        bytes.extend_from_slice(&self.public_key);

        bytes
    }
}

/// Constructor for DnskeyRdata and getter's for the fields
impl DnskeyRdata {
    /// Constructs a new `DnskeyRdata` with default values.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// let dnskey_rdata = DnskeyRdata::new();
    /// ```
    pub fn new() -> DnskeyRdata {
        DnskeyRdata {
            flags: 0,
            protocol: 0,
            algorithm: 0,
            public_key: Vec::new(),
        }
    }

    /// Get the flags of the DNSKEY RDATA.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let dnskey_rdata = DnskeyRdata::new();
    /// assert_eq!(dnskey_rdata.get_flags(), 0);
    /// ```
    pub fn get_flags(&self) -> u16 {
        self.flags.clone()
    }

    /// Get the protocol of the DNSKEY RDATA.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let dnskey_rdata = DnskeyRdata::new();
    /// assert_eq!(dnskey_rdata.get_protocol(), 0);
    /// ```
    pub fn get_protocol(&self) -> u8 {
        self.protocol.clone()
    }

    /// Get the algorithm of the DNSKEY RDATA. 
    /// 
    /// # Examples
    /// 
    /// ```
    /// let dnskey_rdata = DnskeyRdata::new();
    /// assert_eq!(dnskey_rdata.get_algorithm(), 0);
    /// ```
    pub fn get_algorithm(&self) -> u8 {
        self.algorithm.clone()
    }

    /// Get the public key of the DNSKEY RDATA.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let dnskey_rdata = DnskeyRdata::new();
    /// assert_eq!(dnskey_rdata.get_public_key(), Vec::new());
    /// ```
    pub fn get_public_key(&self) -> Vec<u8> {
        self.public_key.clone()
    }
}

/// Setters for DnskeyRdata
impl DnskeyRdata {
    /// Set the flags of the DNSKEY RDATA.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut dnskey_rdata = DnskeyRdata::new();
    /// dnskey_rdata.set_flags(1);
    /// assert_eq!(dnskey_rdata.get_flags(), 1);
    /// ```
    pub fn set_flags(&mut self, flags: u16) {
        self.flags = flags;
    }

    /// Set the protocol of the DNSKEY RDATA.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut dnskey_rdata = DnskeyRdata::new();
    /// dnskey_rdata.set_protocol(1);
    /// assert_eq!(dnskey_rdata.get_protocol(), 1);
    /// ```
    pub fn set_protocol(&mut self, protocol: u8) {
        self.protocol = protocol;
    }

    /// Set the algorithm of the DNSKEY RDATA.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut dnskey_rdata = DnskeyRdata::new();
    /// dnskey_rdata.set_algorithm(1);
    /// assert_eq!(dnskey_rdata.get_algorithm(), 1);
    /// ```
    pub fn set_algorithm(&mut self, algorithm: u8) {
        self.algorithm = algorithm;
    }

    /// Set the public key of the DNSKEY RDATA.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut dnskey_rdata = DnskeyRdata::new();
    /// dnskey_rdata.set_public_key(vec![0x01, 0x02]);
    /// assert_eq!(dnskey_rdata.get_public_key(), vec![0x01, 0x02]);
    /// ```
    pub fn set_public_key(&mut self, public_key: Vec<u8>) {
        self.public_key = public_key;
    }
}