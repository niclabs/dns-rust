use crate::message::resource_record::ToBytes;

/// A structure representing an opaque string.
///
/// This struct is designed to hold a sequence of bytes (`Vec<u8>`),
/// which can represent any kind of binary or textual data.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct OpaqueString {
    /// The underlying byte data for the opaque string.
    data: Vec<u8>,
}
impl OpaqueString {

    /// Returns a clone of the opaque string's data.
    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
    /// Sets new data for the opaque string.
    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    /// Constructs a new `OpaqueString` from a slice of bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the provided byte slice is empty.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.is_empty() {
            return Err("Not enough bytes to parse an OpaqueString");
        }
        Ok(OpaqueString { data: bytes.to_vec() })
    }
}

impl ToBytes for OpaqueString {
    /// Converts the `OpaqueString` into a vector of bytes.
    fn to_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }
}


/*
                +0 (MSB)                       +1 (LSB)
   +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
0: |           LABELCOUNT          |            TYPE               |
   +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
2: |                            VERSION                            |
   /                                                               /
   +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
 */

/// Represents the ZONEVERSION EDNS(0) option data as defined in RFC 9660.
///
/// The ZONEVERSION option is used by DNS clients to request zone version
/// information from authoritative name servers.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ZoneversionOptData {
    label_count: Option<u8>,
    type_: Option<u8>,
    version: Option<OpaqueString>,
}

impl ZoneversionOptData {
    /// Constructs a new, empty `ZoneversionOptData`.
    ///
    /// # Notes
    /// RFC 9660: A DNS client MAY signal its support and desire for zone version information by including
    /// an empty ZONEVERSION option in the EDNS(0) OPT pseudo-RR of a query to an authoritative name server.
    pub fn new() -> Self {
        ZoneversionOptData{ label_count: None, type_: None, version: None }
    }

    /// Constructs a new `ZoneversionOptData` with the specified fields.
    pub fn new_from(label_count: u8, type_: u8, version: OpaqueString) -> Self {
        ZoneversionOptData { label_count: Some(label_count), type_: Some(type_), version: Some(version) }
    }

    // getters

    /// Returns the label count, if set.
    pub fn get_label_count(&self) -> Option<u8> {
        self.label_count.clone()
    }

    /// Returns the type, if set.
    pub fn get_type_(&self) -> Option<u8> {
        self.type_.clone()
    }

    /// Returns the version, if set.
    pub fn get_version(&self) -> Option<OpaqueString> {
        self.version.clone()
    }

    // setters

    /// Sets the label count.
    fn set_label_count(&mut self, label_count: u8) {
        self.label_count = Option::from(label_count);
    }

    /// Sets the type.
    fn set_type_(&mut self, type_: u8) {
        self.type_ = Option::from(type_);
    }

    /// Sets the version.
    fn set_version(&mut self, version: OpaqueString) {
        self.version = Option::from(version);
    }

    /// Parses a `ZoneversionOptData` from a slice of bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the byte slice is insufficient to parse
    /// the required fields.
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.is_empty() {
            return Ok(ZoneversionOptData::new());
        }
        else if bytes.len() < 3 {
            return Err("Not enough bytes to parse ZoneVersion");
        }
        let label_count = bytes[0];
        let type_ = bytes[1];
        let version = OpaqueString::from_bytes(&bytes[2..])
            .map_err(|_| "Error parsing version")?;

        Ok(ZoneversionOptData {
            label_count: Some(label_count),
            type_: Some(type_),
            version: Some(version)
        })
    }
}

impl ToBytes for ZoneversionOptData {
    /// Converts the `ZoneversionOptData` into a vector of bytes.
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = vec![];

        if self.label_count.is_none() {
            return res;
        }

        let label_count: u8 = self.label_count.unwrap();
        res.push(label_count);

        let type_: u8 = self.type_.unwrap();
        res.push(type_);

        let mut version  = self.version.clone().unwrap().to_bytes();
        res.append(&mut version);

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bytes_from_bytes () {
        let label_count:u8 = 0x0a;
        let type_:u8 = 0x00;
        let version :OpaqueString = OpaqueString::from_bytes(&[0x12, 0x34, 0x56, 0x78]).unwrap();
        let data = version.get_data();
        let version_bytes = version.to_bytes();

        let zone_version = ZoneversionOptData::new_from(label_count, type_, version);
        let serialized = zone_version.to_bytes();

        assert_eq!(serialized[0], label_count);
        assert_eq!(serialized[1], type_);
        assert_eq!(serialized[2..], data);
        assert_eq!(serialized[2..], version_bytes);

        let deserialized: ZoneversionOptData = ZoneversionOptData::from_bytes(&serialized).unwrap();
        assert_eq!(deserialized, zone_version);

    }
}
