use crate::a_rdata::ARdata;
use crate::mx_rdata::MxRdata;
use crate::ns_rdata::NsRdata;
use crate::ptr_rdata::PtrRdata;
use crate::resource_record::{FromBytes, ToBytes};
use crate::soa_rdata::SoaRdata;
use crate::txt_rdata::TxtRdata;

#[derive(Clone)]
/// This enum, enumerates the differents types of rdata struct
pub enum Rdata {
    SomeARdata(ARdata),
    SomeMxRdata(MxRdata),
    SomeNsRdata(NsRdata),
    SomePtrRdata(PtrRdata),
    SomeSoaRdata(SoaRdata),
    SomeTxtRdata(TxtRdata),
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
            6 => Rdata::SomeSoaRdata(SoaRdata::from_bytes(&bytes[..bytes.len() - 2])),
            12 => Rdata::SomePtrRdata(PtrRdata::from_bytes(&bytes[..bytes.len() - 2])),
            15 => Rdata::SomeMxRdata(MxRdata::from_bytes(&bytes[..bytes.len() - 2])),
            16 => Rdata::SomeTxtRdata(TxtRdata::from_bytes(&bytes[..bytes.len() - 2])),
            _ => unreachable!(),
        };
        rdata
    }
}
