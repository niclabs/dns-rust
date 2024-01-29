use std::{net::IpAddr, str::FromStr};

use dns_rust::{async_resolver::{config::ResolverConfig, AsyncResolver, resolver_error::ResolverError}, client::client_error::ClientError, dns_cache::cache_by_record_type::rr_stored_data, domain_name::DomainName, message::{resource_record::ResourceRecord, rdata::{a_rdata::ARdata, Rdata}}};
use dns_rust::message::type_rtype::Rtype;



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

/// 6.2.1 Query test Qtype = A with cache check
#[tokio::test]
#[ignore = "Lookup do not save the answer in the cache correctly"]
async fn query_a_type_check_cache() {
    //config and resolver
    let config = ResolverConfig::default();
    let mut resolver = AsyncResolver::new(config);

    let response = resolver.lookup("example.com", "UDP", "A", "IN").await;

    //check if the rrs is in the cache in the resolver
    let dnscache = resolver.get_cache();
    println!("DnsCache is : {:?}", dnscache);
    //FIXME: THERE IS NOTHING INSIDE THE CACHE, lookup do not save the answer in the cache correctly
    assert!(dnscache.is_cached(DomainName::new_from_string("example.com".to_string()), Rtype::A));
    
    //check if the rrs in the cache is the same with the response
    if let Some(cache_domain_name) = dnscache.get_cache().get(Rtype::A) {
        assert_eq!(cache_domain_name.get_domain_names_data().len(), 1);
        if let Some(rr_stored_vec) = cache_domain_name.get(&DomainName::new_from_string("example.com".to_string())) {
            if let Some(rr_stored) = rr_stored_vec.get(0) {
                if let Ok(response_rr_stored_vec) = response {
                    if let Some(response_rr_stored) = response_rr_stored_vec.get(0) {
                        assert_eq!(&rr_stored.get_resource_record(), response_rr_stored);
                    }
                }   
            }
        }
    }
    else {
        println!("No cache for Type A");
    }

}

/// Test offline
#[tokio::test]
#[ignore = "To pass this test you must be offline"]
async fn query_a_type_check_no_conecction() {
    //config and resolver
    let config = ResolverConfig::default();
    let mut resolver = AsyncResolver::new(config);

    let response = resolver.lookup("example.com", "UDP", "A", "IN").await;
    println!("{:?}", response);
    if let Err(error) = response {
        println!("the error is : \n {:?}", error);
        assert!(true);
    } else {
        panic!("The response should be an error");
    }
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
#[ignore = "Error is not the same, becuase the rcode is 2 instead of 3"]
async fn query_a_type_check_cache_negative_answer() {
    //config and resolver
    let config = ResolverConfig::default();
    let mut resolver = AsyncResolver::new(config);

    //the domian examplle.com do not exist
    if let Err(response) = resolver.lookup("aksjdsadkjaka.com", "UDP", "A", "IN").await {
        println!("the error is : \n{:?}", response.to_string());
        //the rcode shoud be 3, so the parse shoud response with that error
        //FIXME: for some reason, the rcode is 2 instead of 3, so the error is differennt
        assert_eq!(response.to_string(), "parse response error: The domain name referenced in the query does not exist.");
    }
    else {
        panic!("The response should be an error");
    }
}

/// No record test
#[tokio::test]
async fn no_resource_available() {
    let response =  query_response("example.com", "UDP", "CNAME").await;
    println!("{:?}", response);
    assert!(response.is_err());
}

        
        
