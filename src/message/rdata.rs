pub mod a_rdata;
pub mod cname_rdata;
pub mod hinfo_rdata;
pub mod mx_rdata;
pub mod ns_rdata;
pub mod ptr_rdata;
pub mod soa_rdata;
pub mod txt_rdata;

use crate::message::resource_record::{FromBytes, ToBytes};
use a_rdata::ARdata;
use cname_rdata::CnameRdata;
use hinfo_rdata::HinfoRdata;
use mx_rdata::MxRdata;
use ns_rdata::NsRdata;
use ptr_rdata::PtrRdata;
use soa_rdata::SoaRdata;
use txt_rdata::TxtRdata;

#[derive(Clone)]
/// This enum, enumerates the differents types of rdata struct
pub enum Rdata {
    SomeARdata(ARdata),
    SomeMxRdata(MxRdata),
    SomeNsRdata(NsRdata),
    SomePtrRdata(PtrRdata),
    SomeSoaRdata(SoaRdata),
    SomeTxtRdata(TxtRdata),
    SomeCnameRdata(CnameRdata),
    SomeHinfoRdata(HinfoRdata),
    //////// Define here more rdata types ////////
}

impl ToBytes for Rdata {
    /// Converts an Rdata to bytes
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Rdata::SomeARdata(val) => val.to_bytes(),
            Rdata::SomeMxRdata(val) => val.to_bytes(),
            Rdata::SomeNsRdata(val) => val.to_bytes(),
            Rdata::SomePtrRdata(val) => val.to_bytes(),
            Rdata::SomeSoaRdata(val) => val.to_bytes(),
            Rdata::SomeTxtRdata(val) => val.to_bytes(),
            Rdata::SomeCnameRdata(val) => val.to_bytes(),
            Rdata::SomeHinfoRdata(val) => val.to_bytes(),
        }
    }
}

impl FromBytes<Rdata> for Rdata {
    /// Given an array of bytes and a type code, returns a new Rdata
    fn from_bytes(bytes: &[u8]) -> Rdata {
        let type_code = (bytes[bytes.len() - 2] as u16) << 8 | bytes[bytes.len() - 1] as u16;
        let rdata = match type_code {
            1 => Rdata::SomeARdata(ARdata::from_bytes(&bytes[..bytes.len() - 2])),
            2 => Rdata::SomeNsRdata(NsRdata::from_bytes(&bytes[..bytes.len() - 2])),
            5 => Rdata::SomeCnameRdata(CnameRdata::from_bytes(&bytes[..bytes.len() - 2])),
            6 => Rdata::SomeSoaRdata(SoaRdata::from_bytes(&bytes[..bytes.len() - 2])),
            12 => Rdata::SomePtrRdata(PtrRdata::from_bytes(&bytes[..bytes.len() - 2])),
            13 => Rdata::SomeHinfoRdata(HinfoRdata::from_bytes(&bytes[..bytes.len() - 2])),
            15 => Rdata::SomeMxRdata(MxRdata::from_bytes(&bytes[..bytes.len() - 2])),
            16 => Rdata::SomeTxtRdata(TxtRdata::from_bytes(&bytes[..bytes.len() - 2])),
            _ => unreachable!(),
        };
        rdata
    }
}
