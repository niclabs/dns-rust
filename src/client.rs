pub mod config;

use crate::client::config::CLIENT_IP_PORT;
use crate::client::config::HOST_NAME;
use crate::client::config::QCLASS;
use crate::client::config::QTYPE;
use crate::client::config::RESOLVER_IP_PORT;
use crate::client::config::TIMEOUT;
use crate::client::connection::CLientConnection;

use crate::client::config::TRANSPORT;
use crate::message::DnsMessage;
use crate::resolver::Resolver;

use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::net::UdpSocket;
use std::time::{Duration, Instant};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::client::connection;



/// Struct that represents a Client dns
pub struct Client {
    /// Conection
    conn: CLientConnection,
    /// query dns
    dns_query: DnsMessage,
}

impl Client{
    
    /// Creates a new Client with default values
    pub fn new(conn: CLientConnection) -> Self {
        
        let client = Client { 
            conn: (),
            dns_query: () 
        };

        client
    }

    ///creates dns query with the given domain name, type and class    
    fn create_dns_query(domain_name: &str,
                        qtype : &str, 
                        qclass: &str )  -> DnsMessage {

        //Create random generator
        let mut rng = thread_rng();

        // Create query id
        let query_id: u16 = rng.gen();

        //get qtype
        //TODO: funcion que hace match para obtener tipo enum y lo mismo para qclass

        // Create query msg
        // TODO: Cambiar firma
        let query_msg =
            DnsMessage::new_query_message(&domain_name, qtype, qclass, 0, false, query_id);

        query_msg
    }

    ///Sends the query to the resolver in the client
    fn send_query(&self, query_msg: DnsMessage) -> DnsMessage {
        self.conn.send(query_msg)
    }

    /// Create and send dns query and receive response
    pub fn query(&self, domain_name: String, qtype : String, qclass: String) -> DnsMessage {
        let query = create_dns_query(domain_name, qtype, qclass);
        
        let response = send_udp_query(query);

        reponse
    }
}
    

// Getters
impl Client{

}

// Setters
impl Client{

}

///funcion q crea query , funcion q enia y recibe y la otra q hace los dos anteriroes

// pub fn run_client(host_name: &str) {
//     // Start timestamp
//     let now = Instant::now();

//     // Create dns message and send it to the resolver
//     let dns_message_query = create_client_query(host_name, QTYPE, QCLASS);

//     //send query and get response
//     let mut dns_message = send_client_query(TRANSPORT, RESOLVER_IP_PORT, dns_message_query);

//     // Print received values
//     dns_message.print_dns_message();

//     let elapsed = now.elapsed();
//     println!("Elapsed: {:.2?}", elapsed);
// }

// ///Create dns message query
// pub fn create_client_query(host_name: &str, qtype: u16, qclass: u16) -> DnsMessage {
//     // Create random generator
//     let mut rng = thread_rng();

//     // Create query id
//     let query_id: u16 = rng.gen();

//     // Create query msg
//     let query_msg =
//         DnsMessage::new_query_message(host_name.to_string(), qtype, qclass, 0, false, query_id);

//     return query_msg;
// }

// ///Send Dns query  to the resolver
// pub fn send_client_query(
//     transport: &str,
//     resolver_ip_port: &str,
//     query_msg: DnsMessage,
// ) -> DnsMessage {
//     // Create response buffer
//     let mut dns_message = DnsMessage::new();

//     // Send query by UDP
//     if transport == "UDP" {
//         let socket = UdpSocket::bind(CLIENT_IP_PORT).expect("No connection");
//         let msg_to_bytes = query_msg.to_bytes();

//         println!("***resolver*********** {}", resolver_ip_port);
//         match socket.send_to(&msg_to_bytes, resolver_ip_port) {
//             Err(_) => panic!("Error sending query"),
//             Ok(_) => (),
//         }

//         // Hashmap to save incomplete messages
//         let messages = HashMap::<u16, DnsMessage>::new();

//         match socket.set_read_timeout(Some(Duration::from_millis(TIMEOUT * 1000))) {
//             Err(_) => panic!("Error setting read timeout for socket"),
//             Ok(_) => (),
//         }

//         loop {
//             let response_result =
//                 Resolver::receive_udp_msg(socket.try_clone().unwrap(), messages.clone());
//             let messages_len = messages.len();

//             match response_result {
//                 Some(val) => {
//                     dns_message = val.0;

//                     break;
//                 }
//                 None => {
//                     if messages_len == messages.len() {
//                         panic!("Temporary Error");
//                     }
//                     continue;
//                 }
//             }
//         }
//     }

//     // Send query by TCP
//     if transport == "TCP" {
//         let mut stream = TcpStream::connect(resolver_ip_port).expect("No connection");

//         let bytes = query_msg.to_bytes();

//         let msg_length: u16 = bytes.len() as u16;

//         let tcp_bytes_length = [(msg_length >> 8) as u8, msg_length as u8];

//         let full_msg = [&tcp_bytes_length, bytes.as_slice()].concat();

//         match stream.set_read_timeout(Some(Duration::from_millis(TIMEOUT * 1000))) {
//             Err(_) => panic!("Error setting read timeout for socket"),
//             Ok(_) => (),
//         }

//         match stream.write(&full_msg) {
//             Err(_) => panic!("Error: could not write to stream"),
//             Ok(_) => (),
//         }

//         match Resolver::receive_tcp_msg(stream) {
//             Some(val) => {
//                 dns_message = match DnsMessage::from_bytes(&val) {
//                     Ok(msg) => msg,
//                     Err(_) => DnsMessage::format_error_msg(),
//                 };
//             }
//             None => {
//                 panic!("Temporary Error");
//             }
//         }
//     }
//     dns_message
// }


#[cfg(test)]
mod client_test {

    /// Constructor test
    
    /// Query UDP
    
    /// Query TCP
    
    /// Query timeout

    /// Querys with error
    
}