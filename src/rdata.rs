use crate::a_rdata::ARdata;
use crate::mx_rdata::MxRdata;
use crate::ns_rdata::NsRdata;
use crate::ptr_rdata::PtrRdata;
use crate::resource_record::ToBytes;
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

/// Trait to get the struct inside the Rdata enum
pub trait Unwrap<T> {
    fn unwrap(&self) -> T;
}

impl Unwrap<ARdata> for Rdata {
    /// Returns the struct inside the Rdata
    fn unwrap(&self) -> ARdata {
        let unwrap = match self {
            Rdata::SomeARdata(val) => val,
            _ => panic!("Can't unwrap"),
        };

        unwrap.clone()
    }
}

impl Unwrap<MxRdata> for Rdata {
    /// Returns the struct inside the Rdata
    fn unwrap(&self) -> MxRdata {
        let unwrap = match self {
            Rdata::SomeMxRdata(val) => val,
            _ => panic!("Can't unwrap"),
        };

        unwrap.clone()
    }
}

impl Unwrap<NsRdata> for Rdata {
    /// Returns the struct inside the Rdata
    fn unwrap(&self) -> NsRdata {
        let unwrap = match self {
            Rdata::SomeNsRdata(val) => val,
            _ => panic!("Can't unwrap"),
        };

        unwrap.clone()
    }
}

impl Unwrap<PtrRdata> for Rdata {
    /// Returns the struct inside the Rdata
    fn unwrap(&self) -> PtrRdata {
        let unwrap = match self {
            Rdata::SomePtrRdata(val) => val,
            _ => panic!("Can't unwrap"),
        };

        unwrap.clone()
    }
}

impl Unwrap<SoaRdata> for Rdata {
    /// Returns the struct inside the Rdata
    fn unwrap(&self) -> SoaRdata {
        let unwrap = match self {
            Rdata::SomeSoaRdata(val) => val,
            _ => panic!("Can't unwrap"),
        };

        unwrap.clone()
    }
}

impl Unwrap<TxtRdata> for Rdata {
    /// Returns the struct inside the Rdata
    fn unwrap(&self) -> TxtRdata {
        let unwrap = match self {
            Rdata::SomeTxtRdata(txt_val) => txt_val,
            _ => panic!("Can't unwrap"),
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
