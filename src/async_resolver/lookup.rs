use crate::client::client_error::ClientError;
use crate::domain_name::DomainName;
use crate::message::DnsMessage;
use crate::message::header::Header;
use crate::client::client_connection::ClientConnection;
use crate::message::class_qclass::Qclass;
use crate::message::type_qtype::Qtype;
use rand::{thread_rng, Rng};
use tokio::net::tcp;
use super::lookup_response::LookupResponse;
use super::resolver_error::ResolverError;
use super::server_info::ServerInfo;
use core::time;
use std::sync::{Mutex,Arc};
use crate::client::client_connection::ConnectionProtocol;
use crate::async_resolver::config::ResolverConfig;
use crate::client::udp_connection::ClientUDPConnection;
use crate::client::tcp_connection::ClientTCPConnection;
use tokio::time::timeout;
use std::num::NonZeroUsize;
use crate::client::udp_connection;

/// Struct that represents the execution of a lookup.
/// 
/// The principal purpose of this struct is to transmit a single query
/// until a proper response is received. 
/// 
/// The result of the lookup is stored in the `query_answer` field.
/// First it is initialized with an empty `DnsMessage` and then it is updated
/// with the response of the query.
/// 
/// The lookup is done asynchronously after calling the asynchronoyus 
/// `lookup_run` method.
pub struct LookupStrategy {
    /// Domain Name associated with the query.
    name: DomainName,
    /// Qtype of search query
    record_type: Qtype,
    /// Qclass of the search query
    record_class: Qclass,
    /// Resolver configuration.
    config: ResolverConfig,
    /// Reference to the response of the query.
    pub query_answer: Arc<std::sync::Mutex<Result<DnsMessage, ResolverError>>>,
}
    
impl LookupStrategy {

    /// Creates a new `LookupStrategy` with the given configuration.
    pub fn new(
        name: DomainName,
        qtype: Qtype,
        qclass: Qclass,
        config: ResolverConfig,
        
    ) -> Self {
        Self { 
            name: name,
            record_type: qtype,
            record_class: qclass,
            config: config,
            query_answer: Arc::new(Mutex::new(Err(ResolverError::EmptyQuery))), 
        }
    }

    /// Executes the lookup of the Domain Name asynchronously.
    /// 
    /// This function performs the lookup of the requested records asynchronously.
    /// It returns a `LookupResponse` with the response of the query.
    /// 
    /// TODO: make lookup_run specific to a single SERVER, it receives the server where it should be quering 
    pub async fn lookup_run(
        &mut self,
        timeout: tokio::time::Duration,           
    ) -> Result<LookupResponse, ResolverError> {
        let config = self.config.clone();
        
        // let upper_limit_of_retransmission = self.config.get_retry();
        let upper_limit_of_retransmission = 4;
        // let number_of_server_to_query = self.config.get_name_servers().len() as u64;
        let max_timeout = 30;  

        // Start interval used by The Berkeley stub-resolver
        // let start_interval = max(4, 5/number_of_server_to_query).into();
        let start_interval = 1;
        let mut interval = start_interval;
            
        // Retransmission loop for a single server
        let mut iter = 0..upper_limit_of_retransmission;

        let mut timeout_interval = tokio::time::Duration::from_secs(interval);
        
        let mut lookup_result = Err(ResolverError::EmptyQuery);

        // The resolver cycles through servers and at the end of a cycle, backs off 
        // the time out exponentially.
        'cycle: while let Some(_retransmission) = iter.next() {

            // Loop between servers
            let servers_to_query = config.get_name_servers();  
            let mut server_iter = servers_to_query.iter();
            while let Some(server) = server_iter.next() {
                lookup_result = self.transmit_query_to_server(
                    server, 
                    timeout_interval
                ).await;

                if self.received_appropriate_response() {break 'cycle}
            }

            // Exponencial backoff
            if interval < max_timeout {
                interval = interval*2;
            }
            timeout_interval = tokio::time::Duration::from_secs(interval);
            tokio::time::sleep(timeout_interval).await;
        }
        return lookup_result;
    }

    /// Checks if an appropiate answer was received.
    /// 
    /// [RFC 2136]: https://datatracker.ietf.org/doc/html/rfc2136#section-4.5
    /// 
    /// 4.5. If the requestor receives a response, and the response has an
    //  RCODE other than SERVFAIL or NOTIMP, then the requestor returns an
    //  appropriate response to its caller.
    pub fn received_appropriate_response(&self) -> bool {
        let response_arc = self.query_answer.lock().unwrap();

        if let Ok(dns_msg) = response_arc.as_ref() {
            match dns_msg.get_header().get_rcode() {
                // SERVFAIL
                2 => return false,
                // NOTIMP
                4 => return false,
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
    /// let record_type = Qtype::A;
    /// 
    /// let name_servers = vec![(conn_udp,conn_tcp)];
    /// let response = transmit_query_to_server(domain_name,record_type, cache, name_servers, waker,query,config).await.unwrap();
    /// ```
    pub async fn transmit_query_to_server(
        &self,
        name_server: &ServerInfo,
        timeout: tokio::time::Duration,
    ) -> Result<LookupResponse, ResolverError>  {
        let response_arc=  self.query_answer.clone();
        let name = self.name.clone();
        let record_type = self.record_type;
        let record_class = self.record_class;
        let protocol = self.config.get_protocol();

        let new_query = create_lookup_query(name, record_type, record_class);
        let response = create_response_from_query(&new_query);

        let mut result_dns_msg: Result<DnsMessage, ResolverError> = Ok(response.clone());

        // Get guard to modify the response
        {
        let mut response_guard = response_arc.lock().unwrap();
        result_dns_msg = tokio::time::timeout(
            timeout, 
            send_query_by_protocol(
                timeout,
                protocol,
                new_query.clone(),
                result_dns_msg.clone(),
                name_server,
            )).await
            .unwrap_or_else(|_| {Err(ResolverError::Message("Execute Strategy Timeout Error".into()))
        });  
        *response_guard = result_dns_msg.clone();
        }

        if !self.received_appropriate_response() {
            if let ConnectionProtocol::UDP = protocol {
                tokio::time::sleep(timeout).await;
                result_dns_msg = tokio::time::timeout(
                    timeout, 
                    send_query_by_protocol(
                        timeout,
                        protocol,
                        new_query.clone(),
                        result_dns_msg.clone(),
                        name_server,
                    )).await
                    .unwrap_or_else(|_| {Err(ResolverError::Message("Execute Strategy Timeout Error".into()))
                }); 
                let mut response_guard = response_arc.lock().unwrap();
                *response_guard = result_dns_msg.clone();
            }
        }

        result_dns_msg.and_then(|dns_msg| Ok(LookupResponse::new(dns_msg)))
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
    protocol: ConnectionProtocol,
    query: DnsMessage,
    mut result_dns_msg: Result<DnsMessage, ResolverError>, 
    connections:  &ServerInfo,
)
->  Result<DnsMessage, ResolverError>{
    let query_id = query.get_query_id();

    match protocol{ 
        ConnectionProtocol::UDP => {
            let mut udp_connection = connections.get_udp_connection().clone();
            udp_connection.set_timeout(timeout);
            let result_response = udp_connection.send(query.clone()).await;
            result_dns_msg = parse_response(result_response,query_id);
        }
        ConnectionProtocol::TCP => {
            let mut tcp_connection = connections.get_tcp_connection().clone();
            tcp_connection.set_timeout(timeout);
            let result_response = tcp_connection.send(query.clone()).await;
            result_dns_msg = parse_response(result_response,query_id);
        }
        _ => {},
    }; 
    
    result_dns_msg
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

fn create_lookup_query(
    name: DomainName,
    record_type: Qtype,
    record_class: Qclass,
) -> DnsMessage {
    // Create random generator
    let mut rng = thread_rng();

    // Create query id
    let query_id: u16 = rng.gen();

    // Create query
    let query = DnsMessage::new_query_message(
        name.clone(),
        record_type,
        record_class,
        0,
        true,
        query_id
    );

    return query;
}

fn create_response_from_query(
    query: &DnsMessage,
) -> DnsMessage {

    // Create Server failure query
    let mut response = query.clone();
    let mut new_header: Header = response.get_header();
    new_header.set_rcode(2);
    new_header.set_qr(true);
    response.set_header(new_header);

    return response;
}

#[cfg(test)]
mod async_resolver_test {
    use crate::async_resolver::server_info;
    // use tokio::runtime::Runtime;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;
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

        let record_type = Qtype::A;
        let record_class = Qclass::IN;

        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        cache.add(domain_name_cache, resource_record, record_type, record_class, None);

        

        let lookup_future = LookupStrategy::new(
            domain_name,
            record_type,
            record_class,
            config,
        );

        assert_eq!(lookup_future.name, DomainName::new_from_string("example.com".to_string()));
        assert_eq!(lookup_future.config.get_addr(),SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5333));
    }
     
    #[tokio::test]
    async fn transmit_query_to_server_a_response() {
        let domain_name: DomainName = DomainName::new_from_string("example.com".to_string());

        let google_server:IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let timeout: Duration = Duration::from_secs(20);

        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(google_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(google_server, timeout);

        let config = ResolverConfig::default();
        let record_type = Qtype::A;
        let record_class = Qclass::IN;
        let server_info = server_info::ServerInfo::new_with_ip(google_server,conn_udp, conn_tcp);
        let name_servers = vec![server_info];
        // let response_arc: Arc<Mutex<Result<DnsMessage, ResolverError>>> = Arc::new(Mutex::new(Err(ResolverError::EmptyQuery)));

        let lookup_strategy = LookupStrategy::new(
            domain_name,
            record_type,
            record_class,
            config,
        );

        let response = lookup_strategy.transmit_query_to_server(
            name_servers.get(0).unwrap(),
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

        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(google_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(google_server, timeout);

        let server_info = server_info::ServerInfo::new_with_ip(google_server,conn_udp, conn_tcp);
        let config = ResolverConfig::default();
        let record_type = Qtype::NS;
        let record_class = Qclass::IN;
        let name_servers = vec![server_info];
        // let response_arc: Arc<Mutex<Result<DnsMessage, ResolverError>>> = Arc::new(Mutex::new(Err(ResolverError::EmptyQuery)));

        let lookup_strategy = LookupStrategy::new(
            domain_name,
            record_type,
            record_class,
            config,
        );

        let response = lookup_strategy.transmit_query_to_server(
            name_servers.get(0).unwrap(),
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

        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(google_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(google_server, timeout);
        let server_info = server_info::ServerInfo::new_with_ip(google_server,conn_udp, conn_tcp);
        let config = ResolverConfig::default();
        let record_type = Qtype::A;
        let record_class = Qclass::CH;
        let name_servers = vec![server_info];
        // let response_arc: Arc<Mutex<Result<DnsMessage, ResolverError>>> = Arc::new(Mutex::new(Err(ResolverError::EmptyQuery)));

        let lookup_strategy = LookupStrategy::new(
            domain_name,
            record_type,
            record_class,
            config,
        );

        let response = lookup_strategy.transmit_query_to_server(
            name_servers.get(0).unwrap(),
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
    #[tokio::test] 
    async fn transmit_query_to_server_max_tries_0() {
       
        let max_retries = 0;

        let domain_name = DomainName::new_from_string("example.com".to_string());
        let timeout = Duration::from_secs(2);
        let record_type = Qtype::A;
        let record_class = Qclass::IN;
        // let response_arc: Arc<Mutex<Result<DnsMessage, ResolverError>>> = Arc::new(Mutex::new(Err(ResolverError::EmptyQuery)));
    
        let mut config: ResolverConfig = ResolverConfig::default();
        let non_existent_server:IpAddr = IpAddr::V4(Ipv4Addr::new(44, 44, 1, 81)); 

        let google_server:IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)); 
            
        config.set_retry(max_retries);
    
        let conn_udp_non:ClientUDPConnection = ClientUDPConnection::new(non_existent_server, timeout);
        let conn_tcp_non:ClientTCPConnection = ClientTCPConnection::new(non_existent_server, timeout);

        let conn_udp_google:ClientUDPConnection = ClientUDPConnection::new(google_server, timeout);
        let conn_tcp_google:ClientTCPConnection = ClientTCPConnection::new(google_server, timeout);
        let server_info_config_1 = server_info::ServerInfo::new_with_ip(google_server,conn_udp_google, conn_tcp_google);
        let server_info_config_2 = server_info::ServerInfo::new_with_ip(non_existent_server,conn_udp_non, conn_tcp_non);
        let server_info_1 = server_info::ServerInfo::new_with_ip(google_server,conn_udp_google, conn_tcp_google);
        let server_info_2 = server_info::ServerInfo::new_with_ip(non_existent_server,conn_udp_non, conn_tcp_non);
        config.set_name_servers(vec![server_info_config_1, server_info_config_2]);
            
        let name_servers =vec![server_info_1, server_info_2];
        // let response = transmit_query_to_server(
        //     domain_name, 
        //     record_type, 
        //     record_class,
        //     name_servers.get(0).unwrap(),
        //     &config,
        //     response_arc,
        //     timeout
        // ).await;

        let lookup_strategy = LookupStrategy::new(
            domain_name,
            record_type,
            record_class,
            config,
        );

        let response = lookup_strategy.transmit_query_to_server(
            name_servers.get(0).unwrap(),
            timeout
        ).await;

        println!("response {:?}",response);
            
        assert!(response.is_ok());
        assert!(response
            .clone()
            .unwrap()
            .to_dns_msg()
            .get_answer()
            .len() == 0);
        assert_eq!(response
            .unwrap()
            .to_dns_msg()
            .get_header()
            .get_rcode(), 2);
    }
           

    #[tokio::test] 
    async fn transmit_query_to_server_max_tries_1() {
        let max_retries = 1;
        let domain_name = DomainName::new_from_string("example.com".to_string());
        let timeout = Duration::from_secs(2);
        let record_type = Qtype::A;
        let record_class = Qclass::IN;
        // let response_arc: Arc<Mutex<Result<DnsMessage, ResolverError>>> = Arc::new(Mutex::new(Err(ResolverError::EmptyQuery)));
    
        let mut config: ResolverConfig = ResolverConfig::default();
        let non_existent_server:IpAddr = IpAddr::V4(Ipv4Addr::new(44, 44, 1, 81)); 

        let google_server:IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)); 
            
        config.set_retry(max_retries);
    
        let conn_udp_non:ClientUDPConnection = ClientUDPConnection::new(non_existent_server, timeout);
        let conn_tcp_non:ClientTCPConnection = ClientTCPConnection::new(non_existent_server, timeout);

        let conn_udp_google:ClientUDPConnection = ClientUDPConnection::new(google_server, timeout);
        let conn_tcp_google:ClientTCPConnection = ClientTCPConnection::new(google_server, timeout);
        let server_info_1 = server_info::ServerInfo::new_with_ip(google_server,conn_udp_google, conn_tcp_google);
        let server_info_2 = server_info::ServerInfo::new_with_ip(non_existent_server,conn_udp_non, conn_tcp_non);
        let server_info_config_1 = server_info::ServerInfo::new_with_ip(google_server,conn_udp_google, conn_tcp_google);
        let server_info_config_2 = server_info::ServerInfo::new_with_ip(non_existent_server,conn_udp_non, conn_tcp_non);
        config.set_name_servers(vec![server_info_config_1, server_info_config_2]);
            
        let name_servers =vec![server_info_2, server_info_1];
        // let response = transmit_query_to_server(
        //     domain_name, 
        //     record_type, 
        //     record_class,
        //     name_servers.get(0).unwrap(),
        //     &config,
        //     response_arc,
        //     timeout
        // ).await.unwrap(); // FIXME: add match instead of unwrap, the timeout error corresponds to

        let lookup_strategy = LookupStrategy::new(
            domain_name,
            record_type,
            record_class,
            config,
        );

        let response = lookup_strategy.transmit_query_to_server(
            name_servers.get(0).unwrap(),
            timeout
        ).await.unwrap();

        // IO error in ResolverError
        println!("response {:?}",response);

       assert!(response
        .to_dns_msg()
        .get_answer()
        .len() == 0);
       assert_eq!(response
        .to_dns_msg()
        .get_header()
        .get_rcode(), 2);
       assert!(response
        .to_dns_msg()
        .get_header()
        .get_ancount() == 0)
    }

    #[tokio::test] // TODO: finish up test
    async fn lookup_ip_cache_test() {
        let domain_name = DomainName::new_from_string("example.com".to_string());
        let record_type = Qtype::A;
        let record_class = Qclass::IN;
        let config: ResolverConfig = ResolverConfig::default();
        let addr = IpAddr::from_str("93.184.216.34").unwrap();
        let a_rdata = ARdata::new_from_addr(addr);
        let rdata = Rdata::A(a_rdata);
        let rr = ResourceRecord::new(rdata);

        let mut cache = DnsCache::new(NonZeroUsize::new(1));
        
        cache.add(domain_name.clone(), rr, record_type, record_class, None);

        // let query_sate: Arc<Mutex<Result<DnsMessage, ResolverError>>> = Arc::new(Mutex::new(Err(ResolverError::EmptyQuery)));

        // let _response_future = transmit_query_to_server(
        //     domain_name, 
        //     record_type, 
        //     record_class,
        //     config.get_name_servers().get(0).unwrap(),
        //     &config, 
        //     query_sate,
        //     tokio::time::Duration::from_secs(3)).await;

        let mut lookup_strategy = LookupStrategy::new(
            domain_name,
            record_type,
            record_class,
            config,
        );

        let _response_future = lookup_strategy.lookup_run(tokio::time::Duration::from_secs(3)).await;
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
            assert_eq!(dns_msg.get_header().get_rcode(), 0);
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
  
    // TODO: test empty response lookup_run
   
    // TODO: test lookup_run max rieswith max of 0 

}



    
