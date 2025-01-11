use crate::message::resource_record::ToBytes;

/// A structure representing an opaque string.
///
/// This struct is designed to hold a sequence of bytes (`Vec<u8>`),
/// which can represent any kind of binary or textual data.
#[derive(Debug, PartialEq, Eq, Clone)]
struct OpaqueString {
    /// The underlying byte data for the opaque string.
    data: Vec<u8>,
}
impl OpaqueString {

    // getter
    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
    // setter
    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.is_empty() {
            return Err("Not enough bytes to parse an OpaqueString");
        }
        Ok(OpaqueString { data: bytes.to_vec() })
    }
}

impl ToBytes for OpaqueString {
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
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ZoneversionOptData {
    label_count: u8,
    type_: u8,
    version: OpaqueString,
}

impl ZoneversionOptData {
    pub fn new(label_count: u8, type_: u8, version: OpaqueString) -> Self {
        ZoneversionOptData { label_count, type_, version }
    }

    // getters
    pub fn get_label_count(&self) -> u8 {
        self.label_count.clone()
    }

    pub fn get_type_(&self) -> u8 {
        self.type_.clone()
    }

    pub fn get_version(&self) -> OpaqueString {
        self.version.clone()
    }

    // setters
    fn set_label_count(&mut self, label_count: u8) {
        self.label_count = label_count;
    }

    fn set_type_(&mut self, type_: u8) {
        self.type_ = type_;
    }

    fn set_version(&mut self, version: OpaqueString) {
        self.version = version;
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < 3 {
            return Err("Not enough bytes to parse ZoneVersion");
        }
        let label_count = bytes[0];
        let type_ = bytes[1];
        let version = OpaqueString::from_bytes(&bytes[2..])
            .map_err(|_| "Error parsing version")?;

        Ok(ZoneversionOptData { label_count, type_, version })
    }
}

impl ToBytes for ZoneversionOptData {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = vec![];
        let label_count: u8 = self.label_count;
        res.push(label_count);

        let type_: u8 = self.type_;
        res.push(type_);

        let mut version  = self.version.to_bytes();
        res.append(&mut version);

        res
    }
}