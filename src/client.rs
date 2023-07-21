pub mod config;
pub mod client_connection;
pub mod tcp_connection;
pub mod udp_connection;

use crate::client::client_connection::ClientConnection;
use crate::message::DnsMessage;
use crate::domain_name::DomainName;

use rand::{thread_rng, Rng};
/*
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
    /// assert_eq!(client.get_conn().get_server_addr(), server_addr);
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
        domain_name: DomainName,
        qtype: &str,
        qclass: &str,
    ) -> DnsMessage {
        // Create random generator
        let mut rng = thread_rng();

        // Create query id
        let query_id: u16 = rng.gen();

        // Create query msg
        let client_query: DnsMessage = DnsMessage::new_query_message(
            domain_name.get_name(),
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
    /// # Example
    /// ```text
    /// let server_addr:IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
    /// let timeout: Duration = Duration::from_secs(2);
    /// let conn_tcp:ClientTCPConnection = ClientConnection::new(server_addr,timeout);
    /// let mut client = Client::new(conn_tcp);
    /// let dns_query = client.create_dns_query("www.test.com", "A", "IN");
    /// let dns_response = client.send_query();
    /// assert_eq!(client.get_conn().get_server_addr(), server_addr);
    /// assert_eq!(dns_response.get_question().get_qtype(), Rtype::A);
    /// assert_eq!(dns_response.get_question().get_qname().get_name(), String::from("www.test.com"));
    /// ```
    fn send_query(&self) -> DnsMessage {

        let client_query = self.get_dns_query();
        let conn: &T = &self.get_conn();

        //conn is in charge of send query
        let dns_response:DnsMessage = match conn.send(client_query) {
            Ok(dns_message) => dns_message,
            Err(e) => panic!("Error: {}",e),
        };

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
    /// assert_eq!(client.get_conn().get_server_addr(), server_addr);
    /// assert_eq!(dns_response.get_question().get_qtype(), Rtype::A);
    /// assert_eq!(dns_response.get_question().get_qname().get_name(), String::from("www.test.com"));
    pub fn query(&mut self, domain_name: DomainName, qtype: &str, qclass: &str) -> DnsMessage {
        let _dns_message = self.create_dns_query(domain_name, qtype, qclass);
        
        let response = self.send_query();

        response
    }

}

#[allow(dead_code)]
//Getters
impl <T: ClientConnection> Client<T> {

    fn get_conn(&self) -> &T {
        &self.conn
    }

    fn get_dns_query(&self)-> DnsMessage {
        return self.dns_query.clone();
    }
}

#[allow(dead_code)]
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
    use std::{net::{IpAddr, Ipv4Addr}, time::Duration};
    use crate::message::type_qtype::Qtype;
    use crate::message::class_qclass::Qclass;
    use crate::message::rdata::Rdata;
    use crate::domain_name::DomainName;
    use super::{Client, tcp_connection::ClientTCPConnection, client_connection::ClientConnection, udp_connection::ClientUDPConnection};

    #[test]
    fn udp_client_query() {
        //create connection
        let server_addr: IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let timeout: Duration = Duration::from_secs(2);

        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(server_addr, timeout);
        let mut udp_client = Client::new(conn_udp); //se crea un cliente vacio?

        let mut domain_name = DomainName::new();



        // sends query
        domain_name.set_name(String::from("test.test2.com."));
        let qtype = "A"; 
        let qclass= "IN";
        let response = udp_client.query(domain_name, qtype, qclass).to_owned();

        let expected_ip: [u8; 4] = [93, 184, 216, 34];
        let answers = response.get_answer();
        for answer in answers {
            let a_rdata = answer.get_rdata();
            match a_rdata {
                Rdata::SomeARdata(val) => {
                    assert_eq!(val.get_address(), expected_ip)
                },
                _ => {}
            }
        }
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
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("test.test2.com."));
        let qtype = "A"; 
        let qclass= "IN";
        let response = tcp_client.query(domain_name, qtype, qclass).to_owned();
        let expected_ip: [u8; 4] = [93, 184, 216, 34];
        let answers = response.get_answer();
        for answer in answers {
            let a_rdata = answer.get_rdata();
            match a_rdata {
                Rdata::SomeARdata(val) => {
                    assert_eq!(val.get_address(), expected_ip)
                },
                _ => {}
            }
        }        
    }

    // Constructor test
    #[test]
    fn constructor_test(){
        let server_addr:IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
        let timeout: Duration = Duration::from_secs(2);

        let conn_tcp:ClientTCPConnection = ClientConnection::new(server_addr,timeout);
        let new_client = Client::new(conn_tcp);
        assert_eq!(new_client.get_conn().get_server_addr(), server_addr);
        assert_eq!(new_client.get_dns_query().get_question().get_qname().get_name(), String::from(""));
    }
    
    // Query UDP
    #[test]
    fn create_dns_query_udp(){
        let server_addr:IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
        let timeout: Duration = Duration::from_secs(2);

        let conn_udp:ClientUDPConnection = ClientConnection::new(server_addr,timeout);
        let mut new_client = Client::new(conn_udp);
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("www.test.com"));
        let dns_query = new_client.create_dns_query(domain_name, "A", "IN");

        assert_eq!(dns_query.get_question().get_qtype(), Qtype::A);
        assert_eq!(dns_query.get_question().get_qname().get_name(), String::from("www.test.com"));
        assert_eq!(dns_query.get_question().get_qclass(), Qclass::IN);
    }
    
    // Query TCP
    #[test]
    fn create_dns_query_tcp(){
        let server_addr:IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
        let timeout: Duration = Duration::from_secs(2);

        let conn_tcp:ClientTCPConnection = ClientConnection::new(server_addr,timeout);
        let mut new_client = Client::new(conn_tcp);
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("www.test.com"));
        let dns_query = new_client.create_dns_query(domain_name, "A", "IN");

        assert_eq!(dns_query.get_question().get_qtype(), Qtype::A);
        assert_eq!(dns_query.get_question().get_qname().get_name(), String::from("www.test.com"));
        assert_eq!(dns_query.get_question().get_qclass(), Qclass::IN);
    }

    #[test]
    #[should_panic]
    fn query_timeout_tcp(){
        let server_addr:IpAddr = IpAddr::V4(Ipv4Addr::new(171, 18, 0, 1));
        let timeout: Duration = Duration::from_secs(2);

        let conn_tcp:ClientTCPConnection = ClientConnection::new(server_addr,timeout);
        let mut new_client = Client::new(conn_tcp);
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("www.u-cursos.cl"));
        new_client.create_dns_query(domain_name, "A", "IN");

        new_client.send_query();
    }

    #[test]
    #[should_panic]
    fn query_timeout_udp(){
        let server_addr:IpAddr = IpAddr::V4(Ipv4Addr::new(171, 18, 0, 1));
        let timeout: Duration = Duration::from_secs(2);

        let conn_udp:ClientUDPConnection = ClientConnection::new(server_addr,timeout);
        let mut new_client = Client::new(conn_udp);
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("www.u-cursos.cl"));
        new_client.create_dns_query(domain_name, "A", "IN");
        new_client.send_query();
    }
    // Querys with error

    //Wrong domain starting with "?"
    #[test]
    #[should_panic]
    fn wrong_written_domain(){
        let server_addr:IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let timeout: Duration = Duration::from_secs(2);

        let conn_tcp:ClientTCPConnection = ClientConnection::new(server_addr,timeout);
        let mut new_client = Client::new(conn_tcp);
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("?www.u-cursos.cl"));
        let domain_name_copy =domain_name.clone();
        new_client.create_dns_query(domain_name, "A", "IN");
        let mut response = new_client.query(domain_name_copy, "A", "IN");

        response.print_dns_message();
    }

    //Wrong domain that doesn't exist: should panic?
    #[test]
    fn domain_that_does_not_exist(){
        let server_addr:IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let timeout: Duration = Duration::from_secs(2);

        let conn_tcp:ClientTCPConnection = ClientConnection::new(server_addr,timeout);
        let mut new_client = Client::new(conn_tcp);
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("www.wrong-domain.cl"));
        let mut response = new_client.query(domain_name, "A", "IN");

        response.print_dns_message();
        
    }

    //Wrong domain that haves a number at the start
    #[test]
    #[should_panic]
    fn wrong_written_domain_2(){
        let server_addr:IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let timeout: Duration = Duration::from_secs(2);

        let conn_tcp:ClientTCPConnection = ClientConnection::new(server_addr,timeout);
        let mut new_client = Client::new(conn_tcp);
        let mut domain_name = DomainName::new();
        domain_name.set_name(String::from("2www.u-cursos.cl"));
        let mut response = new_client.query(domain_name, "A", "IN");

        response.print_dns_message();
    }
 

}