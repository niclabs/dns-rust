use std::{net::IpAddr, str::FromStr};
use std::time::Duration;
use dns_rust::{async_resolver::{config::ResolverConfig, AsyncResolver}, client::client_error::ClientError, domain_name::DomainName, message::{rclass::Rclass, rdata::Rdata, resource_record::{ResourceRecord, ToBytes}, rrtype::Rrtype, DnsMessage}};
use dns_rust::async_resolver::server_info::ServerInfo;

async fn query_from_ip_with_edns(domain_name: &str,
                                 protocol: &str,
                                 qtype: &str,
                                 max_payload: Option<u16>,
                                 version: u8,
                                 do_bit: bool,
                                 option: Option<Vec<u16>>,
                                 ip_addr: IpAddr) -> Result<DnsMessage, ClientError> {

    let mut config = ResolverConfig::default();
    config.add_edns0(max_payload, version, do_bit, option);


    config.set_name_servers(vec![ServerInfo::new_from_addr(ip_addr, Duration::from_secs(3))]);
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
    option: Option<Vec<u16>>) -> Result<DnsMessage, ClientError> {

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
    let response = query_response_edns("example.com", "UDP", "A", Some(1024), 0, false, Some(vec![3])).await;

    if let Ok(rrs) = response {
        println!("{}", rrs);
        assert_eq!(rrs.get_answer().len(), 1);
        let rdata = rrs.get_answer()[0].get_rdata();
        if let Rdata::A(ip) = rdata {
            assert_eq!(ip.get_address(), IpAddr::from_str("93.184.215.14").unwrap());
        } else {
            panic!("No ip address");
        }
        let opt = &rrs.get_additional()[0];
        assert_eq!(opt.get_name(), DomainName::new_from_str(""));
        assert_eq!(opt.get_rtype(), Rrtype::OPT);
        assert_eq!(opt.get_rclass(), Rclass::UNKNOWN(512));
        println!("{:?}", opt);
    } 
}

#[tokio::test]
async fn query_a_type_with_rrsig_edns() {
    let response = query_response_edns("example.com", "UDP", "A", Some(1024), 0, true, Some(vec![3])).await;

    if let Ok(rrs) = response {
        println!("{}", rrs);
        assert_eq!(rrs.get_answer().len(), 2);
        let answers = rrs.get_answer();
        let answer = &answers[0];
        let rrsig = &answers[1];
        if let Rdata::A(ip) = answer.get_rdata() {
            assert_eq!(ip.get_address(), IpAddr::from_str("93.184.215.14").unwrap());
        } else {
            panic!("No ip address");
        }
        if let Rdata::RRSIG(sig) = rrsig.get_rdata() {
            assert_eq!(sig.get_type_covered(), Rrtype::A);
        } else {
            panic!("No RRSIG");
            
        }
        let opt = &rrs.get_additional()[0];
        assert_eq!(opt.get_name(), DomainName::new_from_str(""));
        assert_eq!(opt.get_rtype(), Rrtype::OPT);
        assert_eq!(opt.get_rclass(), Rclass::UNKNOWN(512));
        println!("{:?}", opt);
    }
}

#[tokio::test]
async fn query_from_root() {
    const ROOTSV1: [u8; 4] = [192,58,128,30];

    let mut ip2req = ROOTSV1.into();
    let response = query_from_ip_with_edns("example.com",
                                           "UDP", "A", Some(1024), 0, true,
                                           Some(vec![3]), ip2req).await;
    let response = match response {
        Ok(rrs) => rrs,
        Err(e) => panic!("{:?}", e),
    };

    println!("{}", response);

    let additional_rrs = response.get_additional();

    let a_rrs: Vec<_> = additional_rrs.iter()
        .filter(|arrs|
            if let Rdata::A(_) = arrs.get_rdata() {true}
            else {false}).collect();

    if let Rdata::A(rdata) = a_rrs[4].get_rdata() {
        ip2req = rdata.get_address();
    }
}

#[tokio::test]
async fn query_a_type_with_rrsig_edns() {
    let response = query_response_edns("example.com", "UDP", "A", Some(1024), 0, true, Some(vec![3])).await;

    if let Ok(rrs) = response {
        println!("{}", rrs);
        assert_eq!(rrs.get_answer().len(), 2);
        let answers = rrs.get_answer();
        let answer = &answers[0];
        let rrsig = &answers[1];
        if let Rdata::A(ip) = answer.get_rdata() {
            assert_eq!(ip.get_address(), IpAddr::from_str("93.184.215.14").unwrap());
        } else {
            panic!("No ip address");
        }
        if let Rdata::RRSIG(sig) = rrsig.get_rdata() {
            assert_eq!(sig.get_type_covered(), Rrtype::A);
        } else {
            panic!("No RRSIG");
            
        }
        let opt = &rrs.get_additional()[0];
        assert_eq!(opt.get_name(), DomainName::new_from_str(""));
        assert_eq!(opt.get_rtype(), Rrtype::OPT);
        assert_eq!(opt.get_rclass(), Rclass::UNKNOWN(512));
        println!("{:?}", opt);
    } 
}