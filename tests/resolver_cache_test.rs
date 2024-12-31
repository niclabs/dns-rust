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



}