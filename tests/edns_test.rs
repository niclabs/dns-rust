use std::{net::IpAddr, str::FromStr};
use std::net::Ipv4Addr;
use std::time::Duration;
use dns_rust::edns::opt_option::option_code::OptionCode;
use dns_rust::edns::opt_option::OptOption;
use dns_rust::{async_resolver::{config::ResolverConfig, AsyncResolver}, client::client_error::ClientError, domain_name::DomainName, message::{rclass::Rclass, rdata::Rdata, resource_record::{ResourceRecord, ToBytes}, rrtype::Rrtype, DnsMessage}};
use dns_rust::async_resolver::server_info::ServerInfo;
use dns_rust::edns::opt_option::option_data::OptionData;
use dns_rust::message::rdata::a_rdata::ARdata;

async fn query_from_ip_with_edns(domain_name: &str,
                                 protocol: &str,
                                 qtype: &str,
                                 max_payload: Option<u16>,
                                 version: u8,
                                 do_bit: bool,
                                 option: Option<Vec<OptionCode>>,
                                 ip_addr: IpAddr) -> Result<DnsMessage, ClientError> {

    let mut config = ResolverConfig::default();
    config.add_edns0(max_payload, version, do_bit, option);


    config.set_name_servers(vec![ServerInfo::new_from_addr_with_default_size(ip_addr, Duration::from_secs(2))]);
    let mut resolver = AsyncResolver::new(config);

    let response = resolver.lookup(
        domain_name,
        protocol,
        qtype,
        "IN").await;

    response.map(|lookup_response| lookup_response.to_dns_msg())
}

async fn query_response_edns(domain_name: &str,
                             protocol: &str,
                             qtype: &str,
                             max_payload: Option<u16>,
                             version: u8,
                             do_bit: bool,
                             option: Option<Vec<OptionCode>>) -> Result<DnsMessage, ClientError> {

    let mut config = ResolverConfig::default();
    config.add_edns0(max_payload, version, do_bit, option);
    let mut resolver = AsyncResolver::new(config);

    let response = resolver.lookup(
        domain_name,
        protocol,
        qtype,
        "IN").await;

    response.map(|lookup_response| lookup_response.to_dns_msg())
}

#[tokio::test]
async fn query_a_type_edns() {
    let response = query_response_edns("example.com", "UDP", "A", Some(1024), 0, false, Some(vec![OptionCode::NSID])).await;

    if let Ok(rrs) = response {
        println!("{}", rrs);
        assert_eq!(rrs.get_answer().len(), 6);
        let rdata = rrs.get_answer();
        let data = vec![(&rdata[0]).get_rdata(), (&rdata[1]).get_rdata(),
                        (&rdata[2]).get_rdata(), (&rdata[3]).get_rdata(),
                        (&rdata[4]).get_rdata(), (&rdata[5]).get_rdata()];

        assert!(data.contains(&Rdata::A(ARdata::new_from_addr(IpAddr::V4(Ipv4Addr::new(23, 192, 228, 80))))));
        assert!(data.contains(&Rdata::A(ARdata::new_from_addr(IpAddr::V4(Ipv4Addr::new(23, 192, 228, 84))))));
        assert!(data.contains(&Rdata::A(ARdata::new_from_addr(IpAddr::V4(Ipv4Addr::new(23, 215, 0, 136))))));
        assert!(data.contains(&Rdata::A(ARdata::new_from_addr(IpAddr::V4(Ipv4Addr::new(23, 215, 0, 138))))));
        assert!(data.contains(&Rdata::A(ARdata::new_from_addr(IpAddr::V4(Ipv4Addr::new(96, 7, 128, 175))))));
        assert!(data.contains(&Rdata::A(ARdata::new_from_addr(IpAddr::V4(Ipv4Addr::new(96, 7, 128, 198))))));


        let opt = &rrs.get_additional()[0];
        assert_eq!(opt.get_name(), DomainName::new_from_str(""));
        assert_eq!(opt.get_rtype(), Rrtype::OPT);
        assert_eq!(opt.get_rclass(), Rclass::UNKNOWN(512));
        println!("{:?}", opt);
    }
}
#[tokio::test]
async fn query_a_type_with_rrsig_edns() {
    let response = query_response_edns("example.com",
                                       "UDP", "A", Some(1024), 0,
                                       true, Some(vec![OptionCode::NSID])).await;

    if let Ok(rrs) = response {
        println!("{}", rrs);
        assert_eq!(rrs.get_answer().len(), 7);
        let answers = rrs.get_answer();
        let data = vec![(&answers[0]).get_rdata(), (&answers[1]).get_rdata(),
                        (&answers[2]).get_rdata(), (&answers[3]).get_rdata(),
                        (&answers[4]).get_rdata(), (&answers[5]).get_rdata()];
        let rrsig = &answers[6];

        assert!(data.contains(&Rdata::A(ARdata::new_from_addr(IpAddr::V4(Ipv4Addr::new(23, 192, 228, 80))))));
        assert!(data.contains(&Rdata::A(ARdata::new_from_addr(IpAddr::V4(Ipv4Addr::new(23, 192, 228, 84))))));
        assert!(data.contains(&Rdata::A(ARdata::new_from_addr(IpAddr::V4(Ipv4Addr::new(23, 215, 0, 136))))));
        assert!(data.contains(&Rdata::A(ARdata::new_from_addr(IpAddr::V4(Ipv4Addr::new(23, 215, 0, 138))))));
        assert!(data.contains(&Rdata::A(ARdata::new_from_addr(IpAddr::V4(Ipv4Addr::new(96, 7, 128, 175))))));
        assert!(data.contains(&Rdata::A(ARdata::new_from_addr(IpAddr::V4(Ipv4Addr::new(96, 7, 128, 198))))));


        if let Rdata::RRSIG(sig) = rrsig.get_rdata() {
            assert_eq!(sig.get_type_covered(), Rrtype::A);
        } else {
            panic!("No RRSIG");

        }
        assert_eq!(answers[0].get_ttl(), rrsig.get_ttl());
        let opt = &rrs.get_additional()[0];
        assert_eq!(opt.get_name(), DomainName::new_from_str(""));
        assert_eq!(opt.get_rtype(), Rrtype::OPT);
        assert_eq!(opt.get_rclass(), Rclass::UNKNOWN(512));
        if let Rdata::OPT(rdata) = opt.get_rdata() {
            println!("{:?}", rdata);
            let rdata = rdata.clone();
            let option = &rdata.get_option()[0];
            if let OptionData::NSID(c) = option.get_opt_data() {
                println!("{}", c);
                // because the first query option is 8.8.8.8, it redirects to google public dns.
                assert!(c.starts_with("gpdns"))
            }
            //let (_,_,c) = &rdata.get_option()[0];
            //println!("{}", std::str::from_utf8(c).unwrap(),);
            // assert_eq!(std::str::from_utf8(c).unwrap(), "gpdns-scl")
        }
    }
}
#[ignore]
#[tokio::test]
async fn query_from_root() {
    const ROOTSV1: [u8; 4] = [192,58,128,30];

    let mut ip2req = ROOTSV1.into();
    let response = query_from_ip_with_edns("example.com",
                                           "UDP", "A", Some(1024), 0, false,
                                           None, ip2req).await;
    let mut response = match response {
        Ok(rrs) => rrs,
        Err(e) => panic!("{:?}", e),
    };

    println!("{}", response);
    let xd = response.to_bytes();
    let aa = DnsMessage::from_bytes(&xd).unwrap();
    assert_eq!(aa, response);

    let additional_rrs = response.get_additional();
}