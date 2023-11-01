use crate::client::{udp_connection::ClientUDPConnection, tcp_connection::ClientTCPConnection,client_connection::ClientConnection };

use std::{net::{IpAddr,SocketAddr,Ipv4Addr}, time::Duration, vec};

pub struct ResolverConfig {
    //Servers
    name_servers: Vec<(ClientUDPConnection,ClientTCPConnection)>,
    //Addres of resolver
    addr: SocketAddr,
    //Queries quantity for each query, before the resolver panic in a Temporary Error
    retry: u16,
    //Uses cache or not
    cache_available: bool,
    //Uses recursive 
    recursive_available: bool,
}

impl ResolverConfig {
    pub fn new(resolver_addr: IpAddr) -> Self {
        let resolver_config: ResolverConfig = ResolverConfig {
            name_servers: Vec::new(),
            addr: SocketAddr::new(resolver_addr, 53),
            retry: 30,
            cache_available: true,
            recursive_available: false,
        };

        resolver_config
    }

    pub fn default()-> Self {

        //FIXME: these are examples values
        let google_server:IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        let timeout: Duration = Duration::from_secs(20);

        let conn_udp:ClientUDPConnection = ClientUDPConnection::new(google_server, timeout);
        let conn_tcp:ClientTCPConnection = ClientTCPConnection::new(google_server, timeout);
    
        let resolver_config: ResolverConfig = ResolverConfig {
            name_servers: vec![(conn_udp,conn_tcp)],
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5333),
            retry: 30,
            cache_available: true,
            recursive_available: false,
        };

        resolver_config
    }

}

///Getters
impl ResolverConfig {

    pub fn get_name_servers(&self) -> Vec<(ClientUDPConnection,ClientTCPConnection)>{
        self.name_servers.clone()
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn get_retry(&self) -> u16{
        self.retry
    }

    pub fn get_cache_available(&self) -> bool{
        self.cache_available 
    }

    pub fn get_recursive_available(&self) -> bool{
        self.recursive_available
    }

}

///Setters
impl ResolverConfig{

    pub fn set_name_servers(&mut self,list_name_servers: Vec<(ClientUDPConnection,ClientTCPConnection)>) {
        self.name_servers = list_name_servers;
    }

    pub fn set_ddr(&mut self,addr:SocketAddr){
        self.addr = addr;
    }

    pub fn set_retry(&mut self, retry:u16){
        self.retry = retry;
    }

    pub fn set_cache_available(&mut self, cache_available:bool){
        self.cache_available = cache_available;
    }

    pub fn set_recursive_available(&mut self,recursive_available:bool){
        self.recursive_available = recursive_available;
    }

}


#[cfg(test)]
mod tests_resolver_config {
    //use std::net::{IpAddr, Ipv4Addr};
    //use crate::domain_name::DomainName;
    //use crate::resolver::slist::Slist;
    //use crate::resolver::slist::slist_element::SlistElement;
    // use std::collections::HashMap;
    //use super::*;

    #[test]
    fn example() {
    //    let resolver_addr: IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));

     //   let addr1: IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 2));
       // let addr2: IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 3));
        //let addr3: IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 4));

        //let mut sbelt = Slist::new();
        //let mut ns_list = Vec::new();

        //let mut name = DomainName::new();
        //name.set_name(String::from("VENERA.ISI.EDU"));
        //let ip_address = IpAddr::V4(Ipv4Addr::new(128, 9, 0, 32));
        //let response_time = 5000;

        //let servers = SlistElement::new(name.clone(), ip_address.clone(), response_time.clone());
        // TODO: agregar server al hashmap
        //ns_list.push(servers);

        //sbelt.set_ns_list(ns_list);

        //let google_server:IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
        //let timeout: Duration = Duration::from_secs(20);
        //let type_conn = ClientUDPConnection::new(google_server, timeout);
       // let conn = ClientConnectionType::UDP(type_conn);
        
       // let mut config = ResolverConfig::new(Some(sbelt),resolver_addr,conn);

        //config default
        //let config_default = ResolverConfig::default();
        


        // config.set_Addr(SocketAddr::new(addr1, 53));
        // config.set_retry(10);
        // config.set_cache_available(false);
        // config.set_recursive_available(true);

        // assert_eq!(config.get_sbelt().get_ns_list(), vec![servers]);
        // assert_eq!(config.get_addr(), SocketAddr::new(addr1, 53));
        // assert_eq!(config.get_retry(), 10);
        // assert_eq!(config.get_cache_available(), false);
        // assert_eq!(config.get_recursive_available(), true);
    }

    //Test no se le pasa un sbelt
}
