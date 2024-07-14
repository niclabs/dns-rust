use std::{net::IpAddr, str::FromStr};
use dns_rust::{async_resolver::{config::ResolverConfig, AsyncResolver}, client::client_error::ClientError, domain_name::DomainName, message::{rclass::Rclass, rdata::Rdata, resource_record::{ResourceRecord, ToBytes}, rrtype::Rrtype, DnsMessage}};

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

#[tokio::test]
async fn query_a_type_edns() {
    let response = query_response_edns("example.com", "UDP", "A", Some(1024), 0, 0, Some(vec![3])).await;

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
        println!("{:?}", opt);
    } 
}