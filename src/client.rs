pub mod config;
pub mod client_connection;
pub mod tcp_connection;
pub mod udp_connection;

use crate::client::client_connection::ClientConnection;
use crate::message::{DnsMessage};
use crate::message::rclass::Rclass;
use crate::message::rtype::Rtype;
use std::net::{IpAddr,Ipv4Addr,UdpSocket,SocketAddr};

use rand::{thread_rng, Rng};
/*
TODO: send tcp 
TODO: caso para recibir truncados (no lo hace ahora)
 */


/// Struct that represents a Client dns
pub struct Client<T>
    where
        T: ClientConnection, 
{
    /// Conection
    conn: T ,
    /// query dns
    dns_query: DnsMessage,
}

impl <T: ClientConnection> Client<T> {
    
    /// Creates a new Client with default values
    /// # Example
    /// ```text
    /// let server_addr:IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
    /// let timeout: Duration = Duration::from_secs(2);
    /// let conn_tcp:ClientTCPConnection = ClientConnection::new(server_addr,timeout);
    /// let mut client = Client::new(conn_tcp);
    /// assert_eq!(client.dns_query.get_question().get_qname().get_name(), String::from(""));
    /// ```
    pub fn new(conn: T) -> Self {
        
        let client = Client { 
            conn: conn,
            dns_query:  DnsMessage::new(),
        };

        client
    }

    /// creates dns query with the given domain name, type and class
    /// # Example
    /// ```text
    /// let server_addr:IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
    /// let timeout: Duration = Duration::from_secs(2);
    /// let conn_tcp:ClientTCPConnection = ClientConnection::new(server_addr,timeout);
    /// let mut client = Client::new(conn_tcp);
    /// let dns_query = client.create_dns_query("www.test.com", "A", "IN");
    /// assert_eq!(dns_query.get_qname().get_name(), String::from("www.test.com"));
    /// assert_eq!(dns_query.get_qtype(), Rtype::A);
    /// assert_eq!(dns_query.get_qclass(), Rclass::IN);
    /// ```    
    pub fn create_dns_query(
        &mut self,
        domain_name: &str,
        qtype: &str,
        qclass: &str,
    ) -> DnsMessage {
        // Create random generator
        let mut rng = thread_rng();

        // Create query id
        let query_id: u16 = rng.gen();

        // Create query msg
        let client_query: DnsMessage = DnsMessage::new_query_message(
            domain_name.to_string(),
            qtype, 
            qclass,
            0,
            false,
            query_id,
        );
        self.dns_query = client_query.clone();

        client_query
    }

    /// Sends the query to the resolver in the client
    fn send_query(&self) -> DnsMessage {

        let client_query = self.get_dns_query();
        let conn: &T = &self.get_conn();

        //conn is in charge of send query
        let dns_response:DnsMessage = conn.send(client_query);

        dns_response
    }

    /// Get's the query from send_query and returns the response
    /// # Example
    /// ```text
    /// let server_addr:IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
    /// let timeout: Duration = Duration::from_secs(2);
    /// let conn_tcp:ClientTCPConnection = ClientConnection::new(server_addr,timeout);
    /// let mut client = Client::new(conn_tcp);
    /// let dns_query = client.create_dns_query("www.test.com", "A", "IN");
    /// let dns_response = client.query();
    /// assert_eq!(dns_response.get_question().get_qname().get_name(), String::from("www.test.com"));
    pub fn query(&mut self, domain_name: &str, qtype: &str, qclass: &str) -> DnsMessage {
        let dns_message = self.create_dns_query(domain_name, qtype, qclass);
        
        let response = self.send_query();

        response
    }

}

//Getters
impl <T: ClientConnection> Client<T> {

    fn get_conn(&self) -> &T {
        &self.conn
    }

    fn get_dns_query(&self)-> DnsMessage {
        return self.dns_query.clone();
    }
}

//Setters
impl <T: ClientConnection> Client<T>{

    fn set_conn(&mut self,conn :T) {
        self.conn = conn;
    }

    fn set_dns_query(&mut self,dns_query: DnsMessage) {
        self.dns_query = dns_query;
    }

}



#[cfg(test)]
mod client_test {
    use std::{net::{SocketAddr, IpAddr, Ipv4Addr}, time::Duration};
    use crate::message::{DnsMessage};
    use super::{Client, tcp_connection::ClientTCPConnection, client_connection::ClientConnection, udp_connection::ClientUDPConnection};

    #[test]
    fn udp_client_query() {
        //create connection
        let server_addr: IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let timeout: Duration = Duration::from_secs(2);

        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(server_addr, timeout);
        let mut udp_client = Client::new(conn_udp); //se crea un cliente vacio?


        // sends query
        let domain_name= "example.com";
        let qtype = "A"; 
        let qclass= "IN";
        let mut response = udp_client.query(domain_name, qtype, qclass).to_owned();

        response.print_dns_message()
    }

    #[test]
    fn tcp_client_query() {
        use std::net::{IpAddr,Ipv4Addr};
        use std::time::Duration;

        //create connection
        // let ip_addr: IpAddr = IpAddr::V4(Ipv4Addr::new(172, 18, 0, 1));s
        let server_addr:IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
        let timeout: Duration = Duration::from_secs(2);

        let conn_tcp:ClientTCPConnection = ClientConnection::new(server_addr,timeout);

        //create client
        let mut tcp_client = Client::new(conn_tcp);

        //create query
        let domain_name= "example.com";
        let qtype = "A"; 
        let qclass= "IN";
        let mut response = tcp_client.query(domain_name, qtype, qclass).to_owned();

        response.print_dns_message()
        //sends query
        
    }

    // Constructor test
    #[test]
    fn constructor_test(){
        let server_addr:IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
        let timeout: Duration = Duration::from_secs(2);

        let conn_tcp:ClientTCPConnection = ClientConnection::new(server_addr,timeout);
        let new_client = Client::new(conn_tcp);
        //assert_eq!(new_client.get_conn().get_server_addr(), server_addr);
        assert_eq!(new_client.get_dns_query().get_question().get_qname().get_name(), String::from(""));
    }
    // Query UDP
    
    // Query TCP
    
    // Query timeout

    // Querys with error
    
 

}