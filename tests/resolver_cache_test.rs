mod resolver_cache_test {
    use dns_rust::{
        async_resolver::{
            AsyncResolver, config::ResolverConfig, resolver_error::ResolverError,
            server_info::ServerInfo,
            lookup_response::LookupResponse
        },
        client::{
            client_connection::ClientConnection, client_error::ClientError,
            tcp_connection::ClientTCPConnection, udp_connection::ClientUDPConnection
        },
        dns_cache::CacheKey,
        domain_name::DomainName,
        message::{
            rclass::Rclass, rcode::Rcode, rdata::{a_rdata::ARdata, soa_rdata::SoaRdata, Rdata},
            resource_record::ResourceRecord, rrtype::Rrtype, DnsMessage
        },
    };
    use std::net::{IpAddr, Ipv4Addr};
    use std::str::FromStr;
    use std::time::Duration;
    use std::vec;
    use std::sync::Arc;
    use std::num::NonZeroUsize;
    use tokio::io;

    static TIMEOUT: u64 = 45;

    /// Test inner lookup cache
    #[tokio::test]
    async fn inner_lookup_cache_available() {
        let resolver = AsyncResolver::new(ResolverConfig::default());
        resolver
            .cache
            .lock()
            .unwrap()
            .set_max_size(NonZeroUsize::new(1).unwrap());

        let domain_name = DomainName::new_from_string("example.com".to_string());
        let a_rdata = ARdata::new_from_addr(IpAddr::from_str("93.184.216.34").unwrap());
        let a_rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(a_rdata);
        resolver.cache.lock().unwrap().add_answer(
            domain_name,
            resource_record,
            Some(Rrtype::A),
            Rclass::IN,
            None,
        );

        let domain_name = DomainName::new_from_string("example.com".to_string());
        let response = resolver
            .inner_lookup(domain_name, Rrtype::A, Rclass::IN)
            .await;

        if let Ok(msg) = response {
            assert_eq!(msg.to_dns_msg().get_header().get_aa(), false);
        } else {
            panic!("No response from cache");
        }
    }

    /// Test inner lookup without cache
    #[tokio::test]
    async fn inner_lookup_with_no_cache() {
        let mut config = ResolverConfig::default();
        config.set_cache_enabled(false);

        let resolver = AsyncResolver::new(config);
        {
            let mut cache = resolver.cache.lock().unwrap();
            cache.set_max_size(NonZeroUsize::new(1).unwrap());

            let domain_name = DomainName::new_from_string("example.com".to_string());
            let a_rdata = ARdata::new_from_addr(IpAddr::from_str("93.184.216.34").unwrap());
            let a_rdata = Rdata::A(a_rdata);
            let resource_record = ResourceRecord::new(a_rdata);
            cache.add_answer(
                domain_name,
                resource_record,
                Some(Rrtype::A),
                Rclass::IN,
                None,
            );
        }

        let domain_name = DomainName::new_from_string("example.com".to_string());
        let response = resolver
            .inner_lookup(domain_name, Rrtype::A, Rclass::IN)
            .await;

        if let Ok(msg) = response {
            assert_eq!(msg.to_dns_msg().get_header().get_aa(), false);
        } else {
            panic!("No response from nameserver");
        }
    }

    /// Test cache data
    #[tokio::test]
    async fn cache_data() {
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        resolver
            .cache
            .lock()
            .unwrap()
            .set_max_size(NonZeroUsize::new(1).unwrap());
        assert_eq!(resolver.cache.lock().unwrap().is_empty(), true);

        let _response = resolver.lookup("example.com", "UDP", "A", "IN").await;
        assert_eq!(
            resolver.cache.lock().unwrap().is_cached(CacheKey::Primary(
                Rrtype::A,
                Rclass::IN,
                DomainName::new_from_str("example.com")
            )),
            true
        );
        // TODO: Test special cases from RFC
    }

    #[test]
    fn not_store_data_in_cache_if_truncated() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        resolver
            .cache
            .lock()
            .unwrap()
            .set_max_size(NonZeroUsize::new(10).unwrap());

        let domain_name = DomainName::new_from_string("example.com".to_string());

        // Create truncated dns response
        let mut dns_response =
            DnsMessage::new_query_message(domain_name, Rrtype::A, Rclass::IN, 0, false, 1);
        let mut truncated_header = dns_response.get_header();
        truncated_header.set_tc(true);
        dns_response.set_header(truncated_header);

        resolver.store_data_cache(dns_response);

        assert_eq!(
            resolver
                .cache
                .lock()
                .unwrap()
                .get_cache_answer()
                .get_cache()
                .len(),
            0
        );
    }

    #[test]
    fn not_store_cero_ttl_data_in_cache() {
        let resolver = AsyncResolver::new(ResolverConfig::default());
        resolver
            .cache
            .lock()
            .unwrap()
            .set_max_size(NonZeroUsize::new(10).unwrap());

        let domain_name = DomainName::new_from_string("example.com".to_string());

        // Create dns response with ttl = 0
        let mut dns_response =
            DnsMessage::new_query_message(domain_name, Rrtype::A, Rclass::IN, 0, false, 1);
        // let mut truncated_header = dns_response.get_header();
        // truncated_header.set_tc(false);
        // dns_response.set_header(truncated_header);
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let a_rdata = ARdata::new_from_addr(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);

        // Cero ttl
        let mut rr_cero_ttl = ResourceRecord::new(rdata.clone());
        rr_cero_ttl.set_ttl(0);
        answer.push(rr_cero_ttl);

        // Positive ttl
        let mut rr_ttl_1 = ResourceRecord::new(rdata.clone());
        rr_ttl_1.set_ttl(1);
        answer.push(rr_ttl_1);

        let mut rr_ttl_2 = ResourceRecord::new(rdata);
        rr_ttl_2.set_ttl(2);
        answer.push(rr_ttl_2);

        dns_response.set_answer(answer);
        assert_eq!(dns_response.get_answer().len(), 3);
        assert_eq!(
            resolver
                .cache
                .lock()
                .unwrap()
                .get_cache_answer()
                .get_cache()
                .len(),
            0
        );

        resolver.store_data_cache(dns_response);
        assert_eq!(
            resolver
                .cache
                .lock()
                .unwrap()
                .get_cache_answer()
                .get_cache()
                .len(),
            2
        );
    }

}