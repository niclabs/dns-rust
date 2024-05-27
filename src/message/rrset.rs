use crate::message::Rtype;
use crate::message::Rclass;
use crate::message::resource_record::ResourceRecord;
use std::collections::HashSet;

/// Represents a set of resource records (RRset).
#[derive(Debug)]
pub struct RRset {
    /// The name of the domain associated with this RRset.
    name: String,
    /// The type of resource record in this RRset.
    rtype: Rtype,
    /// The class of resource record in this RRset.
    rclass: Rclass,
    /// The time to live (TTL) value for records in this RRset.
    ttl: u32,
    /// The set of resource records belonging to this RRset.
    records: HashSet<ResourceRecord>,
}

impl RRset {
    /// Creates a new RRset.
    pub fn new(name: String, rtype: Rtype, rclass: Rclass, ttl: u32) -> RRset {
        RRset {
            name,
            rtype,
            rclass,
            ttl,
            records: HashSet::new(),
        }
    }

    /// Adds a resource record to this RRset.
    pub fn add_record(&mut self, record: ResourceRecord) {
        self.records.insert(record);
    }

    /// Gets the name of the domain associated with this RRset.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Gets the type of resource record in this RRset.
    pub fn get_type(&self) -> Rtype {
        self.rtype
    }

    /// Gets the class of resource record in this RRset.
    pub fn get_class(&self) -> Rclass {
        self.rclass
    }

    /// Gets the time to live (TTL) value for records in this RRset.
    pub fn get_ttl(&self) -> u32 {
        self.ttl
    }

    /// Gets the set of resource records belonging to this RRset.
    pub fn get_records(&self) -> &HashSet<ResourceRecord> {
        &self.records
    }
}
