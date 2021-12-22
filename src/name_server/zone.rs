use crate::message::resource_record::ResourceRecord;

#[derive(Clone)]
/// Structs that represents data from a zone
pub struct NSZone {
    value: Vec<ResourceRecord>,
    childs: Vec<NSZone>,
}

impl NSZone {
    pub fn new() -> Self {
        let ns_zone = NSZone {
            value: Vec::<ResourceRecord>::new(),
            childs: Vec::<NSZone>::new(),
        };

        ns_zone
    }

    /*
    pub fn from_file() -> Self {

    }
    */
}

// Setter
impl NSZone {
    // Sets the values with a new value
    pub fn set_value(&mut self, value: Vec<ResourceRecord>) {
        self.value = value;
    }

    // Sets the childs with a new value
    pub fn set_childs(&mut self, childs: Vec<NSZone>) {
        self.childs = childs;
    }
}

// Getters
impl NSZone {
    // Gets the values from the node
    pub fn get_value(&self) -> Vec<ResourceRecord> {
        self.value.clone()
    }

    // Gets the childs from the node
    pub fn get_childs(&self) -> Vec<NSZone> {
        self.childs.clone()
    }
}