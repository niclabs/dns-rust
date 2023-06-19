pub mod config;
pub mod client_connection;
pub mod tcp_connection;
pub mod udp_connection;

use crate::client::client_connection::ClientConnection;
use crate::message::DnsMessage;

use rand::{thread_rng, Rng};

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
    /// TODO:  
    fn create_dns_query(_domain_name: String,
                        _qtype : String, 
                        _qclass: String )  -> DnsMessage {

        //Create random generator
        let mut rng = thread_rng();

        // Create query id
        let query_id: u16 = rng.gen();

        //get qtype
        //TODO: funcion que hace match para obtener tipo enum y lo mismo para qclass
        let qtype:u16 = 1;
        let qclass:u16 = 1;

        // Create query msg
        // TODO: Cambiar firma
        // let query_msg =
        //     DnsMessage::new_query_message(domain_name, qtype, qclass, 0, false, query_id);

        let query_msg = DnsMessage::new();
        

        query_msg
    }

    ///Sends the query to the resolver in the client
    ///  TODO:  
    fn send_query(&self,query_msg: DnsMessage) -> DnsMessage {
        // self.conn.send(query_msg)

        //FIXME: dummt for no warning
        let dns_query_dummy:DnsMessage = DnsMessage::new();
        return  dns_query_dummy;
    }

    // Create and send dns query and receive response
    // pub fn query(&self, domain_name: String, qtype : String, qclass: String) -> DnsMessage {
    //     let query = create_dns_query(domain_name, qtype, qclass);
        
    //     let response = send_udp_query(query);

    //     reponse
    // }
}
    


#[cfg(test)]
mod client_test {

    use super::{Client, tcp_connection::TCPConnection, client_connection::ClientConnection, udp_connection::UDPConnection};

    #[test]
    fn example_use(){

        let conn_udp:UDPConnection = ClientConnection::new();
        let conn_tcp:TCPConnection = ClientConnection::new();

        // let tcp_client_connection: ClientConnection = TCPConnection;
        let _client_udp = Client::new(conn_udp);
        let _client_tcp = Client::new(conn_tcp);
    }


    // Constructor test
    
    // Query UDP
    
    // Query TCP
    
    // Query timeout

    // Querys with error
    
 

}