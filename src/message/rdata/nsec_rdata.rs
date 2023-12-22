use crate::message::resource_record::{FromBytes, ToBytes};
use crate::domain_name::DomainName;
use crate::message::type_rtype::Rtype;

#[derive(Clone, Debug, PartialEq)]
/// Struct for NSEC Rdata
/// [RFC 4034](https://tools.ietf.org/html/rfc4034#section-4.1)
///                        1 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 3 3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// /                      Next Domain Name                         /
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// /                       Type Bit Maps                           /
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

pub struct NsecRdata {
    pub next_domain_name: DomainName,
    pub type_bit_maps: Vec<Rtype>,
}

impl NsecRdata{
    /// Creates a new `NsecRdata` with next_domain_name and type_bit_maps
    pub fn new(next_domain_name: DomainName, type_bit_maps: Vec<Rtype>) -> Self {
        NsecRdata {
            next_domain_name,
            type_bit_maps,
        }
    }

    /// Returns the next_domain_name of the `NsecRdata`.
    /// # Example
    /// ```
    /// let nsec_rdata = NsecRdata::new(String::from("www.example.com"), vec![Rtype::A, Rtype::NS]);
    /// assert_eq!(nsec_rdata.get_next_domain_name().get_name(), String::from("www.example.com"));
    /// ```
    pub fn get_next_domain_name(&self) -> DomainName {
        self.next_domain_name.clone()
    }

    /// Returns the type_bit_maps of the `NsecRdata`.
    /// # Example
    /// ```
    /// let nsec_rdata = NsecRdata::new(String::from("www.example.com"), vec![Rtype::A, Rtype::NS]);
    /// assert_eq!(nsec_rdata.get_type_bit_maps(), vec![Rtype::A, Rtype::NS]);
    /// ```
    pub fn get_type_bit_maps(&self) -> Vec<Rtype> {
        self.type_bit_maps.clone()
    }
}