use crate::message::resource_record::{FromBytes, ResourceRecord, ToBytes};
use crate::message::rdata::Rdata;
use crate::message::Rclass;
use crate::message::Rtype;


#[derive(Clone, Debug, PartialEq)]
/// Struct for OPT Rdata
/// [RFC 6891](https://tools.ietf.org/html/rfc6891#section-6.1.2)
/// +0 (MSB)                            +1 (LSB)
/// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
/// 0: |                          OPTION-CODE                          |
/// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
/// 2: |                         OPTION-LENGTH                         |
/// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
/// 4: |                                                               |
/// /                          OPTION-DATA                          /
/// /                                                               /
/// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+

pub struct OptRdata {
    pub option_code: u16,
    pub option_length: u16,
    pub option_data: Vec<u8>,
}
