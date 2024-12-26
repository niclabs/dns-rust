use std::{net::{IpAddr, Ipv4Addr}, str::FromStr, time::Duration};
use dns_rust::{
    async_resolver::{
            config::ResolverConfig, resolver_error::ResolverError, AsyncResolver, server_info::ServerInfo
        }, client::{
        client_connection::ClientConnection, client_error::ClientError, tcp_connection::ClientTCPConnection, udp_connection::ClientUDPConnection, Client}, domain_name::DomainName, 
        message::{resource_record::ResourceRecord, rdata::{a_rdata::ARdata, Rdata}}};




// RFC 1034 6.2.1 
// Testing with QTYPE=A on an authoritative server for example.com
#[tokio::test]
async fn QTYPE_A_TEST_AUTH_ANSWER() {
    let addr = IpAddr::V4(Ipv4Addr::new(199, 43, 135, 53)); // a.iana-servers.net ip
    let conn = ClientUDPConnection::new_default(addr, Duration::from_secs(10));
    let mut client = Client::new(conn);

    let response = client.query(
        DomainName::new_from_string("example.com".to_string()), 
        "A", 
        "IN"
    ).await;
    if let Ok(resp) = response {
        // header
        assert!(resp.get_header().get_qr());
        assert!(resp.get_header().get_aa());

        // question
        assert_eq!(resp.get_question(), client.get_dns_query().get_question());

        // answer
        let RR = &resp.get_answer()[0];
        assert_eq!(RR.get_name(), DomainName::new_from_string("example.com".to_string()));
        assert_eq!(RR.get_rtype(), "A".into());
        assert_eq!(RR.get_rclass(), "IN".into());
        assert_eq!(RR.get_rdlength(), 4);
        assert_eq!(RR.get_rdata(), Rdata::A(ARdata::new_from_addr(IpAddr::V4(Ipv4Addr::new(93, 184, 216, 34)))));

        // authority
        assert!(resp.get_authority().is_empty());

        // additional
        assert!(resp.get_additional().is_empty());
    } else {
        panic!("response error");
    }
}


// Testing with QTYPE=A on a non authoritative server for example.com
#[tokio::test]
async fn QTYPE_A_TEST() {
    let addr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
    let conn = ClientUDPConnection::new_default(addr, Duration::from_secs(10));
   let mut client = Client::new(conn);

    let response = client.query(
        DomainName::new_from_string("example.com".to_string()), 
        "A", 
        "IN"
    ).await;
    if let Ok(resp) = response {
       // header
       assert!(resp.get_header().get_qr());
       assert!(!resp.get_header().get_aa());

       // question
       assert_eq!(resp.get_question(), client.get_dns_query().get_question());

       // answer
       let RR = &resp.get_answer()[0];
       assert_eq!(RR.get_name(), DomainName::new_from_string("example.com".to_string()));
       assert_eq!(RR.get_rtype(), "A".into());
       assert_eq!(RR.get_rclass(), "IN".into());
       assert_eq!(RR.get_rdlength(), 4);
       assert_eq!(RR.get_rdata(), Rdata::A(ARdata::new_from_addr(IpAddr::V4(Ipv4Addr::new(93, 184, 216, 34)))));

       // authority
       assert!(resp.get_authority().is_empty());

       // additional
       assert!(resp.get_additional().is_empty());
    } else {
        panic!();
    }
}

