
/// This struct is used to represente an element of the slist
#[derive(Debug)]
pub struct SlistElement{
    domain_name: String,
    ip_address: String,
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
    /// * `domain_name` - A String that represents the domain name of the slist element
    /// * `ip_address` - A String that represents the ip address of the slist element
    /// * `response_time` - A u16 that represents the response time of the slist element
    pub fn new(domain_name: String, ip_address: String, response_time: u16) -> SlistElement{
        SlistElement{
            domain_name: domain_name,
            ip_address: ip_address,
            response_time: response_time,
        }
    }
}
    /// getters and setters for the struct
impl SlistElement{

    pub fn get_domain_name(&self) -> String{
        self.domain_name.clone()
    }

    pub fn get_response_time(&self) -> u16{
        self.response_time.clone()
    }

    pub fn get_ip_address(&self) -> String{
        self.ip_address.clone()
    }

    pub fn set_domain_name(&mut self, domain_name: String){
        self.domain_name = domain_name;
    }

    pub fn set_response_time(&mut self, response_time: u16){
        self.response_time = response_time;
    }

    pub fn set_ip_address(&mut self, ip_address: String){
        self.ip_address = ip_address;
    }
}

