use crate::message::resource_record::{FromBytes, ToBytes};
use crate::message::type_rtype::Rtype;

#[derive(Clone, PartialEq, Debug)]
/// Struct for the NSEC3 Rdata
/// [RFC 5155](https://tools.ietf.org/html/rfc5155#section-3.2)
/// ```text
/// 3.2.  The NSEC3 Wire Format
/// The RDATA of the NSEC3 RR is as shown below:
///
/// 1 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 3 3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |   Hash Alg.   |     Flags     |          Iterations           |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |  Salt Length  |                     Salt                      /
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |  Hash Length  |             Next Hashed Owner Name            /
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// /                         Type Bit Maps                         /
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```

pub struct Nsec3 {
    hash_algorithm: u8,
    flags: u8,
    iterations: u16,
    salt: &str,
    next_hashed_owner_name: &str,
    type_bit_maps: Vec<Rtype>,
}