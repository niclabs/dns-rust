pub mod a_ch_rdata;
pub mod a_rdata;
pub mod cname_rdata;
pub mod hinfo_rdata;
pub mod mx_rdata;
pub mod ns_rdata;
pub mod ptr_rdata;
pub mod soa_rdata;
pub mod txt_rdata;

use crate::message::resource_record::{FromBytes, ToBytes};
use a_ch_rdata::AChRdata;
use a_rdata::ARdata;
use cname_rdata::CnameRdata;
use hinfo_rdata::HinfoRdata;
use mx_rdata::MxRdata;
use ns_rdata::NsRdata;
use ptr_rdata::PtrRdata;
use soa_rdata::SoaRdata;
use txt_rdata::TxtRdata;

#[derive(Clone, PartialEq, Debug)]

/// Enumerates the differents types of `Rdata` struct.
pub enum Rdata {
    SomeARdata(ARdata),
    SomeAChRdata(AChRdata),
    SomeMxRdata(MxRdata),
    SomeNsRdata(NsRdata),
    SomePtrRdata(PtrRdata),
    SomeSoaRdata(SoaRdata),
    SomeTxtRdata(TxtRdata),
    SomeCnameRdata(CnameRdata),
    SomeHinfoRdata(HinfoRdata),
    ////// Define here more rdata types //////
}

impl ToBytes for Rdata {
    /// Converts an `Rdata` to bytes.
    /// 
    /// # Examples
    /// ```
    /// use dns_message_parser::message::rdata::Rdata;
    /// use dns_message_parser::message::rdata::a_rdata::ARdata;
    /// use dns_message_parser::message::resource_record::ToBytes;
    ///     
    /// let mut a_rdata = ARdata::new();
    /// a_rdata.set_address([127, 0, 0, 1]);
    /// let rdata = Rdata::SomeARdata(a_rdata);
    /// let bytes = rdata.to_bytes();
    /// assert_eq!(bytes, vec![127, 0, 0, 1]);
    /// ```
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Rdata::SomeARdata(val) => val.to_bytes(),
            Rdata::SomeAChRdata(val) => val.to_bytes(),
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

impl FromBytes<Result<Rdata, &'static str>> for Rdata {
    /// Given an array of bytes and a type in its code form, returns a new `Rdata`.
    fn from_bytes(bytes: &[u8], full_msg: &[u8]) -> Result<Rdata, &'static str> {
        let type_code = (bytes[bytes.len() - 4] as u16) << 8 | bytes[bytes.len() - 3] as u16;
        let class = (bytes[bytes.len() - 2] as u16) << 8 | bytes[bytes.len() - 1] as u16;

        let especific_rdata = match type_code {
            1 => {
                if class == 3 {
                    let rdata = AChRdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);

                    match rdata {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(e);
                        }
                    }

                    Ok(Rdata::SomeAChRdata(rdata.unwrap()))
                } else {
                    let rdata = ARdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);

                    match rdata {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(e);
                        }
                    }

                    Ok(Rdata::SomeARdata(rdata.unwrap()))
                }
            }
            2 => {
                let rdata = NsRdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);

                match rdata {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                Ok(Rdata::SomeNsRdata(rdata.unwrap()))
            }
            5 => {
                let rdata = CnameRdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);

                match rdata {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                Ok(Rdata::SomeCnameRdata(rdata.unwrap()))
            }
            6 => {
                let rdata = SoaRdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);

                match rdata {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                Ok(Rdata::SomeSoaRdata(rdata.unwrap()))
            }
            12 => {
                let rdata = PtrRdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);

                match rdata {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                Ok(Rdata::SomePtrRdata(rdata.unwrap()))
            }
            13 => {
                let rdata = HinfoRdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);

                match rdata {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                Ok(Rdata::SomeHinfoRdata(rdata.unwrap()))
            }
            15 => {
                let rdata = MxRdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);

                match rdata {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                Ok(Rdata::SomeMxRdata(rdata.unwrap()))
            }
            16 => {
                let rdata = TxtRdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);

                match rdata {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                Ok(Rdata::SomeTxtRdata(rdata.unwrap()))
            }
            //////////////// Replace the next line when AAAA is implemented ////////////
            28 => {
                let rdata = TxtRdata::new(vec!["AAAA".to_string()]);

                Ok(Rdata::SomeTxtRdata(rdata))
            }
            ////////////////////////////////////////////////////////////
            //////////////// Replace the next line when type  OPT is implemented ////////////
            41 => {
                println!("OPT");
                let rdata = TxtRdata::new(vec!["OPT".to_string()]);

                Ok(Rdata::SomeTxtRdata(rdata))
            }
            //////////////////////////////////////////////////////////////

            _ => Err("Format Error"),
        };

        especific_rdata
    }
}
#[cfg(test)]
mod resolver_query_tests {
    use crate::domain_name::DomainName;
    use crate::message::resource_record::{ToBytes, FromBytes};
    use crate::message::rdata::Rdata;
    use super:: a_ch_rdata::AChRdata;
    use super::a_rdata::ARdata;
    use super::cname_rdata::CnameRdata;
    use super::hinfo_rdata::HinfoRdata;
    use super::mx_rdata::MxRdata;
    use super::ns_rdata::NsRdata;
    use super::ptr_rdata::PtrRdata;
    use super::soa_rdata::SoaRdata;
    use super::txt_rdata::TxtRdata;

    #[test]
    fn to_bytes_rdata(){
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let bytes= a_rdata.to_bytes();
        let mut expected_bytes: Vec<u8> = Vec::new();
        expected_bytes.push(0);
        expected_bytes.push(0);
        expected_bytes.push(0);
        expected_bytes.push(0);
        assert_eq!(bytes, expected_bytes);
    }

    #[test]
    fn to_bytes_cname(){
        let mut cname_rdata = CnameRdata::new();

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("cname"));
        cname_rdata.set_cname(domain_name);

        let bytes_to_test: [u8; 7] = [5, 99, 110, 97, 109, 101, 0];
        let a_rdata = Rdata::SomeCnameRdata(cname_rdata);
        let bytes = a_rdata.to_bytes();
        let mut expected_bytes: Vec<u8> = Vec::new();
        for byte in bytes_to_test{
            expected_bytes.push(byte);
        }
        assert_eq!(bytes, expected_bytes);
    }

    #[test]
    fn to_bytes_hinfo(){
        let mut hinfo_rdata = HinfoRdata::new();

        hinfo_rdata.set_cpu(String::from("cpu"));
        hinfo_rdata.set_os(String::from("os"));

        let bytes_to_test: [u8; 7] = [99, 112, 117, 0, 111, 115, 0];

        let a_rdata = Rdata::SomeHinfoRdata(hinfo_rdata);
        let bytes = a_rdata.to_bytes();
        let mut expected_bytes: Vec<u8> = Vec::new();
        for byte in bytes_to_test{
            expected_bytes.push(byte);
        }
        assert_eq!(bytes, expected_bytes);
    }

    #[test]
    fn to_bytes_achrdata(){
        let mut domain_name = DomainName::new();
        let name = String::from("test.com");
        domain_name.set_name(name.clone());

        let mut ach_rdata = AChRdata::new();
        ach_rdata.set_ch_address(10);
        ach_rdata.set_domain_name(domain_name);

        let bytes_to_test = [4, 116, 101, 115, 116, 3, 99, 111, 109, 0, 0, 10];

        let a_rdata = Rdata::SomeAChRdata(ach_rdata);
        let bytes = a_rdata.to_bytes();
        let mut expected_bytes: Vec<u8> = Vec::new();
        for byte in bytes_to_test{
            expected_bytes.push(byte);
        }
        assert_eq!(bytes, expected_bytes);
    }
    
    #[test]
    fn to_bytes_mxrdata(){
        let mut mx_rdata = MxRdata::new();

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test.com"));

        mx_rdata.set_exchange(domain_name);
        mx_rdata.set_preference(128);

        let bytes_to_test: [u8; 12] = [0, 128, 4, 116, 101, 115, 116, 3, 99, 111, 109, 0];
        let a_rdata = Rdata::SomeMxRdata(mx_rdata);
        let bytes = a_rdata.to_bytes();

        let mut expected_bytes: Vec<u8> = Vec::new();
        for byte in bytes_to_test{
            expected_bytes.push(byte);
        }
        assert_eq!(bytes, expected_bytes);
    }

    #[test]
    fn to_bytes_nsrdata(){
        let mut domain_name = DomainName::new();

        let bytes_to_test: Vec<u8> = vec![
            4, 116, 101, 115, 116, 5, 116, 101, 115, 116, 50, 3, 99, 111, 109, 0,
        ];
        domain_name.set_name(String::from("test.test2.com"));

        let mut ns_rdatas = NsRdata::new();
        ns_rdatas.set_nsdname(domain_name);

        let ns_rdata = Rdata::SomeNsRdata(ns_rdatas);
        let bytes = ns_rdata.to_bytes();

        let mut expected_bytes: Vec<u8> = Vec::new();
        for byte in bytes_to_test{
            expected_bytes.push(byte);
        }
        assert_eq!(bytes, expected_bytes);
    }

    #[test]
    fn to_bytes_ptrrdata(){
        let mut domain_name = DomainName::new();
        let bytes_to_test: Vec<u8> = vec![
            4, 116, 101, 115, 116, 5, 116, 101, 115, 116, 50, 3, 99, 111, 109, 0,
        ];
        domain_name.set_name(String::from("test.test2.com"));

        let mut ptr_rdatas = PtrRdata::new();
        ptr_rdatas.set_ptrdname(domain_name);
        let ptr_rdata = Rdata::SomePtrRdata(ptr_rdatas);
        let bytes = ptr_rdata.to_bytes();

        let mut expected_bytes: Vec<u8> = Vec::new();
        for byte in bytes_to_test{
            expected_bytes.push(byte);
        }
        assert_eq!(bytes, expected_bytes);
    }
    
    #[test]
    fn to_bytes_soardata(){
        let mut soa_rdata = SoaRdata::new();

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test.com"));

        soa_rdata.set_mname(domain_name.clone());
        soa_rdata.set_rname(domain_name);
        soa_rdata.set_serial(512);
        soa_rdata.set_refresh(8);
        soa_rdata.set_retry(4);
        soa_rdata.set_expire(2);
        soa_rdata.set_minimum(1);

        let bytes_to_test: [u8; 40] = [
            4, 116, 101, 115, 116, 3, 99, 111, 109, 0, 4, 116, 101, 115, 116, 3, 99, 111, 109, 0,
            0, 0, 2, 0, 0, 0, 0, 8, 0, 0, 0, 4, 0, 0, 0, 2, 0, 0, 0, 1,
        ];

        let soa_rdatas = Rdata::SomeSoaRdata(soa_rdata);
        let bytes = soa_rdatas.to_bytes();

        let mut expected_bytes: Vec<u8> = Vec::new();
        for byte in bytes_to_test{
            expected_bytes.push(byte);
        }
        assert_eq!(bytes, expected_bytes);
    }

    #[test]
    fn to_bytes_txtrdata(){
        let text = vec!["dcc test".to_string()];
        let txt_rdata = TxtRdata::new(text);

        let bytes_to_test = [8, 100, 99, 99, 32, 116, 101, 115, 116];

        let txt_rdatas = Rdata::SomeTxtRdata(txt_rdata);
        let bytes = txt_rdatas.to_bytes();

        let mut expected_bytes: Vec<u8> = Vec::new();
        for byte in bytes_to_test{
            expected_bytes.push(byte);
        }
        assert_eq!(bytes, expected_bytes);
    }

    //from bytes tests
    #[test]
    fn from_bytes_a_ch_rdata(){
        let data_bytes = [4, 116, 101, 115, 116, 3, 99, 111, 109, 0, 0, 10, 0, 1, 0, 3];
        let rdata = Rdata::from_bytes(&data_bytes, &data_bytes).unwrap();
        let name = String::from("test.com");

        match rdata {
            Rdata::SomeAChRdata(val) => {
                assert_eq!(val.get_ch_address(), 10);
                assert_eq!(val.get_domain_name().get_name(), name);
            }
            _ => {}
        }
    }

    #[test]
    fn from_bytes_a_rdata(){
        let data_bytes = [128, 0, 0, 1, 0, 1, 0, 1];
        let rdata = Rdata::from_bytes(&data_bytes, &data_bytes).unwrap();
        match rdata {
            Rdata::SomeARdata(val) => {
                assert_eq!(val.get_address(), [128, 0, 0, 1]);
            }
            _ => {}
        }
    }

    #[test]
    fn from_bytes_cname_rdata(){
        let data_bytes = [5, 99, 110, 97, 109, 101, 0, 0, 5, 0, 1];
        let rdata = Rdata::from_bytes(&data_bytes, &data_bytes).unwrap();
        let name = String::from("cname");

        match rdata {
            Rdata::SomeCnameRdata(val) => {
                assert_eq!(val.get_cname().get_name(), name);
            }
            _ => {}
        }     
    }

    #[test]
    fn from_bytes_hinfo_rdata(){
        let data_bytes = [99, 112, 117, 0, 111, 115, 0, 0, 13, 0, 1];
        let rdata = Rdata::from_bytes(&data_bytes, &data_bytes).unwrap();
        let cpu = String::from("cpu");
        let os = String::from("os");

        match rdata {
            Rdata::SomeHinfoRdata(val) => {
                assert_eq!(val.get_cpu(), cpu);
                assert_eq!(val.get_os(), os);
            }
            _ => {}
        }     
    }

    #[test]
    fn from_bytes_mx_rdata(){
        let data_bytes = [0, 128, 4, 116, 101, 115, 116, 3, 99, 111, 109, 0, 0, 15, 0, 1];
        let rdata = Rdata::from_bytes(&data_bytes, &data_bytes).unwrap();
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test.com"));

        match rdata {
            Rdata::SomeMxRdata(val) => {
                assert_eq!(val.get_exchange().get_name(), domain_name.get_name());
                assert_eq!(val.get_preference(), 128);
            }
            _ => {}
        }     
    }
}