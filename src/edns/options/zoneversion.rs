
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
}

impl ZoneversionOptData {

    // getters
    pub fn get_label_count(&self) -> u8 {
        self.label_count.clone()
    }

    pub fn get_type_(&self) -> u8 {
        self.type_.clone()
    }

    // setters
    fn set_label_count(&mut self, label_count: u8) {
        self.label_count = label_count;
    }

    fn set_type_(&mut self, type_: u8) {
        self.type_ = type_;
    }
}