pub mod config;
pub mod client_connection;
pub mod tcp_connection;
pub mod udp_connection;

use crate::client::client_connection::ClientConnection;
use crate::message::{DnsMessage, Rtype,Rclass};

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
    fn create_dns_query(&self,
        domain_name: String,
        _qtype : String, 
        _qclass: String )  -> DnsMessage {

        //Create random generator
        let mut rng = thread_rng();

        // Create query id
        let query_id: u16 = rng.gen();

        //get qtype
        // let rtype: Option<Rtype> = Rtype::from_int_to_rtype(qtype);
        //TODO: funcion que hace match para obtener tipo enum y lo mismo para qclass
        let qtype:Rtype = Rtype::A;
        let qclass:Rclass = Rclass::IN;

        // Create query msg
        let query_msg_custome:DnsMessage =
            DnsMessage::new_query_message(domain_name, 
                                            qtype,
                                            qclass, 
                                            0,
                                            false, 
                                            query_id);
        
        query_msg_custome
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

    use crate::message::DnsMessage;

    use super::{Client, tcp_connection::TCPConnection, client_connection::ClientConnection, udp_connection::UDPConnection};

    #[test]
    fn example_use(){
        use std::net::{IpAddr,Ipv4Addr};
        use std::time::Duration;


        //create connection
        let addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        let timeout = Duration::from_secs(100);

;       let conn_udp:UDPConnection = ClientConnection::new(addr,timeout);
        let conn_tcp:TCPConnection = ClientConnection::new(addr,timeout);

        //create client
        let client_udp = Client::new(conn_udp); //se crea un cliente vacio?
        let client_tcp = Client::new(conn_tcp);

        //create query
        let domain_name_udp = String::from("uchile.cl");
        let domain_name_tcp = String::from("uchile.cl");
        let qtype_udp = String::from("A");
        let qtype_tcp = String::from("A");
        let qclass_udp:String = String::from("IN");
        let qclass_tcp:String = String::from("IN");

        let _query_client_udp = client_udp.create_dns_query(domain_name_udp,qtype_udp,qclass_udp);
        let _query_client_tcp = client_tcp.create_dns_query(domain_name_tcp,qtype_tcp,qclass_tcp);
    }

   

    // Constructor test
    
    // Query UDP
    
    // Query TCP
    
    // Query timeout

    // Querys with error
    
 

}