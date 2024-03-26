pub mod a_ch_rdata;
pub mod a_rdata;
pub mod cname_rdata;
pub mod hinfo_rdata;
pub mod mx_rdata;
pub mod ns_rdata;
pub mod ptr_rdata;
pub mod soa_rdata;
pub mod txt_rdata;
pub mod aaaa_rdata;
pub mod opt_rdata;
pub mod ds_rdata;
pub mod rrsig_rdata;
pub mod nsec_rdata;
pub mod dnskey_rdata;
pub mod nsec3_rdata;
pub mod nsec3param_rdata;
pub mod tsig_rdata;

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
use aaaa_rdata::AAAARdata;
use opt_rdata::OptRdata;
use ds_rdata::DsRdata;
use rrsig_rdata::RRSIGRdata;
use nsec_rdata::NsecRdata;
use dnskey_rdata::DnskeyRdata;
use nsec3_rdata::Nsec3Rdata;
use nsec3param_rdata::Nsec3ParamRdata;
use tsig_rdata::TSigRdata;

#[derive(Clone, PartialEq, Debug)]

/// Enumerates the differents types of `Rdata` struct.
pub enum Rdata {
    A(ARdata),
    ACH(AChRdata),
    MX(MxRdata),
    NS(NsRdata),
    PTR(PtrRdata),
    SOA(SoaRdata),
    TXT(TxtRdata),
    CNAME(CnameRdata),
    HINFO(HinfoRdata),
    ////// Define here more rdata types //////
    AAAA(AAAARdata),
    OPT(OptRdata),
    DS(DsRdata),
    RRSIG(RRSIGRdata),
    NSEC(NsecRdata),
    DNSKEY(DnskeyRdata),
    NSEC3(Nsec3Rdata),
    NSEC3PARAM(Nsec3ParamRdata),
    TSIG(TSigRdata),
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
    /// let rdata = Rdata::A(a_rdata);
    /// let bytes = rdata.to_bytes();
    /// assert_eq!(bytes, vec![127, 0, 0, 1]);
    /// ```
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Rdata::A(val) => val.to_bytes(),
            Rdata::ACH(val) => val.to_bytes(),
            Rdata::MX(val) => val.to_bytes(),
            Rdata::NS(val) => val.to_bytes(),
            Rdata::PTR(val) => val.to_bytes(),
            Rdata::SOA(val) => val.to_bytes(),
            Rdata::TXT(val) => val.to_bytes(),
            Rdata::AAAA(val) => val.to_bytes(),
            Rdata::CNAME(val) => val.to_bytes(),
            Rdata::HINFO(val) => val.to_bytes(),
            Rdata::OPT(val) => val.to_bytes(),
            Rdata::DS(val) => val.to_bytes(),
            Rdata::RRSIG(val) => val.to_bytes(),
            Rdata::NSEC(val) => val.to_bytes(),
            Rdata::DNSKEY(val) => val.to_bytes(),
            Rdata::NSEC3(val) => val.to_bytes(),
            Rdata::NSEC3PARAM(val) => val.to_bytes(),
            Rdata::TSIG(val) => val.to_bytes(),
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

                    Ok(Rdata::ACH(rdata.unwrap()))
                } else {
                    let rdata = ARdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);

                    match rdata {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(e);
                        }
                    }

                    Ok(Rdata::A(rdata.unwrap()))
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

                Ok(Rdata::NS(rdata.unwrap()))
            }
            5 => {
                let rdata = CnameRdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);

                match rdata {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                Ok(Rdata::CNAME(rdata.unwrap()))
            }
            6 => {
                let rdata = SoaRdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);

                match rdata {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                Ok(Rdata::SOA(rdata.unwrap()))
            }
            12 => {
                let rdata = PtrRdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);

                match rdata {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                Ok(Rdata::PTR(rdata.unwrap()))
            }
            13 => {
                let rdata = HinfoRdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);

                match rdata {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                Ok(Rdata::HINFO(rdata.unwrap()))
            }
            15 => {
                let rdata = MxRdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);

                match rdata {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                Ok(Rdata::MX(rdata.unwrap()))
            }
            16 => {
                let rdata = TxtRdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);

                match rdata {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                Ok(Rdata::TXT(rdata.unwrap()))
            }

            28 => {
                let rdata = AAAARdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);

                match rdata {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                Ok(Rdata::AAAA(rdata.unwrap()))
            }

            39 => {
                let rdata = CnameRdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);

                match rdata {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                Ok(Rdata::CNAME(rdata.unwrap()))
            }

            41 => {
                println!("OPT");
                let rdata = OptRdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);
                match rdata {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                    
                }
                Ok(Rdata::OPT(rdata.unwrap()))
            }

            43 => {
                let rdata = DsRdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);
                match rdata {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                    
                }
                Ok(Rdata::DS(rdata.unwrap()))
            }

            46 => {
                let rdata = RRSIGRdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);
                match rdata {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                    
                }
                Ok(Rdata::RRSIG(rdata.unwrap()))
            }

            47 => {
                let rdata = NsecRdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);
                match rdata {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                    
                }
                Ok(Rdata::NSEC(rdata.unwrap()))
            }

            48 => {
                let rdata = DnskeyRdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);
                match rdata {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                    
                }
                Ok(Rdata::DNSKEY(rdata.unwrap()))
            }

            50 => {
                let rdata = Nsec3Rdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);
                match rdata {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                    
                }
                Ok(Rdata::NSEC3(rdata.unwrap()))
            }

            51 => {
                let rdata = Nsec3ParamRdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);
                match rdata {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                    
                }
                Ok(Rdata::NSEC3PARAM(rdata.unwrap()))
            }
            
            250 => {
                let rdata = TSigRdata::from_bytes(&bytes[..bytes.len() - 4], full_msg);

                match rdata {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                Ok(Rdata::TSIG(rdata.unwrap()))
            }
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
    use crate::message::type_rtype::Rtype;
    use super:: a_ch_rdata::AChRdata;
    use super::a_rdata::ARdata;
    use super::cname_rdata::CnameRdata;
    use super::hinfo_rdata::HinfoRdata;
    use super::mx_rdata::MxRdata;
    use super::ns_rdata::NsRdata;
    use super::ptr_rdata::PtrRdata;
    use super::soa_rdata::SoaRdata;
    use super::txt_rdata::TxtRdata;
    use super::opt_rdata::OptRdata;
    use super::ds_rdata::DsRdata;
    use super::rrsig_rdata::RRSIGRdata;
    use super::nsec_rdata::NsecRdata;
    use super::dnskey_rdata::DnskeyRdata;
    use super::nsec3_rdata::Nsec3Rdata;
    use super::nsec3param_rdata::Nsec3ParamRdata;
    use super::tsig_rdata::TSigRdata;
    use super::aaaa_rdata::AAAARdata;
    use std::net::IpAddr;
    use std::vec;

    #[test]
    fn to_bytes_rdata(){
        let a_rdata = Rdata::A(ARdata::new());
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
        let a_rdata = Rdata::CNAME(cname_rdata);
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

        let a_rdata = Rdata::HINFO(hinfo_rdata);
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

        let a_rdata = Rdata::ACH(ach_rdata);
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
        let a_rdata = Rdata::MX(mx_rdata);
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

        let ns_rdata = Rdata::NS(ns_rdatas);
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
        let ptr_rdata = Rdata::PTR(ptr_rdatas);
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

        let soa_rdatas = Rdata::SOA(soa_rdata);
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

        let txt_rdatas = Rdata::TXT(txt_rdata);
        let bytes = txt_rdatas.to_bytes();

        let mut expected_bytes: Vec<u8> = Vec::new();
        for byte in bytes_to_test{
            expected_bytes.push(byte);
        }
        assert_eq!(bytes, expected_bytes);
    }

    #[test]
    fn to_bytes_tsigrdata(){
        let expected_bytes = vec![
        0x8, 0x68, 0x6D, 0x61, 0x63, 0x2D, 0x6D, 0x64,
        0x35, 0x7, 0x73, 0x69, 0x67, 0x2D, 0x61, 0x6C, 0x67,
        0x3, 0x72, 0x65, 0x67, 0x3, 0x69, 0x6E, 0x74, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x7, 0x5B, 0xCD, 0x15, 0x4, 0xD2, 0x0, 0x4, 0xA1, 0xB2, 0xC3, 0xD4,
        0x4, 0xD2, 0x0, 0x0, 0x0, 0x0
        ];

        let mut tsig_rdata = TSigRdata::new();

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("hmac-md5.sig-alg.reg.int"));
        tsig_rdata.set_algorithm_name(domain_name);
        tsig_rdata.set_time_signed(123456789);
        tsig_rdata.set_fudge(1234);
        tsig_rdata.set_mac_size(4);
        tsig_rdata.set_mac(vec![0xA1, 0xB2, 0xC3, 0xD4]);
        tsig_rdata.set_original_id(1234);
        tsig_rdata.set_error(0);
        tsig_rdata.set_other_len(0);
        tsig_rdata.set_other_data(Vec::new());

        let rdata = Rdata::TSIG(tsig_rdata);
        let bytes = rdata.to_bytes();

        assert_eq!(bytes, expected_bytes);
    }

    #[test]
    fn to_bytes_opt_rdata(){
        let expected_bytes = vec![
            0, 1, 0, 2, 6, 4];
        let mut opt_rdata = OptRdata::new();
        opt_rdata.set_option_code(1 as u16);
        opt_rdata.set_option_length(2 as u16);
        opt_rdata.set_option_data(vec![0x06, 0x04]);

        let rdata = Rdata::OPT(opt_rdata);
        let bytes = rdata.to_bytes();

        assert_eq!(bytes, expected_bytes);
    }

    #[test]
    fn to_bytes_dnskey_rdata(){
        let mut dnskey_rdata = DnskeyRdata::new(0, 0, 0, Vec::new());
        dnskey_rdata.set_flags(2 as u16);
        dnskey_rdata.set_protocol(3 as u8);
        dnskey_rdata.set_algorithm(4 as u8);
        let public_key_simple: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        dnskey_rdata.set_public_key(public_key_simple);

        let expected_bytes = vec![
            0, 2, 3, 4, 1, 2, 3, 4, 5, 6, 7, 8
        ];

        let rdata = Rdata::DNSKEY(dnskey_rdata);
        let bytes = rdata.to_bytes();

        assert_eq!(bytes, expected_bytes);
    }

    #[test]
    fn to_bytes_rrsig_rdata(){
        let mut rrsig_rdata = RRSIGRdata::new();
        rrsig_rdata.set_type_covered(Rtype::A);
        rrsig_rdata.set_algorithm(5);
        rrsig_rdata.set_labels(2);
        rrsig_rdata.set_original_ttl(3600);
        rrsig_rdata.set_signature_expiration(1630435200);
        rrsig_rdata.set_signature_inception(1630435200);
        rrsig_rdata.set_key_tag(1234);
        rrsig_rdata.set_signer_name(DomainName::new_from_str("example.com"));
        rrsig_rdata.set_signature(String::from("abcdefg"));

        let expected_bytes:Vec<u8> = vec![0, 1, 5, 2, 0, 0, 14, 16, 97, 46, 119, 128, 97,
        46, 119, 128, 4, 210, 7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109, 0, 97, 
        98, 99, 100, 101, 102, 103];

        let rdata = Rdata::RRSIG(rrsig_rdata);
        let bytes = rdata.to_bytes();

        assert_eq!(bytes, expected_bytes);
    }

    #[test]
    fn to_bytes_nsec_rdata(){
        let mut nsec_rdata = NsecRdata::new(DomainName::new_from_str("."), vec![]);

        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("host.example.com"));
        nsec_rdata.set_next_domain_name(domain_name);

        nsec_rdata.set_type_bit_maps(vec![Rtype::A, Rtype::MX, Rtype::RRSIG, Rtype::NSEC, Rtype::UNKNOWN(1234)]);
        
        let next_domain_name_bytes = vec![4, 104, 111, 115, 116, 7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109, 0];

        let bit_map_bytes_to_test = vec![0, 6, 64, 1, 0, 0, 0, 3, 
                                    4, 27, 0, 0, 0, 0, 0, 0, 0,
                                    0, 0, 0, 0, 0, 0, 0, 0, 0, 
                                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32];

        let bytes_to_test = [next_domain_name_bytes, bit_map_bytes_to_test].concat();

        let rdata = Rdata::NSEC(nsec_rdata);

        let bytes = rdata.to_bytes();

        assert_eq!(bytes, bytes_to_test);
    }

    #[test]
    fn to_bytes_ds_rdata(){
        let mut ds_rdata = DsRdata::new(0, 0, 0, vec![1, 2, 3, 4]);
        ds_rdata.set_key_tag(1);
        ds_rdata.set_algorithm(2);
        ds_rdata.set_digest_type(3);

        let bytes_to_test = [0, 1, 2, 3, 1, 2, 3, 4];

        let rdata = Rdata::DS(ds_rdata);
        let bytes = rdata.to_bytes();

        assert_eq!(bytes, bytes_to_test);
    }

    #[test]
    fn to_bytes_aaaa_rdata(){
        let mut aaaa_rdata = AAAARdata::new();
        aaaa_rdata.set_address(IpAddr::from([1,1,1,1,1,1,1,1]));

        let bytes_to_test = [0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1];

        let rdata = Rdata::AAAA(aaaa_rdata);
        let bytes = rdata.to_bytes();

        assert_eq!(bytes, bytes_to_test);
    }

    #[test]
    fn to_bytes_nsec3_rdata(){
        let nsec3_rdata = Nsec3Rdata::new(1, 2, 3, 
            4, "salt".to_string(), 22, "next_hashed_owner_name".to_string(), vec![Rtype::A, Rtype::MX, Rtype::RRSIG, Rtype::NSEC, Rtype::UNKNOWN(1234)]);

        let rdata = Rdata::NSEC3(nsec3_rdata);
        let bytes = rdata.to_bytes();

        let first_expected_bytes = vec![1, 2, 0, 3, 4, 115, 97, 108, 116, 22, 110, 101, 120, 116, 95, 104,
                                                97, 115, 104, 101, 100, 95, 111, 119, 110, 101, 114, 95, 110, 97, 109, 101];

        let bit_map_bytes_to_test = vec![0, 6, 64, 1, 0, 0, 0, 3, 
                                    4, 27, 0, 0, 0, 0, 0, 0, 0,
                                    0, 0, 0, 0, 0, 0, 0, 0, 0, 
                                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32];
            
        
        let expected_bytes = [&first_expected_bytes[..], &bit_map_bytes_to_test[..]].concat();

        assert_eq!(bytes, expected_bytes);
    }

    #[test]
    fn to_bytes_nsec3param_rdata(){
        let nsec3param_rdata = Nsec3ParamRdata::new(1, 2, 3, 
            4, "salt".to_string());

        let rdata = Rdata::NSEC3PARAM(nsec3param_rdata);
        let bytes = rdata.to_bytes();

        let expected_bytes = vec![1, 2, 0, 3, 4, 115, 97, 108, 116];

        assert_eq!(bytes, expected_bytes);
    }

    //from bytes tests
    #[test]
    fn from_bytes_a_ch_rdata(){
        let data_bytes = [4, 116, 101, 115, 116, 3, 99, 111, 109, 0, 0, 10, 0, 1, 0, 3];
        let rdata = Rdata::from_bytes(&data_bytes, &data_bytes).unwrap();
        let name = String::from("test.com");

        match rdata {
            Rdata::ACH(val) => {
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
            Rdata::A(val) => {
                assert_eq!(val.get_address(), IpAddr::from([128, 0, 0, 1]));
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
            Rdata::CNAME(val) => {
                assert_eq!(val.get_cname().get_name(), name);
            }
            _ => {}
        }     
    }

    #[test]
    fn from_bytes_dname_rdata(){
        let data_bytes = [5, 100, 110, 97, 109, 101, 0, 0, 39, 0, 1];
        let rdata = Rdata::from_bytes(&data_bytes, &data_bytes).unwrap();
        let name = String::from("dname");

        match rdata {
            Rdata::CNAME(val) => {
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
            Rdata::HINFO(val) => {
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
            Rdata::MX(val) => {
                assert_eq!(val.get_exchange().get_name(), domain_name.get_name());
                assert_eq!(val.get_preference(), 128);
            }
            _ => {}
        }     
    }

    #[test]
    fn from_bytes_ns_rdata(){
        let data_bytes = [4, 116, 101, 115, 116, 5, 116, 101, 115, 116, 50, 3, 99, 111, 109, 0, 0, 2, 0, 1];
        let rdata = Rdata::from_bytes(&data_bytes, &data_bytes).unwrap();
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test.test2.com"));

        match rdata {
            Rdata::NS(val) => {
                assert_eq!(val.get_nsdname().get_name(), domain_name.get_name());
            }
            _ => {}
        }     
    }

    #[test]
    fn from_bytes_ptr_rdata(){
        let data_bytes = [4, 116, 101, 115, 116, 5, 116, 101, 115, 116, 50, 3, 99, 111, 109, 0, 0, 12, 0, 1];
        let rdata = Rdata::from_bytes(&data_bytes, &data_bytes).unwrap();
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test.test2.com"));

        match rdata {
            Rdata::PTR(val) => {
                assert_eq!(val.get_ptrdname().get_name(), domain_name.get_name());
            }
            _ => {}
        }     
    }

    #[test]
    fn from_bytes_soa_rdata(){
        let data_bytes = [4, 116, 101, 115, 116,
        3, 99, 111, 109,
        0, 4, 116, 101, 115, 116,
        3, 99, 111, 109,
        0, 0, 0, 2, 0, 0, 0, 0, 8, 0, 0, 0, 4, 0, 0, 0, 2, 0, 0, 0, 1, 0, 6, 0, 1];
        let rdata = Rdata::from_bytes(&data_bytes, &data_bytes).unwrap();
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test.com"));

        match rdata {
            Rdata::SOA(val) => {
                assert_eq!(val.get_mname().get_name(), domain_name.get_name());
                assert_eq!(val.get_rname().get_name(), domain_name.get_name());
                assert_eq!(val.get_serial(), 512);
                assert_eq!(val.get_refresh(), 8);
                assert_eq!(val.get_retry(), 4);
                assert_eq!(val.get_expire(), 2);
                assert_eq!(val.get_minimum(), 1);
            }
            _ => {}
        }     
    }

    #[test]
    fn from_bytes_txt_rdata(){
        let data_bytes = [8, 100, 99, 99, 32, 116, 101, 115, 116, 0, 0, 16, 0, 1];
        let rdata = Rdata::from_bytes(&data_bytes, &data_bytes).unwrap();
        let text = vec!["dcc test".to_string()];

        match rdata {
            Rdata::TXT(val) => {
                assert_eq!(val.get_text(), text);
            }
            _ => {}
        }
    }

    #[test]
    fn from_bytes_tsig_rdata(){
        let data_bytes = vec![
        0x8, 0x68, 0x6D, 0x61, 0x63, 0x2D, 0x6D, 0x64,
        0x35, 0x7, 0x73, 0x69, 0x67, 0x2D, 0x61, 0x6C, 0x67,
        0x3, 0x72, 0x65, 0x67, 0x3, 0x69, 0x6E, 0x74, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x7, 0x5B, 0xCD, 0x15, 0x4, 0xD2, 0x0, 0x4, 0xA1, 0xB2, 0xC3, 0xD4,
        0x4, 0xD2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xFA, 0x0, 0x1
        ];
        let rdata = Rdata::from_bytes(&data_bytes, &data_bytes).unwrap();
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("hmac-md5.sig-alg.reg.int"));

        match rdata {
            Rdata::TSIG(val) => {
                assert_eq!(val.get_algorithm_name().get_name(), domain_name.get_name());
                assert_eq!(val.get_time_signed(), 123456789);
                assert_eq!(val.get_fudge(), 1234);
                assert_eq!(val.get_mac_size(), 4);
                assert_eq!(val.get_mac(), vec![0xA1, 0xB2, 0xC3, 0xD4]);
                assert_eq!(val.get_original_id(), 1234);
                assert_eq!(val.get_error(), 0);
                assert_eq!(val.get_other_len(), 0);
                assert_eq!(val.get_other_data(), Vec::new());
            }
            _ => {}
        }
    }

    #[test]
    fn from_bytes_opt_rdata(){
        let data_bytes = vec![
            0, 1, 0, 2, 6, 4, 0, 41, 0, 1
        ];
        let rdata = Rdata::from_bytes(&data_bytes, &data_bytes).unwrap();
        match rdata {
            Rdata::OPT(val) => {
                assert_eq!(val.get_option_code(), 1);
                assert_eq!(val.get_option_length(), 2);
                assert_eq!(val.get_option_data(), vec![0x06, 0x04]);
            }
            _ => {}
        }
    }

    #[test]
    fn from_bytes_dnskey_rdata(){
        let data_bytes = vec![
            0, 2, 3, 4, 1, 2, 3, 4, 5, 6, 7, 8, 0, 48, 0, 1
        ];
        let rdata = Rdata::from_bytes(&data_bytes, &data_bytes).unwrap();
        match rdata {
            Rdata::DNSKEY(val) => {
                assert_eq!(val.get_flags(), 2);
                assert_eq!(val.get_protocol(), 3);
                assert_eq!(val.get_algorithm(), 4);
                assert_eq!(val.get_public_key(), vec![1, 2, 3, 4, 5, 6, 7, 8]);
            }
            _ => {}
        }
    
    }

    #[test]
    fn from_bytes_rrsig_rdata(){
        let data_bytes:Vec<u8> = vec![0, 1, 5, 2, 0, 0, 14, 16, 97, 46, 119, 128, 97,
        46, 119, 128, 4, 210, 7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109, 0, 97, 
        98, 99, 100, 101, 102, 103, 0, 46, 0, 1];

        let rdata = Rdata::from_bytes(&data_bytes, &data_bytes).unwrap();

        match rdata {
            Rdata::RRSIG(val) => {
                assert_eq!(val.get_type_covered(), Rtype::A);
                assert_eq!(val.get_algorithm(), 5);
                assert_eq!(val.get_labels(), 2);
                assert_eq!(val.get_original_ttl(), 3600);
                assert_eq!(val.get_signature_expiration(), 1630435200);
                assert_eq!(val.get_signature_inception(), 1630435200);
                assert_eq!(val.get_key_tag(), 1234);
                assert_eq!(val.get_signer_name().get_name(), "example.com");
                assert_eq!(val.get_signature(), "abcdefg");
            }
            _ => {}
        }
    }

    #[test]
    fn from_bytes_nsec_rdata(){
        let next_domain_name_bytes = vec![4, 104, 111, 115, 116, 7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109, 0];

        let bit_map_bytes_to_test = vec![0, 6, 64, 1, 0, 0, 0, 3, 
                                    4, 27, 0, 0, 0, 0, 0, 0, 0,
                                    0, 0, 0, 0, 0, 0, 0, 0, 0, 
                                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32];

        let rdata_bytes = [next_domain_name_bytes, bit_map_bytes_to_test].concat();

        let extra_bytes = vec![0, 47, 0, 1];

        let data_bytes = [rdata_bytes, extra_bytes].concat();

        let rdata = Rdata::from_bytes(&data_bytes, &data_bytes).unwrap();

        match rdata {
            Rdata::NSEC(val) => {
                assert_eq!(val.get_next_domain_name().get_name(), String::from("host.example.com"));
                assert_eq!(val.get_type_bit_maps(), vec![Rtype::A, Rtype::MX, Rtype::RRSIG, Rtype::NSEC, Rtype::UNKNOWN(1234)]);
            }
            _ => {}
        }
    }

    #[test]
    fn from_bytes_ds_rdata(){
        let data_bytes = [0, 1, 2, 3, 1, 2, 3, 4, 0, 43, 0, 1];
        let rdata = Rdata::from_bytes(&data_bytes, &data_bytes).unwrap();
        match rdata {
            Rdata::DS(val) => {
                assert_eq!(val.get_key_tag(), 1);
                assert_eq!(val.get_algorithm(), 2);
                assert_eq!(val.get_digest_type(), 3);
                assert_eq!(val.get_digest(), vec![1, 2, 3, 4]);
            }
            _ => {}
        }
    }

    #[test]
    fn from_bytes_aaaa_rdata(){
        let data_bytes = [0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0, 28, 0, 1];
        let rdata = Rdata::from_bytes(&data_bytes, &data_bytes).unwrap();
        match rdata {
            Rdata::AAAA(val) => {
                assert_eq!(val.get_address(), IpAddr::from([1,1,1,1,1,1,1,1]));
            }
            _ => {}
        }
    }

    #[test]
    fn from_bytes_nsec3_rdata(){
        let first_bytes = vec![1, 2, 0, 3, 4, 115, 97, 108, 116, 22, 110, 101, 120, 116, 95, 104,
                                                97, 115, 104, 101, 100, 95, 111, 119, 110, 101, 114, 95, 110, 97, 109, 101];

        let bit_map_bytes_to_test = vec![0, 6, 64, 1, 0, 0, 0, 3, 
                                    4, 27, 0, 0, 0, 0, 0, 0, 0,
                                    0, 0, 0, 0, 0, 0, 0, 0, 0, 
                                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32];
        
        let extra_bytes = vec![0, 50, 0, 1];

        let data_bytes = [&first_bytes[..], &bit_map_bytes_to_test[..], &extra_bytes[..]].concat();

        let rdata = Rdata::from_bytes(&data_bytes, &data_bytes).unwrap();

        match rdata {
            Rdata::NSEC3(val) => {
                assert_eq!(val.get_hash_algorithm(), 1);
                assert_eq!(val.get_flags(), 2);
                assert_eq!(val.get_iterations(), 3);
                assert_eq!(val.get_salt_length(), 4);
                assert_eq!(val.get_salt(), "salt");
                assert_eq!(val.get_hash_length(), 22);
                assert_eq!(val.get_next_hashed_owner_name(), "next_hashed_owner_name");
                assert_eq!(val.get_type_bit_maps(), vec![Rtype::A, Rtype::MX, Rtype::RRSIG, Rtype::NSEC, Rtype::UNKNOWN(1234)]);
            }
            _ => {}
        } 
    }

    #[test]
    fn from_bytes_nsec3param_rdata(){
        let first_bytes = vec![1, 2, 0, 3, 4, 115, 97, 108, 116];
        
        let extra_bytes = vec![0, 51, 0, 1];

        let data_bytes = [&first_bytes[..], &extra_bytes[..]].concat();

        let rdata = Rdata::from_bytes(&data_bytes, &data_bytes).unwrap();

        match rdata {
            Rdata::NSEC3(val) => {
                assert_eq!(val.get_hash_algorithm(), 1);
                assert_eq!(val.get_flags(), 2);
                assert_eq!(val.get_iterations(), 3);
                assert_eq!(val.get_salt_length(), 4);
                assert_eq!(val.get_salt(), "salt");
            }
            _ => {}
        } 
    }

    #[test]
    #[should_panic]
    fn from_bytes_format_error(){
        let data_bytes = [];
        let _rdata = Rdata::from_bytes(&data_bytes, &data_bytes).unwrap();
    }
}