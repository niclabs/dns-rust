use std::{net::IpAddr, option, str::FromStr};
use dns_rust::{async_resolver::{config::ResolverConfig, AsyncResolver}, client::client_error::ClientError, domain_name::DomainName, message::{rclass::Rclass, rdata::Rdata, resource_record::{ResourceRecord, ToBytes}, rrtype::Rrtype, DnsMessage}};



// TODO: Change params type to intoDomainName
async fn query_response(domain_name: &str, protocol: &str, qtype: &str) -> Result<Vec<ResourceRecord>, ClientError> {

    let config = ResolverConfig::default();
    let mut resolver = AsyncResolver::new(config);

    let response = resolver.lookup(
        domain_name,
        protocol,
        qtype,
        "IN").await;

    response.map(|lookup_response| lookup_response.to_vec_of_rr())
}

async fn query_response_edns(domain_name: &str, 
    protocol: &str, 
    qtype: &str, 
    max_payload: Option<u16>, 
    version: u16, 
    flags: u16, 
    option: Option<Vec<u16>>) -> Result<DnsMessage, ClientError> {

    let mut config = ResolverConfig::default();
    config.add_edns0(max_payload, version, flags, option);
    let mut resolver = AsyncResolver::new(config);

    let response = resolver.lookup(
        domain_name,
        protocol,
        qtype,
        "IN").await;

    response.map(|lookup_response| lookup_response.to_dns_msg())
}

/// 6.2.1 Query test Qtype = A
#[tokio::test]
async fn query_a_type() {
    let response = query_response("example.com", "UDP", "A").await;

    if let Ok(rrs) = response {
        assert_eq!(rrs.iter().count(), 1);
        let rdata = rrs[0].get_rdata();
        if let Rdata::A(ip) = rdata {
            assert_eq!(ip.get_address(), IpAddr::from_str("93.184.215.14").unwrap());
        } else {
            panic!("No ip address");
        }
    } 
}

/// 6.2.2 Query normal Qtype = *
#[tokio::test]
/// Ignored due to halting problem
#[ignore]
async fn query_any_type() {
    let udp_response = query_response("example.com", "UDP", "ANY").await;
    let tcp_response = query_response("example.com", "TCP", "ANY").await;
    assert!(udp_response.is_err());
    assert!(tcp_response.is_err());
}

/// 6.2.3 Query Qtype = MX
#[tokio::test]
async fn query_mx_type() {
    let response = query_response("example.com", "UDP", "MX").await;
    
    if let Ok(rrs) = response {
        assert_eq!(rrs.len(), 1);

        if let Rdata::MX(mxdata) = rrs[0].get_rdata() {
            assert_eq!(
                mxdata.get_exchange(),
                DomainName::new_from_str(""));

            assert_eq!(
                mxdata.get_preference(),
                0
            )
        } else { 
            panic!("Record is not MX type");
        }
    }
}


// 6.2.4 Query Qtype = NS
#[tokio::test]
async fn query_ns_type() {
    let response = query_response("example.com", "UDP", "NS").await;
    if let Ok(rrs) = response {
        assert_eq!(rrs.len(), 2);
        
        if let Rdata::NS(ns1) = rrs[0].get_rdata() {
            assert_eq!(
                ns1.get_nsdname(),
                DomainName::new_from_str("a.iana-servers.net"))
        } else { 
            panic!("First record is not NS");
        }
        
        if let Rdata::NS(ns) = rrs[1].get_rdata() {
            assert_eq!(
                ns.get_nsdname(),
                DomainName::new_from_str("b.iana-servers.net"))
        } else {
            panic!("Second record is not NS");
        }
    }
}

/// 6.2.5 Mistyped host name Qtype = A
#[tokio::test]
async fn mistyped_host_name() {
    let response = query_response("exampllee.com", "UDP", "A").await;
    assert!(response.is_err());
}

/// No record test
#[tokio::test]
async fn no_resource_available() {
    let response =  query_response("example.com", "UDP", "CNAME").await;
    println!("{:?}", response);
    assert!(response.is_err());
}

#[tokio::test]
async fn query_a_type_edns() {
    let response = query_response_edns("example.com", "UDP", "A", Some(1024), 0, 0, None).await;

    if let Ok(rrs) = response {
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
    } 
}



