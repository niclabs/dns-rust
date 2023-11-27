use std::net::IpAddr;

pub mod config;
pub mod lookup;
pub mod slist;
pub mod resolver_error;

use rand::{thread_rng, Rng};

use crate::client::client_error::ClientError;
use crate::dns_cache::DnsCache;
use crate::domain_name::DomainName;
use crate::message::DnsMessage;
use crate::message::class_qclass::Qclass;
use crate::message::resource_record::ResourceRecord;
use crate::async_resolver::{config::ResolverConfig,lookup::LookupFutureStub};
use crate::message::rdata::Rdata;
use crate::client::client_connection::ConnectionProtocol;
use crate::async_resolver::resolver_error::ResolverError;
use crate:: message::type_qtype::Qtype;
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
pub struct AsyncResolver{
    /// Cache for the resolver.
    cache: DnsCache,
    /// Configuration for the resolver.
    config: ResolverConfig ,
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
    pub fn new(config: ResolverConfig)-> Self {
        let async_resolver = AsyncResolver {
            cache: DnsCache::new(),
            config: config,
        };
        async_resolver
    } 


    pub fn run(){
        unimplemented!()
    }

    /// [RFC 1034]: https://datatracker.ietf.org/doc/html/rfc1034#section-5.2
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
    /// 
    /// FIXME: DEBE RETORNAR CLIENT ERROR
    /// This method acts as an interface between the Client and the Resolver.
    /// It calls `inner_lookup(&self, domain_name: DomainName)` which will
    /// execute a look up of the given domain name asynchronously.
    pub async fn lookup_ip(&mut self, domain_name: &str, transport_protocol: &str) -> Result<Vec<IpAddr>, ClientError> {
        println!("[LOOKUP IP ASYNCRESOLVER]");

        let domain_name_struct = DomainName::new_from_string(domain_name.to_string());

        let transport_protocol_struct = ConnectionProtocol::from(transport_protocol);
        self.config.set_protocol(transport_protocol_struct);

        let response = self.inner_lookup(domain_name_struct,Qtype::A).await;

        let result_rrs = self.parse_dns_msg(response);
        if let Ok(rrs) = result_rrs {
            let rrs_iter = rrs.into_iter();
            let ip_addresses: Result<Vec<IpAddr>, _> = rrs_iter.map(|rr| 
                {AsyncResolver::from_rr_to_ip(rr)}).collect();
            return ip_addresses;
        } else {
            Err(ClientError::TemporaryError("Error parsing response."))?
        }
    }

    // TODO: move and change as from method  of rr
    fn from_rr_to_ip(rr: ResourceRecord) -> Result<IpAddr, ClientError> {
        let rdata = rr.get_rdata();
        if let Rdata::A(ip) = rdata {
            return Ok(ip.get_address());
        } else {
            Err(ClientError::TemporaryError("Response does not match type A."))?
        }
    }
 
    /// Parses the received `DnsMessage` and returns the corresponding RRs.
    /// 
    /// After receiving the response of the query, this method parses the DNS message
    /// of type `DnsMessage` to a `Vec<ResourceRecord>` with the corresponding resource
    /// records contained in the message. It will return the RRs if the response was
    /// successful. If the response was not successful, it will return the corresponding
    /// error message to the Client.
    fn parse_dns_msg(&self, response: Result<DnsMessage, ResolverError>) -> Result<Vec<ResourceRecord>, ClientError> {
        let dns_mgs = match response {
            Ok(val) => val,
            Err(_) => Err(ClientError::TemporaryError("no DNS message found"))?,
        };

        let header = dns_mgs.get_header();
        let rcode = header.get_rcode();
        if rcode == 0 {
            let answer = dns_mgs.get_answer();
            return Ok(answer);
        } 
        match rcode {
            1 => Err(ClientError::FormatError("The name server was unable to interpret the query."))?,
            2 => Err(ClientError::ServerFailure("The name server was unable to process this query due to a problem with the name server."))?,
            3 => Err(ClientError::NameError("The domain name referenced in the query does not exist."))?,
            4 => Err(ClientError::NotImplemented("The name server does not support the requested kind of query."))?,
            5 => Err(ClientError::Refused("The name server refuses to perform the specified operation for policy reasons."))?,
            _ => Err(ClientError::ResponseError(rcode))?,
        }
    }

    /// Host name to address translation.
    /// 
    /// Performs a DNS lookup for the given domain name and returns the 
    /// corresponding IP address. This lookup is done asynchronously using
    /// the future `LookupIpFutureStub`.
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
    async fn inner_lookup(&mut self, domain_name: DomainName,qtype:Qtype) -> Result<DnsMessage, ResolverError> {

        // Cache lookup
        // Search in cache only if its available
        if self.config.is_cache_enabled() {
            if let Some(cache_lookup) = self.cache.clone().get(domain_name.clone(), Qtype::to_rtype(qtype)) {
                println!("[Cached Data]");

                // Create random generator
                let mut rng = thread_rng();

                // Create query id
                let query_id: u16 = rng.gen();

                // Create query
                let mut new_query = DnsMessage::new_query_message(
                    domain_name.clone(),
                    qtype,
                    Qclass::IN,
                    0,
                    false,
                    query_id
                );
        
                // Add Answer
                let answer: Vec<ResourceRecord> = cache_lookup
                                                    .iter()
                                                    .map(|rr_cache_value| rr_cache_value.get_resource_record())
                                                    .collect::<Vec<ResourceRecord>>();
                new_query.set_answer(answer);
                return Ok(new_query)
            }
        }

        // Async query
        let response = LookupFutureStub::lookup(
            domain_name,
            qtype,
            self.config.clone())
            .await;

        // Cache data
        if let Ok(ref r) = response {
            self.store_data_cache(r.clone());
        }

        response
    }

    /// [RFC 1034]: https://datatracker.ietf.org/doc/html/rfc1034#section-5.2
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
    /// 
    /// Reverse query function

    pub async fn reverse_query() {
        unimplemented!()
    }

    /// [RFC 1034]: https://datatracker.ietf.org/doc/html/rfc1034#section-5.2
    /// 5.2 Client-resolver interface
    /// 
    /// 3. General lookup function
    /// 
    /// This function retrieves arbitrary information from the DNS,
    /// and has no counterpart in previous systems.  The caller
    /// supplies a QNAME, QTYPE, and QCLASS, and wants all of the
    /// matching RRs.  This function will often use the DNS format
    /// for all RR data instead of the local host's, and returns all
    /// RR content (e.g., TTL) instead of a processed form with local
    /// quoting conventions.
    /// 
    /// This method will perform a inner lookup of the given domain name
    /// and qtype, returning the corresponding resource records.
    /// 
    /// # Examples
    /// ```
    /// let mut resolver = AsyncResolver::new(ResolverConfig::default());
    /// let domain_name = "example.com";
    /// let transport_protocol = "UDP";
    /// let qtype = "NS";
    /// let response = resolver.lookup(domain_name, transport_protocol,qtype).await.unwrap();
    /// ```
    /// 
    pub async fn lookup(&mut self, domain_name: &str, transport_protocol: &str, qtype:&str ) -> Result<Vec<ResourceRecord>, ResolverError> {
        println!("[LOOKUP ASYNCRESOLVER]");

        let domain_name_struct = DomainName::new_from_string(domain_name.to_string());
        let qtype_struct = Qtype::from_str_to_qtype(qtype);
        let transport_protocol_struct = ConnectionProtocol::from(transport_protocol);
        self.config.set_protocol(transport_protocol_struct);

        let response = self.inner_lookup(domain_name_struct,qtype_struct).await;
        
        return self.parse_dns_msg(response).map_err(Into::into)
    }

    /// Stores the data of the response in the cache.
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
    /// 
    /// This method stores the data of the response in the cache, depending on the
    /// type of response.
    fn store_data_cache(&mut self, response: DnsMessage) {

        let truncated = response.get_header().get_tc();

        if !truncated {
            // TODO: RFC 1035: 7.4. Using the cache
            response.get_answer()
            .iter()
            .for_each(|rr| self.cache.add(rr.get_name(), rr.clone()));
        } 

    }

}


// Getters
impl AsyncResolver {
    // Gets the cache from the struct
    pub fn get_cache(&self) -> DnsCache {
        self.cache.clone()
    }
}

//TODO: FK test config and documentation

#[cfg(test)]
mod async_resolver_test {
    use tokio::io;

    use crate::client::client_error::ClientError;
    use crate::message::DnsMessage;
    use crate::message::class_qclass::Qclass;
    use crate::message::rdata::Rdata;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::resource_record::ResourceRecord;
    use crate:: message::type_qtype::Qtype;
    use crate::message::type_rtype::Rtype;
    use crate::async_resolver::config::ResolverConfig;
    use super::AsyncResolver;
    use std::net::IpAddr;
    use std::str::FromStr;
    use std::time::Duration;
    use crate::domain_name::DomainName;
    static TIMEOUT: u64 = 10;
    
    #[test]
    fn create_async_resolver() {
        let config = ResolverConfig::default();
        let resolver = AsyncResolver::new(config.clone());
        assert_eq!(resolver.config, config);
        assert_eq!(resolver.config.get_timeout(), Duration::from_secs(TIMEOUT));
    }

    #[tokio::test]
    async fn inner_lookup() {
        // Create a new resolver with default values
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = DomainName::new_from_string("example.com".to_string());
        let qtype = Qtype::A;
        let response = resolver.inner_lookup(domain_name,qtype).await;

        //FIXME: add assert
        assert!(response.is_ok());
    } 

    #[tokio::test]
    async fn inner_lookup_ns() {
        // Create a new resolver with default values
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = DomainName::new_from_string("example.com".to_string());
        let qtype = Qtype::NS;
        let response = resolver.inner_lookup(domain_name,qtype).await;
        assert!(response.is_ok());

        //FIXME: add assert
        println!("Response: {:?}",response);
    }

    #[tokio::test]
    async fn host_name_to_host_address_translation() {
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = "example.com";
        let transport_protocol = "TCP";
        let ip_addresses = resolver.lookup_ip(domain_name, transport_protocol).await.unwrap();
        println!("RESPONSE : {:?}", ip_addresses);
        
        assert!(ip_addresses[0].is_ipv4());
    
        assert!(!ip_addresses[0].is_unspecified());
    }

    #[ignore]
    #[tokio::test]
    async fn lookup_ns() {
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = "example.com";
        let transport_protocol = "UDP";
        let qtype = "NS";
        let response = resolver.lookup(domain_name, transport_protocol,qtype).await.unwrap();
        
        println!("RESPONSE : {:?}",response);
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
        let transport_protocol = "UDP";
    
        // Configura un timeout corto para la resolución (ajusta según tus necesidades)
        let timeout_duration = std::time::Duration::from_secs(2);
        
        let result = tokio::time::timeout(timeout_duration, async {
            resolver.lookup_ip(domain_name, transport_protocol).await
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
    fn parse_dns_msg_ip() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::A,
                Qclass::IN,
                0,
                false,
                1);
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let result_vec_rr = resolver.parse_dns_msg(Ok(dns_response));

        if let Ok(rrs) = result_vec_rr {
            let rdata = rrs[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            panic!("Error parsing response");
        }
    }

    /// Test lookup cache
    #[tokio::test]
    async fn lookup_cache() {
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        resolver.cache.set_max_size(1);

        let domain_name = DomainName::new_from_string("example.com".to_string());
        let a_rdata = ARdata::new_from_addr(IpAddr::from_str("93.184.216.34").unwrap());
        let a_rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(a_rdata);

        resolver.cache.add(domain_name, resource_record);

        let _response = resolver.lookup("example.com", "UDP", "A").await;
    }

    /// Test cache data
    #[tokio::test]
    async fn cache_data() {
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        resolver.cache.set_max_size(1);
        assert_eq!(resolver.cache.is_empty(), true);
        let _response = resolver.lookup("example.com", "UDP", "A").await;
        assert_eq!(resolver.cache.is_cached(DomainName::new_from_str("example.com"), Rtype::A), true);

        // TODO: Test special cases from RFC
    }


    //TODO: test max number of retry
    #[tokio::test]
    async fn max_number_of_retry() {
        let mut config = ResolverConfig::default();
        let max_retries = 6;
        config.set_retry(max_retries);
        let mut resolver = AsyncResolver::new(config);

        // Realiza una resolución de DNS que sabes que fallará
        //let result = resolver.lookup_ip("nonexisten.comt-domain", "UDP").await;

        let mut retries_attempted = 0;

        // Realiza la resolución de DNS que sabes que fallará
        while retries_attempted < max_retries {
            let result = resolver.lookup_ip("nonexistent-domain.com", "UDP").await;
             retries_attempted += 1;

            if result.is_ok() {
                break; // La resolución tuvo éxito, sal del bucle
            }
        }
        if retries_attempted == max_retries {
            assert!(retries_attempted == max_retries, "Número incorrecto de reintentos");
        } else {
            panic!("La resolución DNS tuvo éxito antes de lo esperado");
        }
        
    }

 
    #[tokio::test]
    async fn use_udp() {
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = "example.com";
        let transport_protocol = "UDP";
        let ip_addresses = resolver.lookup_ip(domain_name, transport_protocol).await.unwrap();
        println!("RESPONSE : {:?}", ip_addresses);
        
        assert!(ip_addresses[0].is_ipv4());
    
        assert!(!ip_addresses[0].is_unspecified());
    }
  
    #[tokio::test]
    async fn use_tcp() {
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = "example.com";
        let transport_protocol = "TCP";
        let ip_addresses = resolver.lookup_ip(domain_name, transport_protocol).await.unwrap();
        println!("RESPONSE : {:?}", ip_addresses);
        
        assert!(ip_addresses[0].is_ipv4());
    
        assert!(!ip_addresses[0].is_unspecified());
    }

    
    #[ignore = ""]
    #[tokio::test]
    async fn use_udp_but_fails_and_use_tcp() {
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        let domain_name = "Ecample.com";

        let udp_result = resolver.lookup_ip(domain_name, "UDP").await;
    
       
       match udp_result {
        Ok(_) => {
            panic!("UDP client error expected");
        }
           
       
       Err(_err) => {
        assert!(true);
       }
      
      } 

      let tcp_result = resolver.lookup_ip(domain_name, "TCP").await;

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
    async fn parse_dns_msg_1() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::A,
                Qclass::IN,
                0,
                false,
                1);
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        header.set_rcode(1);
        dns_response.set_header(header);
        let result_vec_rr = resolver.parse_dns_msg(Ok(dns_response));

        if let Ok(rrs) = result_vec_rr {
            let rdata = rrs[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            if let Err(ClientError::FormatError("The name server was unable to interpret the query.")) = result_vec_rr {
                assert!(true);
            }
            else {
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

        let mut dns_response =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::A,
                Qclass::IN,
                0,
                false,
                1);
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        header.set_rcode(2);
        dns_response.set_header(header);
        let result_vec_rr = resolver.parse_dns_msg(Ok(dns_response));

        if let Ok(rrs) = result_vec_rr {
            let rdata = rrs[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            if let Err(ClientError::ServerFailure("The name server was unable to process this query due to a problem with the name server.")) = result_vec_rr {
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

        let mut dns_response =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::A,
                Qclass::IN,
                0,
                false,
                1);
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        header.set_rcode(3);
        dns_response.set_header(header);
        let result_vec_rr = resolver.parse_dns_msg(Ok(dns_response));

        if let Ok(rrs) = result_vec_rr {
            let rdata = rrs[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            if let Err(ClientError::NameError("The domain name referenced in the query does not exist.")) = result_vec_rr {
                assert!(true);
            }
            else {
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

        let mut dns_response =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::A,
                Qclass::IN,
                0,
                false,
                1);
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        header.set_rcode(4);
        dns_response.set_header(header);
        let result_vec_rr = resolver.parse_dns_msg(Ok(dns_response));

        if let Ok(rrs) = result_vec_rr {
            let rdata = rrs[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            if let Err(ClientError::NotImplemented("The name server does not support the requested kind of query.")) = result_vec_rr {
                assert!(true);
            }
            else {
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

        let mut dns_response =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::A,
                Qclass::IN,
                0,
                false,
                1);
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        header.set_rcode(5);
        dns_response.set_header(header);
        let result_vec_rr = resolver.parse_dns_msg(Ok(dns_response));

        if let Ok(rrs) = result_vec_rr {
            let rdata = rrs[0].get_rdata();
            if let Rdata::A(ip) = rdata {
                assert_eq!(ip.get_address(), IpAddr::from([127, 0, 0, 1]));
            } else {
                panic!("Error parsing response");
            }
        } else {
            if let Err(ClientError::Refused("The name server refuses to perform the specified operation for policy reasons.")) = result_vec_rr {
                assert!(true);
            }
            else {
                panic!("Error parsing response");
            }
        }

    }

    //TODO: probar diferentes qtype
    #[tokio::test]
    async fn qtypes_a() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::A,
                Qclass::IN,
                0,
                false,
                1);
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let result_vec_rr = resolver.parse_dns_msg(Ok(dns_response));

        if let Ok(rrs) = result_vec_rr {
            let rdata = rrs[0].get_rdata();
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
    async fn qtypes_ns() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::NS,
                Qclass::IN,
                0,
                false,
                1);
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let result_vec_rr = resolver.parse_dns_msg(Ok(dns_response));

        if let Ok(rrs) = result_vec_rr {
            let rdata = rrs[0].get_rdata();
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
    async fn qtypes_cname() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::CNAME,
                Qclass::IN,
                0,
                false,
                1);
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let result_vec_rr = resolver.parse_dns_msg(Ok(dns_response));

        if let Ok(rrs) = result_vec_rr {
            let rdata = rrs[0].get_rdata();
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
    async fn qtypes_soa() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::SOA,
                Qclass::IN,
                0,
                false,
                1);
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let result_vec_rr = resolver.parse_dns_msg(Ok(dns_response));

        if let Ok(rrs) = result_vec_rr {
            let rdata = rrs[0].get_rdata();
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
    async fn qtypes_ptr() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::PTR,
                Qclass::IN,
                0,
                false,
                1);
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let result_vec_rr = resolver.parse_dns_msg(Ok(dns_response));

        if let Ok(rrs) = result_vec_rr {
            let rdata = rrs[0].get_rdata();
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
    async fn qtypes_hinfo() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::HINFO,
                Qclass::IN,
                0,
                false,
                1);
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let result_vec_rr = resolver.parse_dns_msg(Ok(dns_response));

        if let Ok(rrs) = result_vec_rr {
            let rdata = rrs[0].get_rdata();
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
    async fn qtypes_minfo() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::MINFO,
                Qclass::IN,
                0,
                false,
                1);
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let result_vec_rr = resolver.parse_dns_msg(Ok(dns_response));

        if let Ok(rrs) = result_vec_rr {
            let rdata = rrs[0].get_rdata();
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
    async fn qtypes_wks() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::WKS,
                Qclass::IN,
                0,
                false,
                1);
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let result_vec_rr = resolver.parse_dns_msg(Ok(dns_response));

        if let Ok(rrs) = result_vec_rr {
            let rdata = rrs[0].get_rdata();
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
    async fn qtypes_txt() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::TXT,
                Qclass::IN,
                0,
                false,
                1);
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let result_vec_rr = resolver.parse_dns_msg(Ok(dns_response));

        if let Ok(rrs) = result_vec_rr {
            let rdata = rrs[0].get_rdata();
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
    async fn qtypes_dname() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::DNAME,
                Qclass::IN,
                0,
                false,
                1);
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let result_vec_rr = resolver.parse_dns_msg(Ok(dns_response));

        if let Ok(rrs) = result_vec_rr {
            let rdata = rrs[0].get_rdata();
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
    async fn qtypes_any() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response 
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::ANY,
                Qclass::IN,
                0,
                false,
                1);
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let result_vec_rr = resolver.parse_dns_msg(Ok(dns_response));

        if let Ok(rrs) = result_vec_rr {
            let rdata = rrs[0].get_rdata();
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
    async fn qtypes_tsig() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::TSIG,
                Qclass::IN,
                0,
                false,
                1);
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let result_vec_rr = resolver.parse_dns_msg(Ok(dns_response));

        if let Ok(rrs) = result_vec_rr {
            let rdata = rrs[0].get_rdata();
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
    async fn qtypes_axfr() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::AXFR,
                Qclass::IN,
                0,
                false,
                1);
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let result_vec_rr = resolver.parse_dns_msg(Ok(dns_response));

        if let Ok(rrs) = result_vec_rr {
            let rdata = rrs[0].get_rdata();
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
    async fn qtypes_mailb() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::MAILB,
                Qclass::IN,
                0,
                false,
                1);
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let result_vec_rr = resolver.parse_dns_msg(Ok(dns_response));

        if let Ok(rrs) = result_vec_rr {
            let rdata = rrs[0].get_rdata();
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
    async fn qtypes_maila() {
        let resolver = AsyncResolver::new(ResolverConfig::default());

        // Create a new dns response
        let mut answer: Vec<ResourceRecord> = Vec::new();
        let mut a_rdata = ARdata::new();
        a_rdata.set_address(IpAddr::from([127, 0, 0, 1]));
        let rdata = Rdata::A(a_rdata);
        let resource_record = ResourceRecord::new(rdata);
        answer.push(resource_record);

        let mut dns_response =
            DnsMessage::new_query_message(
                DomainName::new_from_string("example.com".to_string()),
                Qtype::MAILA,
                Qclass::IN,
                0,
                false,
                1);
        dns_response.set_answer(answer);
        let mut header = dns_response.get_header();
        header.set_qr(true);
        dns_response.set_header(header);
        let result_vec_rr = resolver.parse_dns_msg(Ok(dns_response));

        if let Ok(rrs) = result_vec_rr {
            let rdata = rrs[0].get_rdata();
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
        let mut resolver = AsyncResolver::new(ResolverConfig::default());
        resolver.cache.set_max_size(1);

        let domain_name = DomainName::new_from_string("example.com".to_string());
    
        // Create truncated dns response
        let mut dns_response =
            DnsMessage::new_query_message(
                domain_name,
                Qtype::A,
                Qclass::IN,
                0,
                false,
                1);
        let mut truncated_header = dns_response.get_header();
        truncated_header.set_tc(true);
        dns_response.set_header(truncated_header);

        resolver.store_data_cache(dns_response);

        assert_eq!(resolver.get_cache().get_size(), 0);
    }    

}