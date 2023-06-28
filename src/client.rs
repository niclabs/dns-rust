pub mod config;
pub mod client_connection;
pub mod tcp_connection;
pub mod udp_connection;

use crate::client::client_connection::ClientConnection;
use crate::message::{DnsMessage, Rtype,Rclass};
use std::net::{IpAddr,Ipv4Addr,UdpSocket,SocketAddr};

use rand::{thread_rng, Rng};
/*
TODO: send tcp 
TODO: caso para recibir truncados (no lo hace ahora)
TODO: valores que vengan por defecto en un constructor por ejemplo el puerto 53, el socket_Addr 
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
    pub fn new(conn: T) -> Self {
        
        let client = Client { 
            conn: conn,
            dns_query:  DnsMessage::new(),
        };

        client
    }

    ///creates dns query with the given domain name, type and class    
    pub fn create_dns_query(
        &mut self,
        domain_name: String,
        qtype: String,
        qclass: String,
    ){
        // Create random generator
        let mut rng = thread_rng();

        // Create query id
        let query_id: u16 = rng.gen();

        // Create query msg
        let client_query: DnsMessage = DnsMessage::new_query_message(
            domain_name,
            qtype,
            qclass,
            0,
            false,
            query_id,
        );
        self.dns_query = client_query;
    }

    ///Sends the query to the resolver in the client
    fn send_query(&self,server_addr:SocketAddr) -> DnsMessage {

        let client_query = self.get_dns_query();
        let conn: &T = &self.get_conn();

        //conn is in charge of send query
        let dns_response:DnsMessage = conn.send(client_query);
        return  dns_response;
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
    use std::net::SocketAddr;

    use super::{Client, tcp_connection::TCPConnection, client_connection::ClientConnection, udp_connection::UDPConnection};

    #[test]
    fn example_use(){
        use std::net::{IpAddr,Ipv4Addr};
        use std::time::Duration;


        //create connection
        // let ip_addr: IpAddr = IpAddr::V4(Ipv4Addr::new(172, 18, 0, 1));
        let port: u16 = 8089;
        let ip_addr_to_connect:IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));

        // let addr: SocketAddr = SocketAddr::new(ip_addr, port);
        let timeout: Duration = Duration::from_secs(2);
        let addr_cloudfare = SocketAddr::new(ip_addr_to_connect, port)
;       let conn_udp:UDPConnection = ClientConnection::new(ip_addr_to_connect,timeout);
        let conn_tcp:TCPConnection = ClientConnection::new(ip_addr_to_connect,timeout);

        //create client
        let mut client_udp = Client::new(conn_udp); //se crea un cliente vacio?
        let mut client_tcp = Client::new(conn_tcp);

        //create query
        let domain_name_udp = String::from("uchile.cl");
        let domain_name_tcp = String::from("uchile.cl");
        let qtype_udp = String::from("A");
        let qtype_tcp = String::from("A");
        let qclass_udp:String = String::from("IN");
        let qclass_tcp:String = String::from("IN");

        client_udp.create_dns_query(domain_name_udp,qtype_udp,qclass_udp);
        client_tcp.create_dns_query(domain_name_tcp,qtype_tcp,qclass_tcp);        

        //sends query
        let ip_Addr_server:IpAddr = IpAddr::V4(Ipv4Addr::new(1,1,1,1));
        let port_dns_udp:u16 = 53;
        let server_addr:SocketAddr = SocketAddr::new(ip_Addr_server,port_dns_udp);
        client_tcp.send_query(server_addr);    //will send through tcp
        client_udp.send_query(server_addr);    //will send through udp



    }

   

    // Constructor test
    
    // Query UDP
    
    // Query TCP
    
    // Query timeout

    // Querys with error
    
 

}