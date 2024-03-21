use crate::message::resource_record::{FromBytes, ToBytes};
use crate::message::type_rtype::Rtype;
use crate::message::rdata::NsecRdata;

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

impl ToBytes for Nsec3Rdata {
    /// Convert the NSEC3 Rdata to bytes
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        let hash_algorithm: u8 = self.get_hash_algorithm();
        bytes.push(hash_algorithm);
        let flags: u8 = self.get_flags();
        bytes.push(flags);
        let iterations: u16 = self.get_iterations();
        bytes.extend_from_slice(&iterations.to_be_bytes());
        let salt_length: u8 = self.get_salt_length();
        bytes.push(salt_length);
        let salt: &str = self.get_salt();
        bytes.extend_from_slice(salt.as_bytes());
        let hash_length: u8 = self.get_hash_length();
        bytes.push(hash_length);
        let next_hashed_owner_name: &str = self.get_next_hashed_owner_name();
        bytes.extend_from_slice(next_hashed_owner_name.as_bytes());
        let type_bit_maps: Vec<Rtype> = self.get_type_bit_maps();

        let mut enconded_type_bit_maps: Vec<u8> = Vec::new();
        let mut current_window: Option<u8> = None;
        let mut current_bitmap: Vec<u8> = Vec::new();

        for rtype in type_bit_maps {
            let window = match rtype {
                Rtype::UNKNOWN(rr_type) => (rr_type / 256) as u8,
                _ => (Rtype::from_rtype_to_int(rtype) / 256) as u8,
            };

            if let Some(current_window_value) = current_window {
                if current_window_value == window {
                    NsecRdata::add_rtype_to_bitmap(&rtype, &mut current_bitmap);
                    continue;
                }
                else {
                    enconded_type_bit_maps.push(current_window_value);
                    enconded_type_bit_maps.push(current_bitmap.len() as u8);
                    enconded_type_bit_maps.extend_from_slice(&current_bitmap);
                }
            }

            // New window
            current_window = Some(window);
            current_bitmap.clear();
            NsecRdata::add_rtype_to_bitmap(&rtype, &mut current_bitmap);
        }

        if let Some(current_window_value) = current_window {
            enconded_type_bit_maps.push(current_window_value);
            enconded_type_bit_maps.push(current_bitmap.len() as u8);
            enconded_type_bit_maps.extend_from_slice(&current_bitmap);
        }

        bytes.extend_from_slice(&enconded_type_bit_maps);

        bytes
    }
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

    /// Setter for the iterations
    pub fn set_iterations(&mut self, iterations: u16) {
        self.iterations = iterations;
    }

    /// Setter for the salt_length
    pub fn set_salt_length(&mut self, salt_length: u8) {
        self.salt_length = salt_length;
    }

    /// Setter for the salt
    pub fn set_salt(&mut self, salt: &'static str) {
        self.salt = salt;
    }

    /// Setter for the hash_length
    pub fn set_hash_length(&mut self, hash_length: u8) {
        self.hash_length = hash_length;
    }

    /// Setter for the next_hashed_owner_name
    pub fn set_next_hashed_owner_name(&mut self, next_hashed_owner_name: &'static str) {
        self.next_hashed_owner_name = next_hashed_owner_name;
    }

    /// Setter for the type_bit_maps
    pub fn set_type_bit_maps(&mut self, type_bit_maps: Vec<Rtype>) {
        self.type_bit_maps = type_bit_maps;
    }
}