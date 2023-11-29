use std::{net::IpAddr, str::FromStr};

use dns_rust::{async_resolver::{config::ResolverConfig, AsyncResolver, resolver_error::ResolverError}, message::{resource_record::ResourceRecord, rdata::Rdata}};



// TODO: Change params type to intoDomainName
async fn query_response(domain_name: &str, qtype: &str) -> Result<Vec<ResourceRecord>, ResolverError>{

    let config = ResolverConfig::default();
    let mut resolver = AsyncResolver::new(config);

    let response = resolver.lookup(domain_name, "UDP", qtype).await;

    response
}

/// 6.2.1 Query test Qtype = A
#[tokio::test]
async fn query_a_type() {
    let response = query_response("example.com", "A").await;

    if let Ok(rrs) = response {
        assert_eq!(rrs.iter().count(), 1);
        let rdata = rrs[0].get_rdata();
        if let Rdata::A(ip) = rdata {
            assert_eq!(ip.get_address(), IpAddr::from_str("93.184.216.34").unwrap());
        } else {
            panic!("Error parsing response");
        }
    } else {
        panic!("No response")
    }
}

/// 6.2.2 Query normal Qtype = *
#[tokio::test]
async fn query_all_type() {
    let response = query_response("example.com", "ANY").await;
    if let Ok(rrs) = response {
        assert_eq!(rrs.iter().count(), 2);
    } else {
        panic!("No response")
    }
}

// TODO: 6.2.3 Query normal Qtype = MX

// TODO: 6.2.4 Query normal Qtype = NS

// TODO: 6.2.5 Dominio mal escrito Qtype = A


