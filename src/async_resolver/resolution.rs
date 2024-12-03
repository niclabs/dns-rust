use crate::client::client_error::ClientError;
use crate::message::rcode::Rcode;
use crate::message::DnsMessage;
use crate::client::client_connection::ClientConnection;
use super::lookup_response::LookupResponse;
use super::resolver_error::ResolverError;
use super::server_info::ServerInfo;
use super::state_block::StateBlock;
use std::sync::{Mutex,Arc};
use std::time::Instant;
use crate::client::client_connection::ConnectionProtocol;
use crate::async_resolver::config::ResolverConfig;

/// Struct that represents the execution of a lookup.
/// 
/// The principal purpose of this struct is to transmit a single query
/// until a proper response is received. 
/// 
/// The result of the lookup is stored in the `response_msg` field.
/// First it is initialized with an empty `DnsMessage` and then it is updated
/// with the response of the query.
/// 
/// The lookup is done asynchronously after calling the asynchronoyus 
/// `run` method.
pub struct Resolution {
    query: DnsMessage,
    /// Resolver configuration.
    config: ResolverConfig,
    /// Reference to the response of the query.
    response_msg: Arc<std::sync::Mutex<Result<DnsMessage, ResolverError>>>,
    /// Contains information about the state of a pending request.
    state_block: StateBlock
}
    
impl Resolution {

    /// Creates a new `Resolution` with the given configuration.
    pub fn new(
        query: DnsMessage,
        config: ResolverConfig,
        
    ) -> Self {
        let request_global_limit = config.get_global_retransmission_limit();
        let server_transmission_limit = 1; // TODO: add to config
        let servers = config.get_name_servers();
        Self { 
            query: query,
            config: config,
            response_msg: Arc::new(Mutex::new(Err(ResolverError::EmptyQuery))), 
            state_block: StateBlock::new(request_global_limit, server_transmission_limit, servers)
        }
    }

    /// Executes the lookup of the Domain Name asynchronously.
    /// 
    /// This function performs the lookup of the requested records asynchronously.
    /// It returns a `LookupResponse` with the response of the query.
    pub async fn run(
        &mut self,
    ) -> Result<LookupResponse, ResolverError> {
        let config: &ResolverConfig = &self.config;
        let max_interval: u64 = config.get_max_retry_interval_seconds(); 
        let initial_rto = 1.0;
        let mut rto = initial_rto;
        let mut srtt = rto;
        let mut rttvar = rto/2.0;

        let mut timeout_duration = tokio::time::Duration::from_secs_f64(rto);
        let mut lookup_response_result: Result<LookupResponse, ResolverError> = Err(ResolverError::EmptyQuery);
        let start = Instant::now();
        let mut end = start;

        // Incrementar end hasta que cambie
        while end == start {
            end = Instant::now();
        }

        let granularity = end.duration_since(start).as_secs_f64() + end.duration_since(start).subsec_nanos() as f64 * 1e-9;

        // The resolver cycles through servers and at the end of a cycle, backs off 
        // the timeout exponentially.

        // TODO: check if the correct number of retransmissions is being done
        'global_cycle: while let Ok(_) = self.state_block.decrement_work_counter() {
            let initial_server_index = self.state_block.get_current_server_index();
            loop {
                let server_entry = self.state_block.get_current_server_entry();
                if !server_entry.get_info().is_active() { 
                    self.state_block.increment_current_server_index();
                    if self.state_block.get_current_server_index() == initial_server_index {
                        break; 
                    }
                    continue; 
                }

                // start timer
                let start = Instant::now();

                let server_info_arc = server_entry.get_info().clone();
                lookup_response_result = self.transmit_query_to_server(
                    server_info_arc, 
                    timeout_duration
                ).await;

                self.state_block.get_current_server_entry().decrement_work_counter()?; 

                // end timer
                let end = Instant::now();

                let rtt = end.duration_since(start);
                rttvar = (1.0 - 0.25) * rttvar + 0.25 * (rtt.as_secs_f64() - srtt).abs();
                srtt = (1.0 - 0.125) * srtt + 0.125 * rtt.as_secs_f64();
                rto = srtt + granularity.max(4.0 * rttvar) ;
                timeout_duration = tokio::time::Duration::from_secs_f64(rto);   

                if self.received_appropriate_response() { break 'global_cycle }

                self.state_block.increment_current_server_index();
                if self.state_block.get_current_server_index() == initial_server_index {
                    break;
                }
            }
            
            // Exponencial backoff
            rto = (rto * 2.0).min(max_interval as f64);
            timeout_duration = tokio::time::Duration::from_secs_f64(rto);
            tokio::time::sleep(timeout_duration).await; // TODO: sleep is probably not the best way to do this
        }
        return lookup_response_result;
    }

    /// Checks if an appropiate answer was received.
    /// 
    /// [RFC 2136]: https://datatracker.ietf.org/doc/html/rfc2136#section-4.5
    /// 
    /// 4.5. If the requestor receives a response, and the response has an
    //  RCODE other than SERVFAIL or NOTIMP, then the requestor returns an
    //  appropriate response to its caller.
    pub fn received_appropriate_response(&self) -> bool {
        let response_arc = self.response_msg.lock().unwrap();
        if let Ok(dns_msg) = response_arc.as_ref() {
            match dns_msg.get_header().get_rcode().into() {
                Rcode::SERVFAIL => return false,
                Rcode::NOTIMP => return false,
                _ => return true,
            }
        }
        false
    }

    /// Perfoms the lookup of a Domain Name acting as a Stub Resolver.
    /// 
    /// This function performs the lookup of the requested records asynchronously. 
    /// After creating the query with the given parameters, the function sends it to 
    /// the name servers specified in the configuration. 
    /// 
    /// When a response is received, the function performs the parsing of the response 
    /// to a `DnsMessage`. After the response is checked, the function updates the 
    /// value of the reference in `response_arc` with the parsed response.
    /// 
    /// [RFC 1034]: https://datatracker.ietf.org/doc/html/rfc1034#section-5.3.1
    /// 
    /// 5.3.1. Stub resolvers
    /// 
    /// One option for implementing a resolver is to move the resolution
    /// function out of the local machine and into a name server which supports
    /// recursive queries.  This can provide an easy method of providing domain
    /// service in a PC which lacks the resources to perform the resolver
    /// function, or can centralize the cache for a whole local network or
    /// organization.
    /// 
    /// All that the remaining stub needs is a list of name server addresses
    /// that will perform the recursive requests.  This type of resolver
    /// presumably needs the information in a configuration file, since it
    /// probably lacks the sophistication to locate it in the domain database.
    /// The user also needs to verify that the listed servers will perform the
    /// recursive service; a name server is free to refuse to perform recursive
    /// services for any or all clients.  The user should consult the local
    /// system administrator to find name servers willing to perform the
    /// service.
    ///
    /// This type of service suffers from some drawbacks.  Since the recursive
    /// requests may take an arbitrary amount of time to perform, the stub may
    /// have difficulty optimizing retransmission intervals to deal with both
    /// lost UDP packets and dead servers; the name server can be easily
    /// overloaded by too zealous a stub if it interprets retransmissions as new
    /// requests.  Use of TCP may be an answer, but TCP may well place burdens
    /// on the host's capabilities which are similar to those of a real
    /// resolver.
    /// 
    /// # Example
    /// ```
    /// let domain_name = DomainName::new_from_string("example.com".to_string());
    /// let cache = DnsCache::new();
    /// let waker = None;
    /// let query =  Arc::new(Mutex::new(future::err(ResolverError::EmptyQuery).boxed()));
    ///
    /// let google_server:IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
    /// let timeout: Duration = Duration::from_secs(20);
    ///
    /// let conn_udp:ClientUDPConnection = ClientUDPConnection::new(google_server, timeout);
    /// let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(google_server, timeout);
    /// 
    /// let config = ResolverConfig::default();
    /// let record_type = Rrtype::A;
    /// 
    /// let name_servers = vec![(conn_udp,conn_tcp)];
    /// let response = transmit_query_to_server(domain_name,record_type, cache, name_servers, waker,query,config).await.unwrap();
    /// ```
    pub async fn transmit_query_to_server(
        &self,
        server_info: Arc<ServerInfo>,
        timeout_duration: tokio::time::Duration
    ) -> Result<LookupResponse, ResolverError>  {
        let response_arc=  self.response_msg.clone();
        let protocol = self.config.get_protocol();
        let mut dns_msg_result: Result<DnsMessage, ResolverError>;
        {
            // Guard reference to modify the response
            let mut response_guard = response_arc.lock().unwrap(); // TODO: add error handling
            let send_future = send_query_by_protocol(
                timeout_duration,
                &self.query,
                protocol,
                server_info.clone()
            );
            dns_msg_result = tokio::time::timeout(timeout_duration, send_future)
                .await
                .unwrap_or_else(
                    |_| {Err(ResolverError::Message("Execute Strategy Timeout Error".into()))}
                );  
            *response_guard = dns_msg_result.clone();
        }
        if self.received_appropriate_response() {
            return dns_msg_result.and_then(
                |dns_msg| Ok(LookupResponse::new(dns_msg))
            )
        }
        if let ConnectionProtocol::UDP = protocol {
            let tcp_protocol = ConnectionProtocol::TCP;
            let send_future = send_query_by_protocol(
                timeout_duration,
                &self.query,
                tcp_protocol,
                server_info
            );
            tokio::time::sleep(timeout_duration).await;
            dns_msg_result = tokio::time::timeout(timeout_duration, send_future)
                .await
                .unwrap_or_else(
                    |_| {Err(ResolverError::Message("Execute Strategy Timeout Error".into()))}
                ); 
            let mut response_guard = response_arc.lock().unwrap();
            *response_guard = dns_msg_result.clone();
        }
        dns_msg_result.and_then(
            |dns_msg| Ok(LookupResponse::new(dns_msg))
        )
    }
}

///  Sends a DNS query to a resolver using the specified connection protocol.
/// 
///  This function takes a DNS query, a result containing a DNS message,
///  and connection information. Depending on the specified protocol (UDP or TCP),
///  it sends the query using the corresponding connection and updates the result
///  with the parsed response.
async fn send_query_by_protocol(
    timeout: tokio::time::Duration,
    query: &DnsMessage,
    protocol: ConnectionProtocol,
    server_info:  Arc<ServerInfo>,
) ->  Result<DnsMessage, ResolverError> {
    let query_id = query.get_query_id();
    let dns_query = query.clone();
    let dns_msg_result;
    match protocol{ 
        ConnectionProtocol::UDP => {
            let mut udp_connection = server_info.get_udp_connection().clone();
            udp_connection.set_timeout(timeout);
            let response_result = udp_connection.send(dns_query).await;
            dns_msg_result = parse_response(response_result, query_id);
        }
        ConnectionProtocol::TCP => {
            let mut tcp_connection = server_info.get_tcp_connection().clone();
            tcp_connection.set_timeout(timeout);
            let response_result = tcp_connection.send(dns_query).await;
            dns_msg_result = parse_response(response_result, query_id);
        }
        _ => {dns_msg_result = Err(ResolverError::Message("Invalid Protocol".into()))}, // TODO: specific add error handling
    }; 
    dns_msg_result
}

/// Parse the received response datagram to a `DnsMessage`.
/// 
/// [RFC 1035]: https://datatracker.ietf.org/doc/html/rfc1035#section-7.3
/// 
/// 7.3. Processing responses
/// The first step in processing arriving response datagrams is to parse the
/// response.  This procedure should include:
/// 
///    - Check the header for reasonableness.  Discard datagrams which
///      are queries when responses are expected.
/// 
///    - Parse the sections of the message, and insure that all RRs are
///      correctly formatted.
/// 
///    - As an optional step, check the TTLs of arriving data looking
///      for RRs with excessively long TTLs.  If a RR has an
///      excessively long TTL, say greater than 1 week, either discard
///      the whole response, or limit all TTLs in the response to 1
///      week.
fn parse_response(response_result: Result<Vec<u8>, ClientError>, query_id:u16) -> Result<DnsMessage, ResolverError> {
    let dns_msg = response_result.map_err(Into::into)
        .and_then(|response_message| {
            DnsMessage::from_bytes(&response_message)
                .map_err(|_| ResolverError::Parse("The name server was unable to interpret the query.".to_string()))
        })?;
    let header = dns_msg.get_header();
    
    // check Header
    header.format_check()
    .map_err(|e| ResolverError::Parse(format!("Error formated Header: {}", e)))?;

    // Check ID
    if dns_msg.get_query_id() != query_id {
        return  Err(ResolverError::Parse("Error expected ID from query".to_string()))
    }

    if header.get_qr() {
        return Ok(dns_msg);
    }
    Err(ResolverError::Parse("Message is a query. A response was expected.".to_string()))
}

#[cfg(test)]
mod async_resolver_test {
    use crate::async_resolver::server_info;
    use crate::client::tcp_connection::ClientTCPConnection;
    use crate::client::udp_connection::ClientUDPConnection;
    use crate::message;
    use crate::message::rclass::Rclass;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;
    use crate::message::rrtype::Rrtype;
    use crate::{ domain_name::DomainName, dns_cache::DnsCache};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::str::FromStr;
    use std::time::Duration;
    use std::num::NonZeroUsize;
    use super::*;
   
    #[test]
    fn lookup() {

        let domain_name = DomainName::new_from_string("example.com".to_string());
        let domain_name_cache = DomainName::new_from_string("test.com".to_string());
        let config: ResolverConfig = ResolverConfig::default();
        
        let mut cache: DnsCache = DnsCache::new(NonZeroUsize::new(20));

        let record_type = Rrtype::A;
        let record_class = Rclass::IN;

        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        cache.add(domain_name_cache, resource_record, Some(record_type), record_class, None);

        let query = message::create_recursive_query(domain_name, record_type, record_class);

        let lookup_future = Resolution::new(
            query,
            config,
        );

        assert_eq!(lookup_future.query.get_question().get_qname(), DomainName::new_from_string("example.com".to_string()));
        assert_eq!(lookup_future.config.get_addr(),SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5333));
    }
     
    #[tokio::test]
    async fn transmit_query_to_server_a_response() {
        println!("transmit_query_to_server_a_response   inittt");
        let domain_name: DomainName = DomainName::new_from_string("example.com".to_string());

        let google_server:IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let timeout: Duration = Duration::from_secs(20);

        let conn_udp:ClientUDPConnection = ClientUDPConnection::new_default(google_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new_default(google_server, timeout);

        let config = ResolverConfig::default();
        let record_type = Rrtype::A;
        let record_class = Rclass::IN;
        let server_info = server_info::ServerInfo::new_with_ip(google_server,conn_udp, conn_tcp);
        let name_servers = vec![Arc::new(server_info)];

        // let response_arc: Arc<Mutex<Result<DnsMessage, ResolverError>>> = Arc::new(Mutex::new(Err(ResolverError::EmptyQuery)));

        let lookup_strategy = Resolution::new(
            message::create_recursive_query(domain_name, record_type, record_class),
            config,
        );

        let response = lookup_strategy.transmit_query_to_server(
            name_servers.get(0).unwrap().clone(), // CHANGED: ADEED CLONE
            timeout
        ).await;

        // let response = transmit_query_to_server(
        //     domain_name,
        //     record_type,
        //     record_class, 
        //     name_servers.get(0).unwrap(), 
        //     &config,
        //     response_arc,
        //     timeout
        // ).await;

        println!("response {:?}", response);

        assert_eq!(response
            .clone()
            .unwrap()
            .to_dns_msg()
            .get_header()
            .get_qr(),
            true);
        assert_ne!(response
            .unwrap()
            .to_dns_msg()
            .get_answer()
            .len(), 
            0);
    }   

    #[tokio::test]
    async fn transmit_query_to_server_ns_response() {
        let domain_name = DomainName::new_from_string("example.com".to_string());
    
        // Create vect of name servers
        let google_server:IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let timeout: Duration = Duration::from_secs(20);

        let conn_udp:ClientUDPConnection = ClientUDPConnection::new_default(google_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new_default(google_server, timeout);

        let server_info = server_info::ServerInfo::new_with_ip(google_server,conn_udp, conn_tcp);
        let config = ResolverConfig::default();
        let record_type = Rrtype::NS;
        let record_class = Rclass::IN;
        let name_servers = vec![Arc::new(server_info)];
        // let response_arc: Arc<Mutex<Result<DnsMessage, ResolverError>>> = Arc::new(Mutex::new(Err(ResolverError::EmptyQuery)));

        let lookup_strategy = Resolution::new(
            message::create_recursive_query(domain_name, record_type, record_class),
            config,
        );

        let response = lookup_strategy.transmit_query_to_server(
            name_servers.get(0).unwrap().clone(), // CHANGED: ADEED CLONE
            timeout
        ).await.unwrap();

        // let response = transmit_query_to_server(
        //     domain_name, 
        //     record_type, 
        //     record_class,
        //     name_servers.get(0).unwrap(), 
        //     &config,
        //     response_arc,
        //     timeout
        // ).await.unwrap();

        assert_eq!(response
            .to_dns_msg()
            .get_header()
            .get_qr(),true);
        // This changes depending on the server we're using
        assert!(response
            .to_dns_msg()
            .get_header().get_ancount() >= 1);
    } 

    #[tokio::test]
    async fn transmit_query_to_server_ch_response() {
        let domain_name = DomainName::new_from_string("example.com".to_string());

        let google_server:IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let timeout: Duration = Duration::from_secs(20);

        let conn_udp:ClientUDPConnection = ClientUDPConnection::new_default(google_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new_default(google_server, timeout);
        let server_info = server_info::ServerInfo::new_with_ip(google_server,conn_udp, conn_tcp);
        let config = ResolverConfig::default();
        let record_type = Rrtype::A;
        let record_class = Rclass::CH;
        let name_servers = vec![Arc::new(server_info)];
        // let response_arc: Arc<Mutex<Result<DnsMessage, ResolverError>>> = Arc::new(Mutex::new(Err(ResolverError::EmptyQuery)));

        let lookup_strategy = Resolution::new(
            message::create_recursive_query(domain_name, record_type, record_class),
            config,
        );

        let response = lookup_strategy.transmit_query_to_server(
            name_servers.get(0).unwrap().clone(), // CHANGED: ADEED CLONE
            timeout
        ).await.unwrap();

        // let response = transmit_query_to_server(
        //     domain_name,
        //     record_type,
        //     record_class, 
        //     name_servers.get(0).unwrap(),
        //     &config,
        //     response_arc,
        //     timeout
        // ).await.unwrap();


        assert_eq!(response
            .to_dns_msg()
            .get_header()
            .get_qr(),true);
        assert_eq!(response
            .to_dns_msg()
            .get_answer()
            .len(),0);
    } 

    #[tokio::test] // TODO: finish up test
    async fn lookup_ip_cache_test() {
        let domain_name = DomainName::new_from_string("example.com".to_string());
        let record_type = Rrtype::A;
        let record_class = Rclass::IN;
        let config: ResolverConfig = ResolverConfig::default();
        let addr = IpAddr::from_str("93.184.216.34").unwrap();
        let a_rdata = ARdata::new_from_addr(addr);
        let rdata = Rdata::A(a_rdata);
        let rr = ResourceRecord::new(rdata);

        let mut cache = DnsCache::new(NonZeroUsize::new(1));
        
        cache.add(domain_name.clone(), rr, Some(record_type), record_class, None);

        // let query_sate: Arc<Mutex<Result<DnsMessage, ResolverError>>> = Arc::new(Mutex::new(Err(ResolverError::EmptyQuery)));

        // let _response_future = transmit_query_to_server(
        //     domain_name, 
        //     record_type, 
        //     record_class,
        //     config.get_name_servers().get(0).unwrap(),
        //     &config, 
        //     query_sate,
        //     tokio::time::Duration::from_secs(3)).await;

        let mut lookup_strategy = Resolution::new(
            message::create_recursive_query(domain_name, record_type, record_class),
            config,
        );

        let _response_future = lookup_strategy.run().await;
    }  
    


    #[test]
    #[ignore] //FIXME:
    fn parse_response_ok() {
        let bytes: [u8; 50] = [
            //test passes with this one
            0b00100100, 0b10010101, 0b10010010, 0b00000000, 0, 1, 0b00000000, 1, 0, 0, 0, 0, 4, 116,
            101, 115, 116, 3, 99, 111, 109, 0, 0, 16, 0, 1, 3, 100, 99, 99, 2, 99, 108, 0, 0, 16, 0,
            1, 0, 0, 0b00010110, 0b00001010, 0, 6, 5, 104, 101, 108, 108, 111,
        ];
        let query_id = 0b00100100;
        let response_result: Result<Vec<u8>, ClientError> = Ok(bytes.to_vec());
        let response_dns_msg = parse_response(response_result,query_id);
        println!("[###############] {:?}",response_dns_msg);
        assert!(response_dns_msg.is_ok());
        if let Ok(dns_msg) = response_dns_msg {
            assert_eq!(dns_msg.get_header().get_qr(), true); // response (1)
            assert_eq!(dns_msg.get_header().get_ancount(), 1);
            assert_eq!(dns_msg.get_header().get_rcode(), Rcode::NOERROR);
            println!("The message is: {:?}", dns_msg);
        }
    }

    #[test]
    #[ignore]
    fn parse_response_query() {
        let bytes: [u8; 50] = [
            //test passes with this one
            0b10100101, 0b10010101, 0b00010010, 0b00000000, 0, 1, 0b00000000, 1, 0, 0, 0, 0, 4, 116,
            101, 115, 116, 3, 99, 111, 109, 0, 0, 16, 0, 1, 3, 100, 99, 99, 2, 99, 108, 0, 0, 16, 0,
            1, 0, 0, 0b00010110, 0b00001010, 0, 6, 5, 104, 101, 108, 108, 111,
        ];
        let query_id = 0b10100101;
        let response_result: Result<Vec<u8>, ClientError> = Ok(bytes.to_vec());
        let response_dns_msg = parse_response(response_result,query_id);
        let err_msg = "Message is a query. A response was expected.".to_string();
        if let Err(ResolverError::Parse(err)) = response_dns_msg {
            assert_eq!(err, err_msg)
        } else {
            assert!(false);
        }
    }
    
    #[test]
    fn parse_error() {
        let bytes: [u8; 50] = [
            //test passes with this one
            0b10100101, 0b10010101, 0b00101010, 0b00001010, 0, 1, 0b00000000, 1, 0, 0, 0, 0, 4, 116,
            101, 115, 116, 3, 99, 111, 109, 0, 0, 16, 0, 1, 3, 100, 99, 99, 2, 99, 45, 0, 0, 16, 0,
            1, 0, 0, 0b00010110, 0b00001010, 0, 6, 5, 104, 101, 108, 108, 111,
        ];
        let query_id = 0b10100101;
        let response_result: Result<Vec<u8>, ClientError> = Ok(bytes.to_vec());
        let response_dns_msg = parse_response(response_result,query_id);
        let err_msg = "The name server was unable to interpret the query.".to_string();
        if let Err(ResolverError::Parse(err)) = response_dns_msg {
            assert_eq!(err, err_msg)
        } else {
            assert!(false);
        }
    }

    #[test]
    fn parse_error_domain_name() {
        let bytes: [u8; 50] = [
            //test passes with this one
            0b10100101, 0b10010101, 0b11111111, 0b11111111, 0, 1, 0b00000000, 1, 0, 0, 0, 0, 4, 116,
            101, 115, 64, 3, 99, 111, 109, 0, 0, 16, 0, 1, 3, 100, 99, 99, 2, 99, 108, 0, 0, 16, 0,
            1, 0, 0, 0b00010110, 0b00001010, 0, 6, 5, 104, 101, 108, 108, 111,
        ];
        let query_id = 0b10100101;
        let response_result: Result<Vec<u8>, ClientError> = Ok(bytes.to_vec());
        let response_dns_msg = parse_response(response_result,query_id);
        let err_msg = "The name server was unable to interpret the query.".to_string();

        if let Err(ResolverError::Parse(err)) = response_dns_msg {
            assert_eq!(err, err_msg)
        } else {
            assert!(false);
        }
    }
  
    // TODO: test empty response run
   
    // TODO: test run max rieswith max of 0 

}



    
