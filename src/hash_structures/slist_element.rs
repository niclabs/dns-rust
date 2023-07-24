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
mod slist_element_test{
    use std::net::{SocketAddr,IpAddr,Ipv4Addr};
    
    //Constructor Test
    fn new_slist_element(){
        let domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let slist_element = SlistElement::new(domain_name, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4);

        assert_eq!(slist_element.get_domain_name().get_name(), String::from("uchile.cl"));
        assert_eq!(Ok(slist_element.get_ip_address()), "127.0.0.1".parse());
        assert_eq!(slist_element.get_response_time(), 4);
    }

    //Getters and Setters Test
    fn get_address(){
        let domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let slist_element = SlistElement::new(domain_name, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4);

        let address = slist_element.get_ip_address();

        assert_eq!(Ok(address), "127.0.0.1".parse());
    }

    fn set_ip_address(){
        let domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let slist_element = SlistElement::new(domain_name, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4);

        assert_eq!(Ok(slist_element.get_address()), "127.0.0.1".parse());

        slist_element.set_ip_address(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));

        assert_eq!(Ok(slist_element.get_ip_address()), "192.168.0.1".parse());
    }

    fn get_response_time(){
        let domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let slist_element = SlistElement::new(domain_name, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4);

        assert_eq!(slist_element.get_response_time(), 4);
    }

    fn set_response_time(){
        let domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let slist_element = SlistElement::new(domain_name, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4);

        assert_eq!(slist_element.get_response_time(), 4);

        slist_element.set_response_time(5);

        assert_eq!(slist_element.get_response_time(), 5);
    }

    fn get_domain_name(){
        let domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let slist_element = SlistElement::new(domain_name, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4);

        assert_eq!(slist_element.get_domain_name().get_name(), String::from("uchile.cl"));
    }

    fn set_domain_name(){
        let domain_name = DomainName::new();
        domain_name.set_name(String::from("uchile.cl"));
        let slist_element = SlistElement::new(domain_name, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4);

        assert_eq!(slist_element.get_domain_name().get_name(), String::from("uchile.cl"));

        let domain_name_2 = DomainName::new();
        domain_name_2.set_name(String::from("google.com"));
        slist_element.set_domain_name(domain_name_2);

        assert_eq!(slist_element.get_domain_name().get_name(), String::from("google.com"));
    }
}
