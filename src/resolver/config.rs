use crate::resolver::slist::Slist;

use std::{net::{IpAddr,SocketAddr,Ipv4Addr, UdpSocket}, default};

pub struct ResolverConfig{
    //Servers
    sbelt: Slist,
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
    pub fn new(sbelt: Option<Slist>, resolver_addr: IpAddr) -> Self {
        let resolver_config: ResolverConfig = ResolverConfig {
            sbelt: sbelt.unwrap_or_else(Slist::new),
            addr: SocketAddr::new(resolver_addr, 53),
            retry: 30,
            cache_available: true,
            recursive_available: false,
        };

        resolver_config
    }

    pub fn default()-> Self {
        //TODO: crea un resolver con los valores por defecto
        let resolver_config: ResolverConfig = ResolverConfig {
            sbelt: Slist::new(),
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5333),
            retry: 30,
            cache_available: true,
            recursive_available: false
        };

        resolver_config
    }

}

///Getters
impl ResolverConfig {

    pub fn get_sbelt(&self) -> &Slist {
        &self.sbelt
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

    pub fn set_sbelt(&mut self,sbelt: Slist ) {
        self.sbelt = sbelt;
    }

    pub fn set_Addr(&mut self,addr:SocketAddr){
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
    use std::net::{IpAddr, Ipv4Addr};
    use crate::resolver::slist::Slist;
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn example() {
        let resolver_addr: IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));

        let addr1: IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 2));
        let addr2: IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 3));
        let addr3: IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 4));

        let mut sbelt = Slist::new();
        let mut ns_list = Vec::new();
        let servers = HashMap::new();
        // TODO: agregar server al hashmap
        ns_list.push(servers);

        sbelt.set_ns_list(ns_list);

        let mut config = ResolverConfig::new(Some(sbelt),resolver_addr);

        //config default
        let config_default = ResolverConfig::default();
        


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
