#[cfg(test)]
mod client_test {
    use std::{net::{IpAddr, Ipv4Addr, Ipv6Addr}, time::Duration};
    use dns_rust::{
        client::{
            client_connection::ClientConnection,  udp_connection::ClientUDPConnection, Client}, domain_name::DomainName,
            message::{rdata::{a_rdata::ARdata, Rdata, mx_rdata::MxRdata, ns_rdata::NsRdata}}};




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
            
            // this is for the rdata of the authority
            let rdata = authority.get_rdata();
            if let Rdata::SOA(data) = rdata{
                assert_eq!(data.get_mname(), DomainName::new_from_string("ns.icann.org".to_string()));
                assert_eq!(data.get_rname(), DomainName::new_from_string("noc.dns.icann.org".to_string()));
                assert_eq!(data.get_refresh(), 7200);
                assert_eq!(data.get_retry(), 3600);
                assert_eq!(data.get_expire(), 1209600);
                assert_eq!(data.get_minimum(), 3600);
            } else{
                panic!("wrong rdata");
            }

            // additional
            assert!(resp.get_additional().is_empty());
        } else {
            panic!("response error");
        }
    }

    // RFC 1034 6.2.6 
    #[tokio::test]
    async fn QTYPE_A_REFERRAL() {
        let addr = IpAddr::V4(Ipv4Addr::new(198, 41, 0, 4)); // a.root-servers.net ip
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

            // question
            assert_eq!(resp.get_question(), client.get_dns_query().get_question());

            // answer
            assert!(resp.get_answer().is_empty());

            // authority
            let authority = &resp.get_authority();
            for auth in authority{
                assert_eq!(auth.get_name(), DomainName::new_from_string("com".to_string()));
                assert_eq!(auth.get_rtype(), "NS".into());
                assert_eq!(auth.get_rclass(), "IN".into());
                assert_eq!(auth.get_ttl(), 172800);
                
                // Ensure that all RDATAs are of type NS
                if let Rdata::NS(_ns_name) = auth.get_rdata() {
                } else {
                    panic!("NS rdata was expected");
                }

            }
            // Verify the first rdata
            let rdata = &authority[0].get_rdata();
            if let Rdata::NS(data) = rdata{
                assert_eq!(data.get_nsdname(), DomainName::new_from_string("l.gtld-servers.net".to_string()));
            } else{
                panic!("wrong rdata");
            }

            // additional
            let additional = &resp.get_additional();
            let mut i = 0;
            for addi in additional {
                assert_eq!(addi.get_rclass(), "IN".into());
                assert_eq!(addi.get_ttl(), 172800);

                // test for the rdata type
                if i==0 {
                    if let Rdata::A(_data) = addi.get_rdata() {
                        i = 1;
                    } else {
                        panic!("wrong rdata type");
                    }
                } else {
                    if let Rdata::AAAA(_data) = addi.get_rdata() {
                        i = 0;
                    } else {
                        panic!("wrong rdata type");
                    }
                }
            } 
            // Verify the first rdata
            let rdata1 = &additional[0].get_rdata();
            if let Rdata::A(data) = rdata1{
                assert_eq!(data.get_address(), IpAddr::V4(Ipv4Addr::new(192, 41, 162, 30)));
            } else{
                panic!("wrong rdata type");
            }
            // Verify the second rdata
            let rdata2 = &additional[1].get_rdata();
            if let Rdata::AAAA(data) = rdata2{
                assert_eq!(data.get_address(), IpAddr::V6(Ipv6Addr::new(0x2001, 0x500, 0xd937, 0, 0, 0, 0, 0x30)));
            } else{
                panic!("wrong rdata type");
            }

        } else {
            panic!("response error");
        }
    }

    // RFC 1034 6.2.7 


    // RFC 1034 6.2.8

    // Instructions for running test on a Debian-Based Linux System

    // Step 1: Install BIND9
    // ----------------------------------------
    // BIND9 is a DNS server that allows the configuration of custom DNS zones for testing purposes.
    // 
    // sudo apt install bind9

    // Step 2: Configure the Zone File
    // ----------------------------------------
    // 1. Open a new zone file for `uchile.cl` using a text editor:
    // 
    // sudo nano /etc/bind/db.uchile.cl
    //
    // 2. Copy the following content into the file:
    // 
    // $TTL 3600
    // @   IN  SOA VENERA. Action\.domains (
    //         20     ; SERIAL
    //         7200   ; REFRESH
    //         600    ; RETRY
    //         3600000; EXPIRE
    //         60 )   ; MINIMUM
    //     NS      A.ISI.EDU.
    //     NS      VENERA
    //     NS      VAXA
    //     MX      10      VENERA
    //     MX      20      VAXA
    // 
    // dcc    IN  A   192.80.24.11
    // 
    // test   IN  CNAME no-test.com.
    // 
    // delegation IN NS ns.delegation.uchile.cl.
    // delegation IN NS ns2.test.com.
    // 
    // ; Glue records
    // ns.delegation IN A 127.0.0.1
    // VENERA        IN A 192.168.99.12
    // VAXA          IN A 192.168.99.13

    // Step 3: Configure the Named Configuration File
    // ----------------------------------------
    // 1. Edit the `named.conf.local` file to include the zone configuration:
    // 
    // sudo nano /etc/bind/named.conf.local
    // 
    // 2. Add the following configuration:
    // 
    // zone "uchile.cl" {
    //    type master;
    //    file "/etc/bind/db.uchile.cl";
    // };

    // Step 4: Validate the Zone Configuration
    // ----------------------------------------
    // 1. Validate your zone file with the following command:
    // 
    // named-checkzone uchile.cl /etc/bind/db.uchile.cl
    // 
    // 2. If the output states "OK," the configuration is correct.

    // Step 5: Restart the DNS Server
    // ----------------------------------------
    // Restart the BIND9 service to apply the changes:
    // 
    // sudo systemctl restart bind9

    #[tokio::test]
    async fn QTYPE_CNAME(){
        let addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)); // localhost
        let conn = ClientUDPConnection::new_default(addr, Duration::from_secs(10));
        let mut client = Client::new(conn);

        let response = client.query(
            DomainName::new_from_string("test.uchile.cl".to_string()),
            "CNAME", 
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
            assert_eq!(RR.get_name(), DomainName::new_from_string("test.uchile.cl".to_string()));
            assert_eq!(RR.get_rtype(), "CNAME".into());
            assert_eq!(RR.get_rclass(), "IN".into());
            assert_eq!(RR.get_ttl(), 3600);
            assert_eq!(RR.get_rdlength(), 13);
            if let Rdata::CNAME(data) = RR.get_rdata() {
                assert_eq!(data.get_cname(), DomainName::new_from_string("no-test.com".to_string()))
            } else {
                panic!("wrong rdata type")
            }

            // authority

            // additional
            
        } else {
            panic!("response error");
        }
    }

    
    



}