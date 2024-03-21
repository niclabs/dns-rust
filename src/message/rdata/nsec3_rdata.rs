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

pub struct Nsec3Rdata {
    hash_algorithm: u8,
    flags: u8,
    iterations: u16,
    salt_length: u8,
    salt: &'static str,
    hash_length: u8,
    next_hashed_owner_name: &'static str,
    type_bit_maps: Vec<Rtype>,
}

impl Nsec3Rdata {
    /// Create a new NSEC3 Rdata
    pub fn new(
        hash_algorithm: u8,
        flags: u8,
        iterations: u16,
        salt_length: u8,
        salt: &'static str,
        hash_length: u8,
        next_hashed_owner_name: &'static str,
        type_bit_maps: Vec<Rtype>,
    ) -> Nsec3Rdata {
        Nsec3Rdata {
            hash_algorithm,
            flags,
            iterations,
            salt_length,
            salt,
            hash_length,
            next_hashed_owner_name,
            type_bit_maps,
        }
    }

    /// Getter for the hash_algorithm
    pub fn get_hash_algorithm(&self) -> u8 {
        self.hash_algorithm.clone()
    }

    /// Getter for the flags
    pub fn get_flags(&self) -> u8 {
        self.flags.clone()
    }

    /// Getter for the iterations
    pub fn get_iterations(&self) -> u16 {
        self.iterations.clone()
    }

    /// Getter for the salt_length
    pub fn get_salt_length(&self) -> u8 {
        self.salt_length.clone()
    }

    /// Getter for the salt
    pub fn get_salt(&self) -> &'static str {
        self.salt
    }

    /// Getter for the hash_length
    pub fn get_hash_length(&self) -> u8 {
        self.hash_length.clone()
    }

    /// Getter for the next_hashed_owner_name
    pub fn get_next_hashed_owner_name(&self) -> &'static str {
        self.next_hashed_owner_name
    }

    /// Getter for the type_bit_maps
    pub fn get_type_bit_maps(&self) -> Vec<Rtype> {
        self.type_bit_maps.clone()
    }
}

impl Nsec3Rdata {
    /// Setters for the NSEC3 Rdata
    
    /// Setter for the hash_algorithm
    pub fn set_hash_algorithm(&mut self, hash_algorithm: u8) {
        self.hash_algorithm = hash_algorithm;
    }

    /// Setter for the flags
    pub fn set_flags(&mut self, flags: u8) {
        self.flags = flags;
    }
}