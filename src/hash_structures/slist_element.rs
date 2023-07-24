use crate::domain_name::DomainName;
use std::net::{SocketAddr,IpAddr,Ipv4Addr};
/// This struct is used to represente an element of the slist
#[derive(Debug)]
pub struct SlistElement{
    domain_name:  DomainName,
    ip_address: IpAddr,
    response_time: u16,
}

impl SlistElemen{
    /// function to create a new SlistElement
    /// # Example
    /// ```
    /// let slist_element = SlistElement::new(String::from("uchile.cl"), String::from("8.8.8.8"), 0);
    /// 
    /// ```
    /// # Arguments
    /// * `domain_name` - A DomainName that represents the domain name of the slist element
    /// * `ip_address` - A IpAddr that represents the ip address of the slist element
    /// * `response_time` - A u16 that represents the response time of the slist element
    pub fn new(domain_name: DomainName, ip_address: IpAddr, response_time: u16) -> SlistElement{
        SlistElement{
            domain_name: DomainName,
            ip_address: ip_address,
            response_time: response_time,
        }
    }
}
    /// getters and setters for the struct
impl SlistElement{

    pub fn get_domain_name(&self) -> DomainName{
        self.domain_name.clone()
    }

    pub fn get_response_time(&self) -> u16{
        self.response_time.clone()
    }

    pub fn get_ip_address(&self) -> IpAddr{
        self.ip_address.clone()
    }

    pub fn set_domain_name(&mut self, domain_name: DomainName){
        self.domain_name = DomainName
    }

    pub fn set_response_time(&mut self, response_time: u16){
        self.response_time = response_time;
    }

    pub fn set_ip_address(&mut self, ip_address: IpAddr){
        self.ip_address = ip_address;
    }
}

#[cfg(test)]
use std::net::{SocketAddr,IpAddr,Ipv4Addr};

//Constructor Test
fn new_slist_element(){
    let slist_element = SlistElement::new(String::from("uchile.cl"), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4);

    assert_eq!(slist_element.get_domain_name(), String::from("uchile.cl"));
    assert_eq!(Ok(slist_element.get_ip_address()), "127.0.0.1".parse());
    assert_eq!(slist_element.get_response_time(), 4);
}


