pub mod header;
pub mod question;
pub mod rdata;
pub mod resource_record;

use crate::message::header::Header;
use crate::message::question::Question;
use crate::message::resource_record::ResourceRecord;
use std::vec::Vec;

/// Structs that represents a dns message
pub struct DnsMessage {
    header: Header,
    question: Question,
    answer: Vec<ResourceRecord>,
    authority: Vec<ResourceRecord>,
    additional: Vec<ResourceRecord>,
}
