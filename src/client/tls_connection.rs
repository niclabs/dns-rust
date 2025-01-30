use crate::message::DnsMessage;
use crate::message::rdata::Rdata;
use crate::message::rdata::a_rdata::ARdata;
use crate::message::resource_record::ResourceRecord;
use super::client_error::ClientError;
use super::client_security::ClientSecurity;
use async_trait::async_trait;
use rustls::pki_types::ServerName;
use std::convert::TryFrom;
use std::io::Error as IoError;
use std::io::ErrorKind;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
use tokio::net::{lookup_host, TcpStream};
use std::net::IpAddr;
use std::net::SocketAddr;
use tokio::time::Duration;
use tokio::time::timeout;
use tokio_rustls::rustls::ClientConfig;
use tokio_rustls::TlsConnector;
use std::sync::Arc;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClientTLSConnection {
    /// Client address
    server_addr: IpAddr,
    /// Read time timeout
    timeout: tokio::time::Duration,
    new_default: fn(IpAddr, Duration) -> Self,
}

#[async_trait]
impl ClientSecurity for ClientTLSConnection {

    /// Creates TLSConnection
    fn new(server_addr:IpAddr, timeout: Duration) -> Self {
        ClientTLSConnection {
            server_addr: server_addr,
            timeout: timeout,
            new_default: ClientTLSConnection::new_default,
        }
    }

    fn new_default(server_addr:IpAddr, timeout: Duration) -> Self {
        ClientTLSConnection {
            server_addr: server_addr,
            timeout: timeout,
            new_default: ClientTLSConnection::new_default,
        }
    }

    ///implement get_ip
    /// returns IpAddr
    fn get_ip(&self) -> IpAddr {
        return self.server_addr.clone();
    }

    /// creates socket tcp, sends query and receive response
    async fn send(self, dns_query: DnsMessage) -> Result<Vec<u8>, ClientError> {
        // Configure the root certificate store with platform-native certificates
        let mut roots = rustls::RootCertStore::empty();
        for cert in rustls_native_certs::load_native_certs().expect("could not load platform certs") {
            roots.add(cert).unwrap();
        }
        let config = ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth();

        // Resolve the server's IP address to a domain name
        let server_name_res = Self::resolve_hostname(self.get_server_addr()).await;
        let server_name = match server_name_res {
            Ok(server_name_str) => ServerName::try_from(server_name_str).expect("invalid DNS name"),
            Err(_) => return Err(ClientError::FormatError("Unable to resolve the IP address to a valid domain.")),
        };

        // Create a TLS connector with the configured certificates
        let connector = TlsConnector::from(Arc::new(config));

        // Connect to the DNS server over TCP on port 853
        let server_addr: SocketAddr = SocketAddr::new(self.get_server_addr(), 853);
        let stream = TcpStream::connect(server_addr).await.map_err(|e| ClientError::from(e))?;

        // Verify that the connected IP matches the expected IP
        let actual_ip = stream.peer_addr()?.ip();
        let expected_ip = self.get_server_addr();
        if actual_ip != expected_ip {
            return Err(ClientError::Io(IoError::new(
                ErrorKind::PermissionDenied,
                format!("IP mismatch: expected {}, got {}", expected_ip, actual_ip),
            )).into());
        }

        // // Establish the TLS connection
        let mut tls_stream = connector.connect(server_name, stream).await.map_err(|e| {
            ClientError::Io(IoError::new(ErrorKind::Other, format!("TLS connection error: {}", e)))
        })?;


        // Prepare the DNS query message
        let bytes = dns_query.to_bytes();
        let msg_length = bytes.len() as u16;
        let full_msg = [&msg_length.to_be_bytes(), bytes.as_slice()].concat();

        // Send the DNS query over the TLS connection
        tls_stream.write_all(&full_msg).await?;

        // Read the size of the response
        let mut msg_size_response: [u8; 2] = [0; 2];
        tls_stream.read_exact(&mut msg_size_response).await?;
        let tls_msg_len: u16 = u16::from_be_bytes(msg_size_response);

        // Read the full DNS response
        let mut response = vec![0u8; tls_msg_len as usize];
        tls_stream.read_exact(&mut response).await?;

        // Return the response
        Ok(response)
    }
}

//Getters
impl ClientTLSConnection {

    pub fn get_server_addr(&self)-> IpAddr {
        return self.server_addr.clone();
    }

    pub fn get_timeout(&self)-> Duration {
        return self.timeout.clone();
    }

    /// Resolves the IP to a domain name or returns an error if it cannot be resolved.
    async fn resolve_hostname(ip: IpAddr) -> Result<String, String> {
        let socket_addr = format!("{}:843", ip); // Use port 443 (HTTPS) or the appropriate one
        match lookup_host(socket_addr).await {
            Ok(mut addrs) => {
                // If the IP is resolved, return the domain name
                if let Some(SocketAddr::V4(addr)) = addrs.next() {
                    return Ok(addr.ip().to_string());
                }
            }
            Err(_) => {
                // If resolution fails, return an error
                return Err("Could not resolve the IP to a domain name.".to_string());
            }
        }

        // If no domain is found, return an error
        Err("Unable to resolve the IP address to a valid domain.".to_string())
    }



}

//Setters
impl ClientTLSConnection {

    pub fn set_server_addr(&mut self,addr :IpAddr) {
        self.server_addr = addr;
    }

    pub fn set_timeout(&mut self,timeout: Duration) {
        self.timeout = timeout;
    }

}

#[cfg(test)]
mod tls_connection_test{
    use super::*;
    use std::net::{IpAddr,Ipv4Addr,Ipv6Addr};
    use crate::domain_name::DomainName;
    use crate::message::rrtype::Rrtype;
    use crate::message::rclass::Rclass;
    #[test]
    fn create_tls() {

        // let domain_name = String::from("uchile.cl");
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let _port: u16 = 8088;
        let timeout = Duration::from_secs(100);

        let _conn_new = ClientTLSConnection::new(ip_addr,timeout);

        assert_eq!(_conn_new.get_server_addr(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
        assert_eq!(_conn_new.get_timeout(),  Duration::from_secs(100));
    }
    #[test]
    fn get_ip_v4(){
        let ip_address = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);
        let connection = ClientTLSConnection::new(ip_address, timeout);
        //check if the ip is the same
        assert_eq!(connection.get_ip(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
    }

    #[test]
    fn get_ip_v6(){
        // ip in V6 version is the equivalent to (192, 168, 0, 1) in V4
        let ip_address = IpAddr::V6(Ipv6Addr::new(0xc0, 0xa8, 0, 1, 0, 0, 0, 0));
        let timeout = Duration::from_secs(100);
        let connection = ClientTLSConnection::new(ip_address, timeout);
        //check if the ip is the same
        assert_eq!(connection.get_ip(), IpAddr::V6(Ipv6Addr::new(0xc0, 0xa8, 0, 1, 0, 0, 0, 0)));
    }
    #[test]
    fn get_server_addr(){
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);
        let mut _conn_new = ClientTLSConnection::new(ip_addr,timeout);

        assert_eq!(_conn_new.get_server_addr(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
    }

    #[test]
    fn set_server_addr(){
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);
        let mut _conn_new = ClientTLSConnection::new(ip_addr,timeout);

        assert_eq!(_conn_new.get_server_addr(), IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));

        _conn_new.set_server_addr(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

        assert_eq!(_conn_new.get_server_addr(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    }
    #[test]
    fn get_timeout(){
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);
        let mut _conn_new = ClientTLSConnection::new(ip_addr,timeout);

        assert_eq!(_conn_new.get_timeout(),  Duration::from_secs(100));
    }

    #[test]
    fn set_timeout(){
        let ip_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);
        let mut _conn_new = ClientTLSConnection::new(ip_addr,timeout);

        assert_eq!(_conn_new.get_timeout(),  Duration::from_secs(100));

        _conn_new.set_timeout(Duration::from_secs(200));

        assert_eq!(_conn_new.get_timeout(),  Duration::from_secs(200));
    }

    #[tokio::test]
    async fn send() {
        let ip_addr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
        let timeout = Duration::from_secs(100);
        let mut _conn_new = ClientTLSConnection::new(ip_addr,timeout);
        let mut domain_name = DomainName::new();
        domain_name.set_name("example.com".to_string());
        let question = DnsMessage::new_query_message(domain_name, Rrtype::A, Rclass::IN, 0, true, 0);
        let mut dns_query = DnsMessage::new();
        
        let response = _conn_new.send(dns_query).await;
        assert_eq!(response.is_ok(), true);
    }


   

}