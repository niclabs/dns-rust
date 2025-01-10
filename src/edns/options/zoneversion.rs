
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
}