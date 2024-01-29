use crate::client::client_error::ClientError;
use crate::domain_name::DomainName;
use crate::message::DnsMessage;
use crate::message::header::Header;
use crate::client::client_connection::ClientConnection;
use crate::message::class_qclass::Qclass;
use crate::message::type_qtype::Qtype;
use std::net::IpAddr;
use std::thread;
use std::time::Duration;
use rand::{thread_rng, Rng};
use tokio::time::error::Elapsed;
use super::resolver_error::ResolverError;
use std::sync::{Mutex,Arc};
use crate::client::client_connection::ConnectionProtocol;
use crate::async_resolver::config::ResolverConfig;
use crate::client::udp_connection::ClientUDPConnection;
use crate::client::tcp_connection::ClientTCPConnection;
use tokio::time::timeout;
/// Future returned from `AsyncResolver` when performing a lookup with Rtype A.
/// 
/// This implementation of `Future` is used to send a single query to a DNS server.
/// When this future is polled by `AsyncResolver`, 
pub struct LookupStrategy {
    /// Domain Name associated with the query.
    name: DomainName,
    /// Qtype of search query
    record_type: Qtype,
    /// Qclass of the search query
    record_class: Qclass,
    /// Resolver configuration.
    config: ResolverConfig,
    /// Future that contains the response of the query.
    /// 
    /// The `Output` of this future is a `Result<DnsMessage, ResolverError>`.
    /// The returned `DnsMessage` contains the corresponding response of the query.
    pub query_answer: Arc<std::sync::Mutex<Result<DnsMessage, ResolverError>>>,
}
    
impl LookupStrategy {

    /// Creates a new `LookupIpFutureStub` with the given configuration.
    /// 
    /// The resulting future created by default contains an empty `DnsMessage`
    /// which is going to be replaced by the response of the query after
    /// `LookupIpFutureStub` is polled.
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

    pub async fn lookup_run(
        &mut self           
    ) -> Result<DnsMessage, ResolverError> {
        let response=  
        self.query_answer.clone();

        let name = self.name.clone();
        let record_type = self.record_type;
        let record_class = self.record_class;
        let config = self.config.clone();
        
        let result_response = execute_lookup_strategy(
            name, 
            record_type,
            record_class,
            config.get_name_servers(), 
            config,
            response).await;
        return result_response;
    }
}

/// [RFC 1034]: https://datatracker.ietf.org/doc/html/rfc1034#section-5.3.1
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
/// Perfoms the lookup of a Domain Name acting as a Stub Resolver.
/// This function performs the lookup of the requested records asynchronously. 
/// The given `waker` is used to wake up the task when the query is answered. 
/// The `referenced_query` is used to update the future that contains the response of the query.
/// 
/// After creating the query with the given parameters, the function sends it to the name servers 
/// specified in the configuration. 
/// 
/// When a response is received, the function performs the parsing of the response to a `DnsMessage`.
/// After the response is checked, the function updates the future that contains the response of the query.
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
/// let response = execute_lookup_strategy(domain_name,record_type, cache, name_servers, waker,query,config).await.unwrap();
/// ```
///
pub async fn execute_lookup_strategy(
    name: DomainName,
    record_type: Qtype,
    record_class: Qclass,
    name_servers: Vec<(ClientUDPConnection, ClientTCPConnection)>,
    config: ResolverConfig,
    response_arc: Arc<std::sync::Mutex<Result<DnsMessage, ResolverError>>>,
) -> Result<DnsMessage, ResolverError>  {
    println!("[execute_lookup_strategy]");
    // Create random generator
    let mut rng = thread_rng();

    // Create query id
    let query_id: u16 = rng.gen();

    // Create query
    let new_query = DnsMessage::new_query_message(
        name.clone(),
        record_type,
        record_class,
        0,
        false,
        query_id
    );

    // Create Server failure query 
    let mut response = new_query.clone(); // le quite el to_owned
    let mut new_header: Header = response.get_header();
    new_header.set_rcode(2);
    new_header.set_qr(true);
    response.set_header(new_header);

    let mut result_dns_msg: Result<DnsMessage, ResolverError> = Ok(response.clone());
    let mut retry_count = 0;
    let mut i = 0;
    
    loop {
        let mut response_guard = response_arc.lock().unwrap();
        let response = response_guard.as_ref();

        if response.is_ok() || retry_count >= config.get_retry() {
            break; 
        }

        let connections = name_servers.get(i).unwrap();
        result_dns_msg = 
                timeout(Duration::from_secs(6), 
            send_query_resolver_by_protocol(
                        config.get_protocol(),
                        new_query.clone(),
                        result_dns_msg.clone(),
                        connections,
                    )).await
                .unwrap_or_else(|_| {
                    Err(ResolverError::Message("Timeout Error".into()))
                });  
        
        *response_guard = result_dns_msg.clone();
        retry_count = retry_count + 1;
        i = i+1;

    }

    let response_dns_msg = match result_dns_msg.clone() {
        Ok(response_message) => response_message,
        Err(ResolverError::Parse(_)) => {
            let mut format_error_response = response.clone();
            let mut header = format_error_response.get_header();
            header.set_rcode(1);
            format_error_response.set_header(header);
            format_error_response
        }
        Err(_) => response,
    };

    Ok(response_dns_msg)  
}



///  Sends a DNS query to a resolver using the specified connection protocol.
/// 
///  This function takes a DNS query, a result containing a DNS message,
///  and connection information. Depending on the specified protocol (UDP or TCP),
///  it sends the query using the corresponding connection and updates the result
///  with the parsed response.

async fn send_query_resolver_by_protocol(
    protocol: ConnectionProtocol,
    query:DnsMessage,
    mut result_dns_msg: Result<DnsMessage, ResolverError>, 
    connections:  &(ClientUDPConnection , ClientTCPConnection)
)
->  Result<DnsMessage, ResolverError>{
    let query_id = query.get_query_id();
    match protocol{ 
        ConnectionProtocol::UDP => {
            let result_response = connections.0.send(query.clone()).await;
            result_dns_msg = parse_response(result_response,query_id);
        }
        ConnectionProtocol::TCP => {
            let result_response = connections.1.send(query.clone()).await;
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
fn parse_response(response_result: Result<(Vec<u8>, IpAddr), ClientError>, query_id:u16) -> Result<DnsMessage, ResolverError> {
    let dns_msg = response_result.map_err(Into::into)
        .and_then(|(response_message , _ip)| {
            DnsMessage::from_bytes(&response_message)
                .map_err(|_| ResolverError::Parse("The name server was unable to interpret the query.".to_string()))
        })?;
    let header = dns_msg.get_header();
    
    // check Header
    header.format_check()
    .map_err(|e| ResolverError::Parse(format!("Error formated Header: {}", e)))?;

    // Check ID
    if dns_msg.get_query_id() != query_id {
        println!("[ID RESPONSE] {:?}",dns_msg.get_query_id());
        println!("[ID QUERY] {:?}",query_id);
        return  Err(ResolverError::Parse("Error expected ID from query".to_string()))
    }

    if header.get_qr() {
        return Ok(dns_msg);
    }
    Err(ResolverError::Parse("Message is a query. A response was expected.".to_string()))
}

#[cfg(test)]
mod async_resolver_test {
    // use tokio::runtime::Runtime;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;
    use crate::{ domain_name::DomainName, dns_cache::DnsCache};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::str::FromStr;
    use std::time::Duration;
    use super::*;
   
    #[test]
    fn lookup() {

        let domain_name = DomainName::new_from_string("example.com".to_string());
        let domain_name_cache = DomainName::new_from_string("test.com".to_string());
        let config: ResolverConfig = ResolverConfig::default();
        
        let mut cache: DnsCache = DnsCache::new();
        cache.set_max_size(20);

        let a_rdata = Rdata::A(ARdata::new());
        let resource_record = ResourceRecord::new(a_rdata);
        cache.add(domain_name_cache, resource_record);

        let record_type = Qtype::A;
        let record_class = Qclass::IN;

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
    async fn execute_lookup_strategy_a_response() {
        let domain_name: DomainName = DomainName::new_from_string("example.com".to_string());

        let google_server:IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let timeout: Duration = Duration::from_secs(20);

        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(google_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(google_server, timeout);

        let config = ResolverConfig::default();
        let record_type = Qtype::A;
        let record_class = Qclass::IN;
        let name_servers = vec![(conn_udp,conn_tcp)];
        let response_arc = Arc::new(Mutex::new(Err(ResolverError::EmptyQuery)));

        let response = execute_lookup_strategy(
            domain_name,
            record_type,
            record_class, 
            name_servers, 
            config,
            response_arc
        ).await;

        println!("response {:?}", response);

        // assert_eq!(response.get_header().get_qr(),true);
        // assert_ne!(response.get_answer().len(),0);
    }   

    #[tokio::test]
    async fn execute_lookup_strategy_ns_response() {
        let domain_name = DomainName::new_from_string("example.com".to_string());
    
        // Create vect of name servers
        let google_server:IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let timeout: Duration = Duration::from_secs(20);

        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(google_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(google_server, timeout);

        let config = ResolverConfig::default();
        let record_type = Qtype::NS;
        let record_class = Qclass::IN;
        let name_servers = vec![(conn_udp,conn_tcp)];
        let response_arc = Arc::new(Mutex::new(Err(ResolverError::EmptyQuery)));
        
        let response = execute_lookup_strategy(
            domain_name, 
            record_type, 
            record_class,
            name_servers, 
            config,
            response_arc
        ).await.unwrap();

        assert_eq!(response.get_header().get_qr(),true);
        assert_ne!(response.get_header().get_ancount(),2);

    } 

    #[tokio::test]
    async fn execute_lookup_strategy_ch_response() {
        let domain_name = DomainName::new_from_string("example.com".to_string());

        let google_server:IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let timeout: Duration = Duration::from_secs(20);

        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(google_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(google_server, timeout);

        let config = ResolverConfig::default();
        let record_type = Qtype::A;
        let record_class = Qclass::CH;
        let name_servers = vec![(conn_udp,conn_tcp)];
        let response_arc = Arc::new(Mutex::new(Err(ResolverError::EmptyQuery)));

        let response = execute_lookup_strategy(
            domain_name,
            record_type,
            record_class, 
            name_servers,
            config,
            response_arc
        ).await.unwrap();


        assert_eq!(response.get_header().get_qr(),true);
        assert_eq!(response.get_answer().len(),0);
    } 
    #[tokio::test] 
    async fn execute_lookup_strategy_max_tries_0() {
       
        let max_retries = 0;

        let domain_name = DomainName::new_from_string("example.com".to_string());
        let timeout = Duration::from_secs(2);
        let record_type = Qtype::A;
        let record_class = Qclass::IN;
        let response_arc = Arc::new(Mutex::new(Err(ResolverError::EmptyQuery)));
    
        let mut config: ResolverConfig = ResolverConfig::default();
        let non_existent_server:IpAddr = IpAddr::V4(Ipv4Addr::new(44, 44, 1, 81)); 

        let google_server:IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)); 
            
        config.set_retry(max_retries);
    
        let conn_udp_non:ClientUDPConnection = ClientUDPConnection::new(non_existent_server, timeout);
        let conn_tcp_non:ClientTCPConnection = ClientTCPConnection::new(non_existent_server, timeout);

        let conn_udp_google:ClientUDPConnection = ClientUDPConnection::new(google_server, timeout);
        let conn_tcp_google:ClientTCPConnection = ClientTCPConnection::new(google_server, timeout);
        config.set_name_servers(vec![(conn_udp_non,conn_tcp_non), (conn_udp_google,conn_tcp_google)]);
            
        let name_servers =vec![(conn_udp_non,conn_tcp_non), (conn_udp_google,conn_tcp_google)];
        let response = execute_lookup_strategy(
            domain_name, 
            record_type, 
            record_class,
            name_servers,
            config,
            response_arc
        ).await;
        println!("response {:?}",response);
            
        assert!(response.is_ok());
        assert!(response.clone().unwrap().get_answer().len() == 0);
        assert_eq!(response.unwrap().get_header().get_rcode(), 2);
    }
           

    #[tokio::test] 
    async fn execute_lookup_strategy_max_tries_1() {
       
        let max_retries = 1;

        let domain_name = DomainName::new_from_string("example.com".to_string());
        let timeout = Duration::from_secs(2);
        let record_type = Qtype::A;
        let record_class = Qclass::IN;
        let response_arc = Arc::new(Mutex::new(Err(ResolverError::EmptyQuery)));
    
        let mut config: ResolverConfig = ResolverConfig::default();
        let non_existent_server:IpAddr = IpAddr::V4(Ipv4Addr::new(44, 44, 1, 81)); 

        let google_server:IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)); 
            
        config.set_retry(max_retries);
    
        let conn_udp_non:ClientUDPConnection = ClientUDPConnection::new(non_existent_server, timeout);
        let conn_tcp_non:ClientTCPConnection = ClientTCPConnection::new(non_existent_server, timeout);

        let conn_udp_google:ClientUDPConnection = ClientUDPConnection::new(google_server, timeout);
        let conn_tcp_google:ClientTCPConnection = ClientTCPConnection::new(google_server, timeout);
        config.set_name_servers(vec![(conn_udp_non,conn_tcp_non), (conn_udp_google,conn_tcp_google)]);
            
        let name_servers =vec![(conn_udp_non,conn_tcp_non), (conn_udp_google,conn_tcp_google)];
        let response = execute_lookup_strategy(
            domain_name, 
            record_type, 
            record_class,
            name_servers,
            config,
            response_arc
        ).await.unwrap();
        println!("response {:?}",response);

       assert!(response.get_answer().len() == 0);
       assert_eq!(response.get_header().get_rcode(), 2);
       assert!(response.get_header().get_ancount() == 0)
                
    }

    #[tokio::test]
    async fn lookup_ip_cache_test() {

        let domain_name = DomainName::new_from_string("example.com".to_string());
        let record_type = Qtype::A;
        let record_class = Qclass::IN;
        
        let config: ResolverConfig = ResolverConfig::default();
        
        let addr = IpAddr::from_str("93.184.216.34").unwrap();
        let a_rdata = ARdata::new_from_addr(addr);
        let rdata = Rdata::A(a_rdata);
        let rr = ResourceRecord::new(rdata);

        let mut cache = DnsCache::new();
        cache.set_max_size(1);
        cache.add(domain_name.clone(), rr);

        let query_sate = Arc::new(Mutex::new(Err(ResolverError::EmptyQuery)));

        let _response_future = execute_lookup_strategy(domain_name, record_type, record_class,config.get_name_servers(),config, query_sate).await;
        
        // TODO: test
    }  
    
/*
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
        let ip 
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
  */  

    // TODO: test empty response lookup_run
   
    // TODO: test lookup_run max rieswith max of 0 

}


