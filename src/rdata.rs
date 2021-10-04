use crate::resource_record::ToBytes;
use crate::txt_rdata::TxtRdata;

#[derive(Clone)]
/// This enum, enumerates the differents types of rdata struct
pub enum Rdata {
    SomeTxtRdata(TxtRdata),
    //////// Define here more rdata types ////////
}

/// Trait to get the struct inside the Rdata enum
pub trait Unwrap<T> {
    fn unwrap(&self) -> T;
}

impl Unwrap<TxtRdata> for Rdata {
    /// Returns the struct inside the Rdata
    fn unwrap(&self) -> TxtRdata {
        let unwrap = match self {
            Rdata::SomeTxtRdata(val) => val,
            //////// Implement here for more rdata types ///////////////
        };

        unwrap.clone()
    }
}

///////////////////////////////////////////////////////////
// Implement unwrap here for more rdata types
///////////////////////////////////////////////////////////

impl ToBytes for Rdata {
    /// Converts an Rdata to bytes
    fn to_bytes(&self) -> Vec<u8> {
        self.unwrap().to_bytes()
    }
}
