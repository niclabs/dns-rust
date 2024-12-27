#[cfg(test)]
mod client_test {
    use std::{net::{IpAddr, Ipv4Addr}, str::FromStr, time::Duration};
    use dns_rust::{
        async_resolver::{
                config::ResolverConfig, resolver_error::ResolverError, AsyncResolver, server_info::ServerInfo
            }, client::{
            client_connection::ClientConnection, client_error::ClientError, tcp_connection::ClientTCPConnection, udp_connection::ClientUDPConnection, Client}, domain_name::DomainName, 
            message::{resource_record::ResourceRecord, rdata::{a_rdata::ARdata, Rdata, mx_rdata::MxRdata, ns_rdata::NsRdata}}};




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
            assert_eq!(RR.get_ttl(), 3600);
            assert_eq!(RR.get_rdlength(), 4);
            assert_eq!(RR.get_rdata(), Rdata::A(ARdata::new_from_addr(IpAddr::V4(Ipv4Addr::new(93, 184, 215, 14)))));

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
        assert_eq!(RR.get_rdata(), Rdata::A(ARdata::new_from_addr(IpAddr::V4(Ipv4Addr::new(93, 184, 215, 14)))));

        // authority
        assert!(resp.get_authority().is_empty());

        // additional
        assert!(resp.get_additional().is_empty());
        } else {
            panic!();
        }
    }


    // RFC 1034 6.2.2
    // Testing with QTYPE=ANY on an authoritative server for example.com
    #[tokio::test]
    async fn QTYPE_ANY_AUTH_ANSWER() {
        let addr = IpAddr::V4(Ipv4Addr::new(199, 43, 135, 53)); // a.iana-servers.net ip
        let conn = ClientUDPConnection::new_default(addr, Duration::from_secs(10));
        let mut client = Client::new(conn);

        let response = client.query(
            DomainName::new_from_string("example.com".to_string()), 
            "ANY", 
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
            assert_eq!(RR.get_ttl(), 3600);
            assert_eq!(RR.get_rdlength(), 4);
            assert_eq!(RR.get_rdata(), Rdata::A(ARdata::new_from_addr(IpAddr::V4(Ipv4Addr::new(93, 184, 215, 14)))));

            // authority
            assert!(resp.get_authority().is_empty());

            // additional
            assert!(resp.get_additional().is_empty());
        } else {
            panic!("response error");
        }
    }

    // Testing with QTYPE=ANY on a non authoritative server for example.com
    #[tokio::test]
    async fn QTYPE_ANY() {
        let addr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)); 
        let conn = ClientUDPConnection::new_default(addr, Duration::from_secs(10));
        let mut client = Client::new(conn);

        let response = client.query(
            DomainName::new_from_string("example.com".to_string()), 
            "ANY", 
            "IN"
        ).await;


        if let Ok(resp) = response {
            // header
            assert!(resp.get_header().get_qr());
            assert!(!resp.get_header().get_aa());

            // question
            assert_eq!(resp.get_question(), client.get_dns_query().get_question());

            // answer
            assert!(resp.get_answer().is_empty());

            // authority
            assert!(resp.get_authority().is_empty());

            // additional
            assert!(resp.get_additional().is_empty());
        } else {
            panic!("response error");
        }
    }

    // RFC 1034 6.2.3 
    // Testing with QTYPE=MX
    #[tokio::test]
    async fn QTYPE_MX() {
        let addr = IpAddr::V4(Ipv4Addr::new(199, 43, 135, 53)); // a.iana-servers.net ip
        let conn = ClientUDPConnection::new_default(addr, Duration::from_secs(10));
        let mut client = Client::new(conn);

        let response = client.query(
            DomainName::new_from_string("example.com".to_string()), 
            "MX", 
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
            assert_eq!(RR.get_rtype(), "MX".into());
            assert_eq!(RR.get_rclass(), "IN".into());
            assert_eq!(RR.get_ttl(), 86400);
            assert_eq!(RR.get_rdlength(), 3);
            assert_eq!(RR.get_rdata(), Rdata::MX(MxRdata::new()));

            // authority
            assert!(resp.get_authority().is_empty());

            // additional
            assert!(resp.get_additional().is_empty());
        } else {
            panic!("response error");
        }
    }

    // RFC 1034 6.2.4 
    // Testing with QTYPE=NS 
    #[tokio::test]
    async fn QTYPE_NS() {
        let addr = IpAddr::V4(Ipv4Addr::new(199, 43, 135, 53)); // a.iana-servers.net ip
        let conn = ClientUDPConnection::new_default(addr, Duration::from_secs(10));
        let mut client = Client::new(conn);

        let response = client.query(
            DomainName::new_from_string("example.com".to_string()), 
            "NS", 
            "IN"
        ).await;


        if let Ok(resp) = response {
            // header
            assert!(resp.get_header().get_qr());
            assert!(resp.get_header().get_aa());

            // question
            assert_eq!(resp.get_question(), client.get_dns_query().get_question());

            // answer 1
            let RR1 = &resp.get_answer()[0];
            assert_eq!(RR1.get_name(), DomainName::new_from_string("example.com".to_string()));
            assert_eq!(RR1.get_rtype(), "NS".into());
            assert_eq!(RR1.get_rclass(), "IN".into());
            assert_eq!(RR1.get_ttl(), 86400);
            // FIX
            //assert_eq!(RR1.get_rdlength(), );            

            let mut nsdata = NsRdata::new();
            nsdata.set_nsdname(DomainName::new_from_string("a.iana-servers.net".to_string()));
            let data0 = Rdata::NS(nsdata);
            assert_eq!(RR1.get_rdata(), data0);

            // answer 2
            let RR2 = &resp.get_answer()[1];
            assert_eq!(RR2.get_name(), DomainName::new_from_string("example.com".to_string()));
            assert_eq!(RR2.get_rtype(), "NS".into());
            assert_eq!(RR2.get_rclass(), "IN".into());
            assert_eq!(RR2.get_ttl(), 86400);
            // FIX
            //assert_eq!(RR2.get_rdlength(), );            

            nsdata = NsRdata::new();
            nsdata.set_nsdname(DomainName::new_from_string("b.iana-servers.net".to_string()));
            let data1 = Rdata::NS(nsdata);
            assert_eq!(RR2.get_rdata(), data1);

            // authority
            assert!(resp.get_authority().is_empty());

            // additional
            assert!(resp.get_additional().is_empty());

        } else {
            panic!("response error");
        }
    }


    // RFC 1034 6.2.5 
    // testing QTYPE=A with a mistyped host name (the host name must not exist)
    #[tokio::test]
    async fn QTYPE_A_MISTYPED() {
        let addr = IpAddr::V4(Ipv4Addr::new(199, 43, 135, 53)); // a.iana-servers.net ip
        let conn = ClientUDPConnection::new_default(addr, Duration::from_secs(10));
        let mut client = Client::new(conn);

        let response = client.query(
            DomainName::new_from_string("notexists.example.com".to_string()), // this domain doesnt exists
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
            assert!(resp.get_answer().is_empty());

            // authority
            let authority = &resp.get_authority()[0];
            assert_eq!(authority.get_name(), DomainName::new_from_string("example.com".to_string()));
            assert_eq!(authority.get_rtype(), "SOA".into());
            assert_eq!(authority.get_rclass(), "IN".into());
            assert_eq!(authority.get_ttl(), 3600);
            // TODO
            // example.com  IN  SOA  3600  ns.icann.org noc.dns.icann.org 2024081477 7200 3600 1209600 3600

            // additional
            assert!(resp.get_additional().is_empty());
        } else {
            panic!("response error");
        }
    }

    // RFC 1034 6.2.6 

    // RFC 1034 6.2.7 

    // RFC 1034 6.2.8


}