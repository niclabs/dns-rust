pub mod config;
pub mod resolution;
pub mod lookup_response;
pub mod resolver_error;
pub mod server_info;
pub mod slist;
pub mod state_block;
pub mod server_state;
pub mod server_entry;

use self::lookup_response::LookupResponse;
use crate::async_resolver::resolver_error::ResolverError;
use crate::async_resolver::{config::ResolverConfig, resolution::Resolution};
use crate::client::client_connection::ConnectionProtocol;
use crate::client::client_error::ClientError;
use crate::domain_name::DomainName;
use crate::message::rclass::Rclass;
use crate::message::rcode::Rcode;
use crate::message::rdata::Rdata;
use crate::message::resource_record::ResourceRecord;
use crate::message::rrtype::Rrtype;
use crate::message::{self, DnsMessage};
use crate::resolver_cache::ResolverCache;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};

/// Asynchronous resolver for DNS queries.
///
/// This struct contains a cache and a configuration for the resolver.
/// The cache is used to store the responses of the queries and the
/// configuration is used to set the parameters of the resolver.
///
/// The `AsyncResolver` struct is used to send queries to a DNS server in
/// a asynchronous way. This means that the queries are sent and the
/// resolver continues with the execution of the program without waiting
/// for the response of the query.
///
/// Each query corresponds to a future that is going to be spawned using
/// `lookup_ip` method.
#[derive(Clone)]
pub struct AsyncResolver {
    /// Cache for the resolver
    cache: Arc<Mutex<ResolverCache>>,
    /// Configuration for the resolver.
    config: ResolverConfig,
}

impl AsyncResolver {
    /// Creates a new `AsyncResolver` with the given configuration.
    ///
    /// # Example
    /// ```
    /// use std::time::Duration;
    /// use dns_resolver::resolver::config::ResolverConfig;
    /// use dns_resolver::resolver::async_resolver::AsyncResolver;
    ///
    /// let config = ReolverConfig::default();
    /// let resolver = AsyncResolver::new(config.clone());
    /// assert_eq!(resolver.config, config);
    /// ```
    pub fn new(config: ResolverConfig) -> Self {
        let async_resolver = AsyncResolver {
            cache: Arc::new(Mutex::new(ResolverCache::new(None))),
            config: config,
        };
        async_resolver
    }

    pub fn run() {
        unimplemented!()
    }

    /// This method acts as an interface between the Client and the Resolver.
    ///
    /// It calls `inner_lookup(&self, domain_name: DomainName)` which will
    /// execute a look up of the given domain name asynchronously. The method
    /// retuns the corresponding `Result<Vec<IpAddr>, ClientError>` to the Client.
    /// The `Vec<IpAddr>` contains the IP addresses of the domain name.
    ///
    /// [RFC 1034]: https://datatracker.ietf.org/doc/html/rfc1034#section-5.2
    ///
    /// 5.2. Client-resolver interface
    ///
    /// 1. Host name to host address translation
    /// This function is often defined to mimic a previous HOSTS.TXT
    /// based function.  Given a character string, the caller wants
    /// one or more 32 bit IP addresses.  Under the DNS, it
    /// translates into a request for type A RRs.  Since the DNS does
    /// not preserve the order of RRs, this function may choose to
    /// sort the returned addresses or select the "best" address if
    /// the service returns only one choice to the client.  Note that
    /// a multiple address return is recommended, but a single
    /// address may be the only way to emulate prior HOSTS.TXT
    /// services.
    pub async fn lookup_ip(
        &mut self,
        domain_name: &str,
        rclass: &str
    ) -> Result<Vec<IpAddr>, ClientError> {
        let domain_name_struct = DomainName::new_from_string(domain_name.to_string());
        let transport_protocol=  self.config.get_protocol();
        let transport_protocol_struct = ConnectionProtocol::from(transport_protocol);
        self.config.set_protocol(transport_protocol_struct);

        let response = self
            .inner_lookup(domain_name_struct, Rrtype::A, rclass.into())
            .await;

        return self
            .check_error_from_msg(response)
            .and_then(|lookup_response| {
                let rrs_iter = lookup_response.to_vec_of_rr().into_iter();
                let ip_addresses: Result<Vec<IpAddr>, _> = rrs_iter
                    .map(|rr| AsyncResolver::from_rr_to_ip(rr))
                    .collect();
                return ip_addresses;
            });
    }

    /// Performs a DNS lookup of the given domain name, qtype and rclass.
    ///
    /// This method calls the `inner_lookup` method with the given domain name,
    /// qtype, rclass and the chosen transport protocol. It performs a DNS lookup
    /// asynchronously and returns the corresponding `Result<LookupResponse, ClientError>`.
    /// The `LookupResponse` contains the response of the query which can be translated
    /// to different formats.
    ///
    /// If the response has an error, the method returns the corresponding `ClientError`
    /// to the Client.
    ///
    /// [RFC 1034]: https://datatracker.ietf.org/doc/html/rfc1034#section-5.2
    ///
    /// 5.2 Client-resolver interface
    ///
    /// 3. General lookup function
    ///
    /// This function retrieves arbitrary information from the DNS,
    /// and has no counterpart in previous systems.  The caller
    /// supplies a QNAME, QTYPE, and RCLASS, and wants all of the
    /// matching RRs.  This function will often use the DNS format
    /// for all RR data instead of the local host's, and returns all
    /// RR content (e.g., TTL) instead of a processed form with local
    /// quoting conventions.
    ///
    /// # Examples
    /// ```
    /// let mut resolver = AsyncResolver::new(ResolverConfig::default());
    /// let domain_name = "example.com";
    /// let transport_protocol = "UDP";
    /// let rrtype = "NS";
    /// let response = resolver.lookup(domain_name, transport_protocol,rrtype).await.unwrap();
    /// ```
    pub async fn lookup(
        &mut self,
        domain_name: &str,
        transport_protocol: &str,
        rrtype: &str,
        rclass: &str,
    ) -> Result<LookupResponse, ClientError> {
        let domain_name_struct = DomainName::new_from_string(domain_name.to_string());
        let transport_protocol_struct = ConnectionProtocol::from(transport_protocol);
        self.config.set_protocol(transport_protocol_struct);

        let response = self
            .inner_lookup(
                domain_name_struct,
                Rrtype::from(rrtype),
                Rclass::from(rclass),
            )
            .await;

        return self.check_error_from_msg(response);
    }

    // TODO: move and change as from method  of rr
    fn from_rr_to_ip(rr: ResourceRecord) -> Result<IpAddr, ClientError> {
        let rdata = rr.get_rdata();
        if let Rdata::A(ip) = rdata {
            return Ok(ip.get_address());
        } else {
            Err(ClientError::TemporaryError(
                "Response does not match type A.",
            ))?
        }
    }

    /// Host name to address translation.
    ///
    /// Performs a DNS lookup for the given domain name and returns the corresponding
    /// `Result<LookupResponse, ResolverError>`. Here, the `LookupResponse` contains the
    /// response of the query which can translate the response to different formats.
    ///
    /// This lookup is done asynchronously using the `tokio` runtime. It calls the
    /// asynchronous method `run()` of the `Resolution` struct. This method
    /// is used to perform the DNS lookup and return the response of the query.
    ///
    /// If the response has an error, the method returns the corresponding `ResolverError`
    /// to the Client.
    ///
    /// # Examples
    ///
    /// ```
    /// use dns_resolver::resolver::config::ResolverConfig;
    /// use dns_resolver::resolver::async_resolver::AsyncResolver;
    ///
    /// let resolver = AsyncResolver::new(ResolverConfig::default());
    /// let domain_name = DomainName::new_from_string("example.com".to_string());
    /// let response = resolver.inner_lookup(domain_name).await;
    /// assert!(response.is_ok());
    /// ```
    /// TODO: Refactor to use the three caches
    async fn inner_lookup(
        &self,
        domain_name: DomainName,
        rrtype: Rrtype,
        rclass: Rclass,
    ) -> Result<LookupResponse, ResolverError> {
        let mut query = message::create_recursive_query(domain_name.clone(), rrtype, rclass);

        let config = self.config.clone();

        if config.get_ends0() {
            config.add_edns0_to_message(&mut query);
        }

        // Cache lookup
        // Search in cache only if its available
        if self.config.is_cache_enabled() {
            let lock_result = self.cache.lock();
            let cache = match lock_result {
                Ok(val) => val,
                Err(_) => Err(ClientError::Message("Error getting cache"))?, // FIXME: it shouldn't
                                                                             // return the error, it should go to the next part of the code
            };
            if let Some(cache_lookup) = cache.clone().get(query.clone()) {
                let new_lookup_response = LookupResponse::new(cache_lookup.clone());

                return Ok(new_lookup_response);
            }
        }

        let mut lookup_strategy = Resolution::new(query, self.config.clone());

        // TODO: add general timeout
        let lookup_response = lookup_strategy.run().await;

        if let Ok(ref r) = lookup_response {
            self.store_data_cache(r.to_dns_msg().clone());
        }

        return lookup_response;
    }

    /// Performs the reverse query of the given IP address.
    ///
    /// [RFC 1034]: https://datatracker.ietf.org/doc/html/rfc1034#section-5.2
    ///
    /// 5.2. Client-resolver interface
    ///
    /// Host address to host name translation
    ///
    /// This function will often follow the form of previous
    /// functions.  Given a 32 bit IP address, the caller wants a
    /// character string.  The octets of the IP address are reversed,
    /// used as name components, and suffixed with "IN-ADDR.ARPA".  A
    /// type PTR query is used to get the RR with the primary name of
    /// the host.  For example, a request for the host name
    /// corresponding to IP address 1.2.3.4 looks for PTR RRs for
    /// domain name "4.3.2.1.IN-ADDR.ARPA".
    pub async fn reverse_query() {
        unimplemented!()
    }

    /// Stores the data of the response in the cache.
    ///
    /// This method stores the data of the response in the cache, depending on the
    /// type of response.
    ///
    /// [RFC 1035]: https://datatracker.ietf.org/doc/html/rfc1035#section-7.4
    ///
    /// 7.4. Using the cache
    ///
    /// In general, we expect a resolver to cache all data which it receives in
    /// responses since it may be useful in answering future client requests.
    /// However, there are several types of data which should not be cached:
    ///
    ///     - When several RRs of the same type are available for a
    ///     particular owner name, the resolver should either cache them
    ///     all or none at all.  When a response is truncated, and a
    ///     resolver doesn't know whether it has a complete set, it should
    ///     not cache a possibly partial set of RRs.
    ///
    ///   - Cached data should never be used in preference to
    ///     authoritative data, so if caching would cause this to happen
    ///     the data should not be cached.
    ///
    ///   - The results of an inverse query should not be cached.
    ///
    ///   - The results of standard queries where the QNAME contains "*"
    ///     labels if the data might be used to construct wildcards.  The
    ///     reason is that the cache does not necessarily contain existing
    ///     RRs or zone boundary information which is necessary to
    ///     restrict the application of the wildcard RRs.
    ///
    ///   - RR data in responses of dubious reliability.  When a resolver
    ///     receives unsolicited responses or RR data other than that
    ///     requested, it should discard it without caching it.  The basic
    ///     implication is that all sanity checks on a packet should be
    ///     performed before any of it is cached.
    ///
    /// In a similar vein, when a resolver has a set of RRs for some name in a
    /// response, and wants to cache the RRs, it should check its cache for
    /// already existing RRs.  Depending on the circumstances, either the data
    /// in the response or the cache is preferred, but the two should never be
    /// combined.  If the data in the response is from authoritative data in the
    /// answer section, it is always preferred.
    fn store_data_cache(&self, response: DnsMessage) {
        let truncated = response.get_header().get_tc();
        let rcode = response.get_header().get_rcode();
        {
            let mut cache = self.cache.lock().unwrap();
            cache.timeout();
            if !truncated {
                cache.add(response.clone());
            }
        }
    }

    /// Stores the data of negative answers in the cache.
    ///
    /// [RFC 1123]: https://datatracker.ietf.org/doc/html/rfc1123#section-6.1.3.3
    ///
    /// 6.1.3.3  Efficient Resource Usage
    ///
    /// (4)  All DNS name servers and resolvers SHOULD cache
    /// negative responses that indicate the specified name, or
    /// data of the specified type, does not exist, as
    /// described in [DNS:2].
    ///
    /// [RFC 1034]: https://datatracker.ietf.org/doc/html/rfc1034#section-4.3.4
    ///
    /// 4.3.4. Negative response caching (Optional)
    ///
    /// The DNS provides an optional service which allows name servers to
    /// distribute, and resolvers to cache, negative results with TTLs.  For
    /// example, a name server can distribute a TTL along with a name error
    /// indication, and a resolver receiving such information is allowed to
    /// assume that the name does not exist during the TTL period without
    /// consulting authoritative data.  Similarly, a resolver can make a query
    /// with a QTYPE which matches multiple types, and cache the fact that some
    /// of the types are not present.
    ///
    /// The method is that a name server may add an SOA RR to the additional
    /// section of a response when that response is authoritative.  The SOA must
    /// be that of the zone which was the source of the authoritative data in
    /// the answer section, or name error if applicable.  The MINIMUM field of
    /// the SOA controls the length of time that the negative result may be
    /// cached.
    #[allow(unused)]
    fn save_negative_answers(&self, response: DnsMessage) {
        let qname = response.get_question().get_qname();
        let qtype = response.get_question().get_rrtype();
        let qclass = response.get_question().get_rclass();
        let rcode = response.get_header().get_rcode();
        let additionals = response.get_additional();
        let answer = response.get_answer();
        let aa = response.get_header().get_aa();

        // If not existence RR for query, add SOA to cache
        let mut cache = self.cache.lock().unwrap(); // FIXME: que la función entregue result
        if additionals.len() > 0 && answer.len() == 0 && aa == true {
            for additional in additionals {
                if additional.get_rtype() == Rrtype::SOA {
                    cache.add_additional(
                        qname.clone(),
                        additional,
                        Some(qtype),
                        qclass,
                        Some(rcode),
                    );
                }
            }
        }
    }

    /// Checks the received `LookupResponse` for errors to return to the Client.
    ///
    /// After receiving the response of the query, this method checks if the
    /// corresponding `DnsMessage` contained in the `LookupResponse` has any
    /// error. This error could be specified in the RCODE of the DNS message or it
    /// could be any other temporary error. If the response has an error, the method
    /// returns the corresponding`ClientError` to the Client.
    fn check_error_from_msg(
        &self,
        response: Result<LookupResponse, ResolverError>,
    ) -> Result<LookupResponse, ClientError> {
        let lookup_response = match response {
            Ok(val) => val,
            Err(_) => Err(ClientError::TemporaryError("no DNS message found"))?,
        };

        let header = lookup_response.to_dns_msg().get_header();
        let rcode = Rcode::from(header.get_rcode());
        if let Rcode::NOERROR = rcode {
            let answer = lookup_response.to_dns_msg().get_answer();
            if answer.len() == 0 {
                Err(ClientError::TemporaryError("no answer found"))?;
            }
            return Ok(lookup_response);
        }
        match rcode {
            Rcode::FORMERR => Err(ClientError::FormatError("The name server was unable to interpret the query."))?,
            Rcode::SERVFAIL => Err(ClientError::ServerFailure("The name server was unable to process this query due to a problem with the name server."))?,
            Rcode::NXDOMAIN => Err(ClientError::NameError("The domain name referenced in the query does not exist."))?,
            Rcode::NOTIMP => Err(ClientError::NotImplemented("The name server does not support the requested kind of query."))?,
            Rcode::REFUSED => Err(ClientError::Refused("The name server refuses to perform the specified operation for policy reasons."))?,
            _ => Err(ClientError::ResponseError(rcode.into()))?,
        }
    }
}

// Getters
impl AsyncResolver {
    // Gets the cache from the struct
    pub fn get_cache(&self) -> ResolverCache {
        let cache = self.cache.lock().unwrap(); // FIXME: ver que hacer con el error
        return cache.clone();
    }
}

//TODO: FK test config and documentation

#[cfg(test)]
mod async_resolver_test {
    use super::lookup_response::LookupResponse;
    use super::AsyncResolver;
    use crate::async_resolver::config::ResolverConfig;
    use crate::async_resolver::resolver_error::ResolverError;
    use crate::async_resolver::server_info::ServerInfo;
    use crate::client::client_connection::ClientConnection;
    use crate::client::client_error::ClientError;
    use crate::client::tcp_connection::ClientTCPConnection;
    use crate::client::udp_connection::ClientUDPConnection;
    use crate::dns_cache::CacheKey;
    use crate::domain_name::DomainName;
    use crate::message::rclass::Rclass;
    use crate::message::rcode::Rcode;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::soa_rdata::SoaRdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;
    use crate::message::rrtype::Rrtype;
    use crate::message::DnsMessage;
    use std::net::{IpAddr, Ipv4Addr};
    use std::str::FromStr;
    use std::time::Duration;
    use std::vec;
    use tokio::io;
    static TIMEOUT: u64 = 45;
    use std::num::NonZeroUsize;
    use std::sync::Arc;

    #[test]
    fn create_async_resolver() {
        let config = ResolverConfig::default();
        let resolver = AsyncResolver::new(config.clone());
        assert_eq!(resolver.config, config);
        assert_eq!(resolver.config.get_timeout(), Duration::from_secs(TIMEOUT));
    }

    #[tokio::test]
    async fn inner_lookup_rrtype_a() {
        // Create a new resolver with default values
        let resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = DomainName::new_from_string("example.com".to_string());
        let rrtype = Rrtype::A;
        let record_class = Rclass::IN;
        let response = resolver
            .inner_lookup(domain_name, rrtype, record_class)
            .await;

        let response = match response {
            Ok(val) => val,
            Err(error) => panic!("Error in the response: {:?}", error),
        };
        //analize if the response has the correct type according with the rrtype
        let answers = response.to_dns_msg().get_answer();
        for answer in answers {
            let a_rdata = answer.get_rdata();
            // Check if the answer is A type
            assert!(matches!(a_rdata, Rdata::A(_a_rdata)))
        }
    }

    #[tokio::test]
    async fn inner_lookup_rrtype_ns() {
        // Create a new resolver with default values
        let resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = DomainName::new_from_string("example.com".to_string());
        let rrtype = Rrtype::NS;
        let record_class = Rclass::IN;
        let response = resolver
            .inner_lookup(domain_name, rrtype, record_class)
            .await;

        let response = match response {
            Ok(val) => val,
            Err(error) => panic!("Error in the response: {:?}", error),
        };
        //analize if the response has the correct type according with the rrtype
        let answers = response.to_dns_msg().get_answer();
        for answer in answers {
            let ns_rdata = answer.get_rdata();
            // Check if the answer is NS type
            assert!(matches!(ns_rdata, Rdata::NS(_ns_rdata)))
        }
    }

    #[tokio::test]
    async fn inner_lookup_rrtype_mx() {
        // Create a new resolver with default values
        let resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = DomainName::new_from_string("example.com".to_string());
        let rrtype = Rrtype::MX;
        let record_class = Rclass::IN;
        let response = resolver
            .inner_lookup(domain_name, rrtype, record_class)
            .await;

        let response = match response {
            Ok(val) => val,
            Err(error) => panic!("Error in the response: {:?}", error),
        };
        //analize if the response has the correct type according with the qtype
        let answers = response.to_dns_msg().get_answer();
        for answer in answers {
            let mx_rdata = answer.get_rdata();
            // Check if the answer is MX type
            assert!(matches!(mx_rdata, Rdata::MX(_mx_rdata)))
        }
    }

    #[tokio::test]
    async fn inner_lookup_rrtype_ptr() {
        // Create a new resolver with default values
        let resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = DomainName::new_from_string("example.com".to_string());
        let rrtype = Rrtype::PTR;
        let record_class = Rclass::IN;
        let response = resolver
            .inner_lookup(domain_name, rrtype, record_class)
            .await;

        let response = match response {
            Ok(val) => val,
            Err(error) => panic!("Error in the response: {:?}", error),
        };
        //analize if the response has the correct type according with the qtype
        let answers = response.to_dns_msg().get_answer();
        for answer in answers {
            let ptr_rdata = answer.get_rdata();
            // Check if the answer is PTR type
            assert!(matches!(ptr_rdata, Rdata::PTR(_ptr_rdata)))
        }
    }

    #[tokio::test]
    async fn inner_lookup_rrtype_soa() {
        // Create a new resolver with default values
        let resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = DomainName::new_from_string("example.com".to_string());
        let rrtype = Rrtype::SOA;
        let record_class = Rclass::IN;
        let response = resolver
            .inner_lookup(domain_name, rrtype, record_class)
            .await;

        let response = match response {
            Ok(val) => val,
            Err(error) => panic!("Error in the response: {:?}", error),
        };
        //analize if the response has the correct type according with the qtype
        let answers = response.to_dns_msg().get_answer();
        for answer in answers {
            let soa_rdata = answer.get_rdata();
            // Check if the answer is SOA type
            assert!(matches!(soa_rdata, Rdata::SOA(_soa_rdata)))
        }
    }

    #[tokio::test]
    async fn inner_lookup_rrtype_txt() {
        // Create a new resolver with default values
        let resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = DomainName::new_from_string("example.com".to_string());
        let rrtype = Rrtype::TXT;
        let record_class = Rclass::IN;
        let response = resolver
            .inner_lookup(domain_name, rrtype, record_class)
            .await;

        let response = match response {
            Ok(val) => val,
            Err(error) => panic!("Error in the response: {:?}", error),
        };
        //analize if the response has the correct type according with the qtype
        let answers = response.to_dns_msg().get_answer();
        for answer in answers {
            let txt_rdata = answer.get_rdata();
            // Check if the answer is TXT type
            assert!(matches!(txt_rdata, Rdata::TXT(_txt_rdata)))
        }
    }

    #[tokio::test]
    async fn inner_lookup_rrtype_cname() {
        // Create a new resolver with default values
        let resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = DomainName::new_from_string("example.com".to_string());
        let rrtype = Rrtype::CNAME;
        let record_class = Rclass::IN;
        let response = resolver
            .inner_lookup(domain_name, rrtype, record_class)
            .await;

        let response = match response {
            Ok(val) => val,
            Err(error) => panic!("Error in the response: {:?}", error),
        };
        //analize if the response has the correct type according with the qtype
        let answers = response.to_dns_msg().get_answer();
        for answer in answers {
            let cname_rdata = answer.get_rdata();
            // Check if the answer is CNAME type
            assert!(matches!(cname_rdata, Rdata::CNAME(_cname_rdata)))
        }
    }

    #[tokio::test]
    async fn inner_lookup_rrtype_hinfo() {
        // Create a new resolver with default values
        let resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = DomainName::new_from_string("example.com".to_string());
        let rrtype = Rrtype::HINFO;
        let record_class = Rclass::IN;
        let response = resolver
            .inner_lookup(domain_name, rrtype, record_class)
            .await;

        let response = match response {
            Ok(val) => val,
            Err(error) => panic!("Error in the response: {:?}", error),
        };
        //analize if the response has the correct type according with the qtype
        let answers = response.to_dns_msg().get_answer();
        for answer in answers {
            let hinfo_rdata = answer.get_rdata();
            // Check if the answer is HINFO type
            assert!(matches!(hinfo_rdata, Rdata::HINFO(_hinfo_rdata)))
        }
    }

    #[tokio::test]
    async fn inner_lookup_rrtype_tsig() {
        // Create a new resolver with default values
        let resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = DomainName::new_from_string("example.com".to_string());
        let rrtype = Rrtype::TSIG;
        let record_class = Rclass::IN;
        let response = resolver
            .inner_lookup(domain_name, rrtype, record_class)
            .await;

        let response = match response {
            Ok(val) => val,
            Err(error) => panic!("Error in the response: {:?}", error),
        };
        //analize if the response has the correct type according with the rrtype
        let answers = response.to_dns_msg().get_answer();
        for answer in answers {
            let tsig_rdata = answer.get_rdata();
            // Check if the answer is TSIG type
            assert!(matches!(tsig_rdata, Rdata::TSIG(_tsig_rdata)))
        }
    }
    #[tokio::test]
    async fn inner_lookup_ns() {
        // Create a new resolver with default values
        let resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = DomainName::new_from_string("example.com".to_string());
        let rrtype = Rrtype::NS;
        let record_class = Rclass::IN;

        let response = resolver
            .inner_lookup(domain_name, rrtype, record_class)
            .await;
        assert!(response.is_ok());

        //FIXME: add assert
        println!("Response: {:?}", response);
    }

    #[tokio::test]
    async fn host_name_to_host_address_translation() {
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = "example.com";
        let rclass = "IN";
        let ip_addresses = resolver.lookup_ip(domain_name,rclass).await.unwrap();
        println!("RESPONSE : {:?}", ip_addresses);

        assert!(ip_addresses[0].is_ipv4());
        assert!(!ip_addresses[0].is_unspecified());
    }

    #[tokio::test]
    async fn lookup_ip_ch() {
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = "example.com";
        let rclass = "CH";
        let ip_addresses = resolver.lookup_ip(domain_name,rclass).await;
        println!("RESPONSE : {:?}", ip_addresses);

        assert!(ip_addresses.is_err());
    }

    #[tokio::test]
    async fn lookup_ip_rclass_any() {
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = "example.com";
        let rclass = "ANY";
        let ip_addresses = resolver.lookup_ip(domain_name,rclass).await;
        println!("RESPONSE : {:?}", ip_addresses);

        assert!(ip_addresses.is_err());
    }

    #[tokio::test]
    async fn lookup_ch() {
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = "example.com";
        let transport_protocol = "UDP";
        let rrtype = "NS";
        let rclass = "CH";
        let ip_addresses = resolver
            .lookup(domain_name, transport_protocol, rrtype, rclass)
            .await;
        println!("RESPONSE : {:?}", ip_addresses);

        assert!(ip_addresses.is_err());
    }

    #[tokio::test]
    async fn host_name_to_host_address_translation_ch() {
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = "example.com";
        let rclass = "IN";
        let ip_addresses = resolver.lookup_ip(domain_name,rclass).await.unwrap();
        println!("RESPONSE : {:?}", ip_addresses);

        assert!(ip_addresses[0].is_ipv4());
        assert!(!ip_addresses[0].is_unspecified());
    }

    #[tokio::test]
    async fn lookup_ns() {
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        resolver.config.set_retransmission_loop_attempts(10);
        let domain_name = "example.com";
        let transport_protocol = "UDP";
        match resolver
            .lookup(domain_name, transport_protocol, "NS", "IN")
            .await
        {
            Ok(val) => {
                println!("RESPONSE : {:?}", val);
            }
            Err(e) => assert!(false, "Error: {:?}", e),
        };
    }

    // async fn reverse_query() {
    //     let resolver = AsyncResolver::new(ResolverConfig::default());
    //     let ip_address = "192.168.0.1";
    //     let domain_name = resolver.reverse_query(ip_address).await;

    //     // Realiza aserciones para verificar que domain_name contiene un nombre de dominio válido.
    //     assert!(!domain_name.is_empty(), "El nombre de dominio no debe estar vacío");

    //     // Debe verificar que devuelve el nombre de dominio correspondiente a la dirección IP dada.
    //     // Dependiendo de tu implementación, puedes comparar el resultado con un valor esperado.
    //     // Por ejemplo, si esperas que la dirección IP "192.168.0.1" se traduzca a "ejemplo.com":
    //     assert_eq!(domain_name, "ejemplo.com", "El nombre de dominio debe ser 'ejemplo.com'");
    // }

    #[tokio::test]
    async fn timeout() {
        // Crea una instancia de tu resolutor con la configuración adecuada
        let mut resolver = AsyncResolver::new(ResolverConfig::default());

        // Intenta resolver un nombre de dominio que no existe o no está accesible
        let domain_name = "nonexistent-example.com";
        let rclass = "IN";

        // Configura un timeout corto para la resolución (ajusta según tus necesidades)
        let timeout_duration = std::time::Duration::from_secs(2);

        let result = tokio::time::timeout(timeout_duration, async {
            resolver.lookup_ip(domain_name,rclass).await
        }).await;

        // Verifica que el resultado sea un error de timeout
        match result {
            Ok(Ok(_)) => {
                panic!("Se esperaba un error de timeout, pero se resolvió exitosamente");
            }
            Ok(Err(_err)) => {
                assert!(true);
            }
            Err(_) => {
                panic!("El timeout no se manejó correctamente");
            }
        }
    }

    #[test]
    fn check_dns_msg_ip() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response = DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::A,
            Rclass::IN,
            0,
            false,
            1,
        );
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let lookup_response = LookupResponse::new(dns_response);
        let result_lookup = resolver.check_error_from_msg(Ok(lookup_response));

        if let Ok(lookup_response) = result_lookup {
            let rdata = lookup_response.to_dns_msg().get_answer()[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            panic!("Error parsing response");
        }
    }

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

    #[ignore = "Taking too long"]
    #[tokio::test]
    async fn max_number_of_retry() {
        let mut config = ResolverConfig::default();
        let max_retries = 6;
        config.set_retransmission_loop_attempts(max_retries);

        let bad_server: IpAddr = IpAddr::V4(Ipv4Addr::new(7, 7, 7, 7));
        let timeout = Duration::from_secs(2);

        let conn_udp: ClientUDPConnection = ClientUDPConnection::new(bad_server, timeout);
        let conn_tcp: ClientTCPConnection = ClientTCPConnection::new(bad_server, timeout);
        let server_info = ServerInfo::new_with_ip(bad_server, conn_udp, conn_tcp);
        let name_servers = vec![server_info];
        config.set_name_servers(name_servers);
        let mut resolver = AsyncResolver::new(config);

        let result = resolver.lookup("dfasdfsda.com", "TCP", "A", "IN").await;

        match result {
            Ok(_) => {
                panic!("Timeout limit exceeded error was expected.");
            }
            Err(_) => {
                assert!(true);
            }
        }
    }

    #[tokio::test]
    async fn use_udp() {
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = "example.com";
        let rclass = "IN";
        let ip_addresses = resolver.lookup_ip(domain_name,rclass).await.unwrap();
        println!("RESPONSE : {:?}", ip_addresses);

        assert!(ip_addresses[0].is_ipv4());
        assert!(!ip_addresses[0].is_unspecified());
    }

    #[tokio::test]
    async fn use_tcp() {
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = "example.com";
        let rclass = "IN";
        let ip_addresses = resolver.lookup_ip(domain_name, rclass).await.unwrap();
        println!("RESPONSE : {:?}", ip_addresses);

        assert!(ip_addresses[0].is_ipv4());
        assert!(!ip_addresses[0].is_unspecified());
    }

    #[tokio::test]
    #[ignore]
    async fn use_udp_but_fails_and_use_tcp() {
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = "Ecample.com";
        let rclass = "IN";
        let udp_result = resolver.lookup_ip(domain_name,rclass).await;

        match udp_result {
            Ok(_) => {
                panic!("UDP client error expected");
            }
            Err(_err) => {
                assert!(true);
            }
        }

        let tcp_result = resolver.lookup_ip(domain_name,  rclass).await;
        match tcp_result {
            Ok(_) => {
                assert!(true);
            }
            Err(_err) => {
                panic!("unexpected TCP client error");
            }
        }
    }

    //TODO: diferent types of errors
    #[tokio::test]
    async fn resolver_with_client_error_io() {
        let io_error = io::Error::new(io::ErrorKind::Other, "Simulated I/O Error");
        let result = ClientError::Io(io_error);

        match result {
            ClientError::Io(_) => {
                // La operación generó un error de I/O simulado, la prueba es exitosa
            }
            _ => {
                panic!("Se esperaba un error de I/O simulado");
            }
        }
    }

    #[tokio::test]
    async fn check_dns_msg_1() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response = DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::A,
            Rclass::IN,
            0,
            false,
            1,
        );
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        header.set_rcode(Rcode::FORMERR);
        dns_response.set_header(header);
        let lookup_response = LookupResponse::new(dns_response);
        let result_lookup = resolver.check_error_from_msg(Ok(lookup_response));

        if let Ok(lookup_response) = result_lookup {
            let rdata = lookup_response.to_dns_msg().get_answer()[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            if let Err(ClientError::FormatError(
                "The name server was unable to interpret the query.",
            )) = result_lookup
            {
                assert!(true);
            } else {
                panic!("Error parsing response");
            }
        }
    }

    #[tokio::test]
    async fn parse_dns_msg_2() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response = DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::A,
            Rclass::IN,
            0,
            false,
            1,
        );
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        header.set_rcode(Rcode::SERVFAIL);
        dns_response.set_header(header);
        let lookup_response = LookupResponse::new(dns_response);
        let result_lookup = resolver.check_error_from_msg(Ok(lookup_response));

        if let Ok(lookup_response) = result_lookup {
            let rdata = lookup_response.to_dns_msg().get_answer()[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            if let Err(ClientError::ServerFailure("The name server was unable to process this query due to a problem with the name server.")) = 
            result_lookup {
                assert!(true);
            }
            else {
                panic!("Error parsing response");
            }
        }
    }

    #[tokio::test]
    async fn parse_dns_msg_3() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response = DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::A,
            Rclass::IN,
            0,
            false,
            1,
        );
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        header.set_rcode(Rcode::NXDOMAIN);
        dns_response.set_header(header);
        let lookup_response = LookupResponse::new(dns_response);
        let result_lookup = resolver.check_error_from_msg(Ok(lookup_response));

        if let Ok(lookup_response) = result_lookup {
            let rdata = lookup_response.to_dns_msg().get_answer()[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            if let Err(ClientError::NameError(
                "The domain name referenced in the query does not exist.",
            )) = result_lookup
            {
                assert!(true);
            } else {
                panic!("Error parsing response");
            }
        }
    }

    #[tokio::test]
    async fn parse_dns_msg_4() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response = DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::A,
            Rclass::IN,
            0,
            false,
            1,
        );
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        header.set_rcode(Rcode::NOTIMP);
        dns_response.set_header(header);
        let lookup_response = LookupResponse::new(dns_response);
        let result_lookup = resolver.check_error_from_msg(Ok(lookup_response));

        if let Ok(lookup_response) = result_lookup {
            let rdata = lookup_response.to_dns_msg().get_answer()[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            if let Err(ClientError::NotImplemented(
                "The name server does not support the requested kind of query.",
            )) = result_lookup
            {
                assert!(true);
            } else {
                panic!("Error parsing response");
            }
        }
    }

    #[tokio::test]
    async fn parse_dns_msg_5() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response = DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::A,
            Rclass::IN,
            0,
            false,
            1,
        );
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        header.set_rcode(Rcode::REFUSED);
        dns_response.set_header(header);
        let lookup_response = LookupResponse::new(dns_response);
        let result_lookup = resolver.check_error_from_msg(Ok(lookup_response));

        if let Ok(lookup_response) = result_lookup {
            let rdata = lookup_response.to_dns_msg().get_answer()[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            if let Err(ClientError::Refused(
                "The name server refuses to perform the specified operation for policy reasons.",
            )) = result_lookup
            {
                assert!(true);
            } else {
                panic!("Error parsing response");
            }
        }
    }

    //TODO: probar diferentes rrtype
    #[tokio::test]
    async fn rrtypes_a() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response = DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::A,
            Rclass::IN,
            0,
            false,
            1,
        );
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let lookup_response = LookupResponse::new(dns_response);
        let result_lookup = resolver.check_error_from_msg(Ok(lookup_response));

        if let Ok(lookup_response) = result_lookup {
            let rdata = lookup_response.to_dns_msg().get_answer()[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            panic!("Error parsing response");
        }
    }

    #[tokio::test]
    async fn rrtypes_ns() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response = DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::NS,
            Rclass::IN,
            0,
            false,
            1,
        );
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let lookup_response = LookupResponse::new(dns_response);
        let result_lookup = resolver.check_error_from_msg(Ok(lookup_response));

        if let Ok(lookup_response) = result_lookup {
            let rdata = lookup_response.to_dns_msg().get_answer()[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            panic!("Error parsing response");
        }
    }

    #[tokio::test]
    async fn rrtypes_cname() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response = DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::CNAME,
            Rclass::IN,
            0,
            false,
            1,
        );
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let lookup_response = LookupResponse::new(dns_response);
        let result_vec_rr = resolver.check_error_from_msg(Ok(lookup_response));

        if let Ok(lookup_response) = result_vec_rr {
            let rdata = lookup_response.to_dns_msg().get_answer()[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            panic!("Error parsing response");
        }
    }

    #[tokio::test]
    async fn rrtypes_soa() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response = DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::SOA,
            Rclass::IN,
            0,
            false,
            1,
        );
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let lookup_response = LookupResponse::new(dns_response);
        let result_vec_rr = resolver.check_error_from_msg(Ok(lookup_response));

        if let Ok(lookup_response) = result_vec_rr {
            let rdata = lookup_response.to_dns_msg().get_answer()[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            panic!("Error parsing response");
        }
    }

    #[tokio::test]
    async fn rrtypes_ptr() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response = DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::PTR,
            Rclass::IN,
            0,
            false,
            1,
        );
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let lookup_response = LookupResponse::new(dns_response);
        let result_vec_rr = resolver.check_error_from_msg(Ok(lookup_response));

        if let Ok(lookup_response) = result_vec_rr {
            let rdata = lookup_response.to_dns_msg().get_answer()[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            panic!("Error parsing response");
        }
    }

    #[tokio::test]
    async fn rrtypes_hinfo() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response = DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::HINFO,
            Rclass::IN,
            0,
            false,
            1,
        );
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let lookup_response = LookupResponse::new(dns_response);
        let result_vec_rr = resolver.check_error_from_msg(Ok(lookup_response));

        if let Ok(lookup_response) = result_vec_rr {
            let rdata = lookup_response.to_dns_msg().get_answer()[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            panic!("Error parsing response");
        }
    }

    #[tokio::test]
    async fn rrtypes_minfo() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response = DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::MINFO,
            Rclass::IN,
            0,
            false,
            1,
        );
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let lookup_response = LookupResponse::new(dns_response);
        let result_vec_rr = resolver.check_error_from_msg(Ok(lookup_response));

        if let Ok(lookup_response) = result_vec_rr {
            let rdata = lookup_response.to_dns_msg().get_answer()[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            panic!("Error parsing response");
        }
    }

    #[tokio::test]
    async fn rrtypes_wks() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response = DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::WKS,
            Rclass::IN,
            0,
            false,
            1,
        );
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let lookup_response = LookupResponse::new(dns_response);
        let result_vec_rr = resolver.check_error_from_msg(Ok(lookup_response));

        if let Ok(lookup_response) = result_vec_rr {
            let rdata = lookup_response.to_dns_msg().get_answer()[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            panic!("Error parsing response");
        }
    }

    #[tokio::test]
    async fn rrtypes_txt() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response = DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::TXT,
            Rclass::IN,
            0,
            false,
            1,
        );
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let lookup_response = LookupResponse::new(dns_response);
        let result_vec_rr = resolver.check_error_from_msg(Ok(lookup_response));

        if let Ok(lookup_response) = result_vec_rr {
            let rdata = lookup_response.to_dns_msg().get_answer()[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            panic!("Error parsing response");
        }
    }

    #[tokio::test]
    async fn rrtypes_dname() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response = DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::DNAME,
            Rclass::IN,
            0,
            false,
            1,
        );
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let lookup_response = LookupResponse::new(dns_response);
        let result_vec_rr = resolver.check_error_from_msg(Ok(lookup_response));

        if let Ok(lookup_response) = result_vec_rr {
            let rdata = lookup_response.to_dns_msg().get_answer()[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            panic!("Error parsing response");
        }
    }

    #[tokio::test]
    async fn rrtypes_any() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response = DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::ANY,
            Rclass::IN,
            0,
            false,
            1,
        );
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let lookup_response = LookupResponse::new(dns_response);
        let result_vec_rr = resolver.check_error_from_msg(Ok(lookup_response));

        if let Ok(lookup_response) = result_vec_rr {
            let rdata = lookup_response.to_dns_msg().get_answer()[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            panic!("Error parsing response");
        }
    }

    #[tokio::test]
    async fn rrtypes_tsig() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response = DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::TSIG,
            Rclass::IN,
            0,
            false,
            1,
        );
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let lookup_response = LookupResponse::new(dns_response);
        let result_vec_rr = resolver.check_error_from_msg(Ok(lookup_response));

        if let Ok(lookup_response) = result_vec_rr {
            let rdata = lookup_response.to_dns_msg().get_answer()[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            panic!("Error parsing response");
        }
    }

    #[tokio::test]
    async fn rrtypes_axfr() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response = DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::AXFR,
            Rclass::IN,
            0,
            false,
            1,
        );
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let lookup_response = LookupResponse::new(dns_response);
        let result_vec_rr = resolver.check_error_from_msg(Ok(lookup_response));

        if let Ok(lookup_response) = result_vec_rr {
            let rdata = lookup_response.to_dns_msg().get_answer()[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            panic!("Error parsing response");
        }
    }

    #[tokio::test]
    async fn rrtypes_mailb() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response = DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::MAILB,
            Rclass::IN,
            0,
            false,
            1,
        );
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let lookup_response = LookupResponse::new(dns_response);
        let result_vec_rr = resolver.check_error_from_msg(Ok(lookup_response));

        if let Ok(lookup_response) = result_vec_rr {
            let rdata = lookup_response.to_dns_msg().get_answer()[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            panic!("Error parsing response");
        }
    }

    #[tokio::test]
    async fn rrtypes_maila() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response = DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::MAILA,
            Rclass::IN,
            0,
            false,
            1,
        );
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let lookup_response = LookupResponse::new(dns_response);
        let result_vec_rr = resolver.check_error_from_msg(Ok(lookup_response));

        if let Ok(lookup_response) = result_vec_rr {
            let rdata = lookup_response.to_dns_msg().get_answer()[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            panic!("Error parsing response");
        }
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

    #[test]
    fn save_cache_negative_answer() {
        let resolver = AsyncResolver::new(ResolverConfig::default());
        resolver
            .cache
            .lock()
            .unwrap()
            .set_max_size(NonZeroUsize::new(1).unwrap());

        let domain_name = DomainName::new_from_string("banana.exaple".to_string());
        let mname = DomainName::new_from_string("a.root-servers.net.".to_string());
        let rname = DomainName::new_from_string("nstld.verisign-grs.com.".to_string());
        let serial = 2023112900;
        let refresh = 1800;
        let retry = 900;
        let expire = 604800;
        let minimum = 86400;

        //Create RR type SOA
        let mut soa_rdata = SoaRdata::new();
        soa_rdata.set_mname(mname);
        soa_rdata.set_rname(rname);
        soa_rdata.set_serial(serial);
        soa_rdata.set_refresh(refresh);
        soa_rdata.set_retry(retry);
        soa_rdata.set_expire(expire);
        soa_rdata.set_minimum(minimum);

        let rdata = Rdata::SOA(soa_rdata);
        let mut rr = ResourceRecord::new(rdata);
        rr.set_name(domain_name.clone());

        // Create dns response
        let mut dns_response =
            DnsMessage::new_query_message(domain_name, Rrtype::A, Rclass::IN, 0, false, 1);
        let mut new_header = dns_response.get_header();
        new_header.set_aa(true);
        dns_response.set_header(new_header);

        // Save RR type SOA in Additional section of response
        dns_response.add_additionals(vec![rr]);

        resolver.save_negative_answers(dns_response.clone());

        let rrtype_search = Rrtype::A;
        assert_eq!(dns_response.get_answer().len(), 0);
        assert_eq!(dns_response.get_additional().len(), 1);
        assert_eq!(
            resolver
                .cache
                .lock()
                .unwrap()
                .get_cache_answer()
                .get_cache()
                .len(),
            1
        );
        // assert!(resolver.cache.lock().unwrap().get_cache_answer().get(dns_response.get_question().get_qname().clone(), qtype_search, Qclass::IN).is_some())
    }

    /*  #[ignore = "Optional, not implemented"]
    #[tokio::test]
    async fn inner_lookup_negative_answer_in_cache(){
        let resolver = AsyncResolver::new(ResolverConfig::default());
        let mut cache = resolver.cache.lock().unwrap().get_cache_answer();
        let qtype = Qtype::A;
        cache.set_max_size(NonZeroUsize::new(9).unwrap());

        let domain_name = DomainName::new_from_string("banana.exaple".to_string());

        //Create RR type SOA
        let mname = DomainName::new_from_string("a.root-servers.net.".to_string());
        let rname = DomainName::new_from_string("nstld.verisign-grs.com.".to_string());
        let serial = 2023112900;
        let refresh = 1800;
        let retry = 900;
        let expire = 604800;
        let minimum = 86400;

        let mut soa_rdata = SoaRdata::new();
        soa_rdata.set_mname(mname);
        soa_rdata.set_rname(rname);
        soa_rdata.set_serial(serial);
        soa_rdata.set_refresh(refresh);
        soa_rdata.set_retry(retry);
        soa_rdata.set_expire(expire);
        soa_rdata.set_minimum(minimum);

        let rdata = Rdata::SOA(soa_rdata);
        let mut rr = ResourceRecord::new(rdata);
        rr.set_name(domain_name.clone());

        // Add negative answer to cache
        let mut cache  = resolver.cache.lock().unwrap().get_cache_answer();
        cache.set_max_size(NonZeroUsize::new(9).unwrap());
        cache.add_negative_answer(domain_name.clone(),qtype ,Qclass::IN, rr.clone());
        let mut cache_guard = resolver.cache.lock().unwrap().get_cache_answer();
        *cache_guard = cache;

        assert_eq!(resolver.cache.lock().unwrap().get_cache_answer().get_cache().len(), 1);

        let rclass = Rclass::IN;
        let response = resolver.inner_lookup(domain_name,rrtype,rclass).await.unwrap();

        assert_eq!(resolver.cache.lock().unwrap().get_cache_answer().get_cache().len(), 1);
        assert_eq!(response.to_dns_msg().get_answer().len(), 0);
        assert_eq!(response
            .to_dns_msg()
            .get_additional()
            .len(), 1);
        assert_eq!(response
            .to_dns_msg()
            .get_header()
            .get_rcode(), Rcode::NXDOMAIN);
    } */

    // TODO: Finish tests, it shoudl verify that we can send several asynchroneous queries concurrently
    #[tokio::test]
    async fn test3() {
        let resolver = Arc::new(AsyncResolver::new(ResolverConfig::default()));
        let rrtype = Rrtype::A;
        let rclass = Rclass::IN;

        let domain_name = DomainName::new_from_string("example.com".to_string());
        let resolver_1 = resolver.clone();
        let resolver_2 = resolver.clone();

        let _result: (
            Result<LookupResponse, ResolverError>,
            Result<LookupResponse, ResolverError>,
        ) = tokio::join!(
            resolver_1.inner_lookup(domain_name.clone(), rrtype.clone(), rclass.clone()),
            resolver_2.inner_lookup(domain_name.clone(), rrtype.clone(), rclass.clone())
        );
    }
}
