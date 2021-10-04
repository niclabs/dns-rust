use crate::resource_record::ToBytes;
use crate::txt_rdata::TxtRdata;

#[derive(Clone)]
pub enum Rdata {
    SomeTxtRdata(TxtRdata),
    //Define here more rdata types
}

pub trait Unwrap<T> {
    fn unwrap(&self) -> T;
}

impl Unwrap<TxtRdata> for Rdata {
    fn unwrap(&self) -> TxtRdata {
        let unwrap = match self {
            Rdata::SomeTxtRdata(val) => val,
        };

        unwrap.clone()
    }
}

// Implement unwrap here for more rdata types
//
//////

impl ToBytes for Rdata {
    fn to_bytes(&self) -> Vec<u8> {
        self.unwrap().to_bytes()
    }
}
