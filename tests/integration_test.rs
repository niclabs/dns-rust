use std::{net::IpAddr, str::FromStr};

use dns_rust::{async_resolver::{config::ResolverConfig, AsyncResolver, resolver_error::ResolverError}, message::{resource_record::ResourceRecord, rdata::Rdata}, domain_name::DomainName};



// TODO: Change params type to intoDomainName
async fn query_response(domain_name: &str, protocol: &str, qtype: &str) -> Result<Vec<ResourceRecord>, ResolverError>{

    let config = ResolverConfig::default();
    let mut resolver = AsyncResolver::new(config);

    let response = resolver.lookup(
        domain_name,
        protocol,
        qtype,
        "IN").await;

    response
}

/// 6.2.1 Query test Qtype = A
#[tokio::test]
async fn query_a_type() {
    let response = query_response("example.com", "UDP", "A").await;

    if let Ok(rrs) = response {
        assert_eq!(rrs.iter().count(), 1);
        let rdata = rrs[0].get_rdata();
        if let Rdata::A(ip) = rdata {
            assert_eq!(ip.get_address(), IpAddr::from_str("93.184.216.34").unwrap());
        } else {
            panic!("No ip address");
        }
    } 
}

/// 6.2.2 Query normal Qtype = *
#[tokio::test]
async fn query_all_type() {
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



