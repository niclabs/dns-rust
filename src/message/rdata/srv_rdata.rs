use crate::domain_name::DomainName;


/// RFC 2782: https://datatracker.ietf.org/doc/html/rfc2782
/// An struct that represents the `Rdata` for srv type.

pub struct SrvRdata {
    /// The priority of this target host. A client MUST attempt to contact the target host with the lowest-numbered priority it can reach; target hosts with the same priority SHOULD be tried in an order defined by the weight field.
    priority: u16,
    /// A server selection mechanism. The weight field specifies a relative weight for entries with the same priority. Larger weights SHOULD be given a proportionately higher probability of being selected.
    weight: u16,
    /// The port on this target host of this service.
    port: u16,
    /// The domain name of the target host.
    target: DomainName,
}

impl SrvRdata {

    /// Creates a new `SrvRdata` with default values.
    /// 
    /// # Example
    /// ```
    /// let srv_rdata = SrvRdata::new();
    /// ```
    pub fn new() -> SrvRdata {
        SrvRdata {
            priority: 0,
            weight: 0,
            port: 0,
            target: DomainName::new(),
        }
    }

    /// Creates a new `SrvRdata` with the specified values.
    /// # Example
    /// ```
    /// let srv_rdata = SrvRdata::new_with_values(1, 2, 3, DomainName::new_from_str("www.example.com"));
    /// ```
    pub fn new_with_values(priority: u16, weight: u16, port: u16, target: DomainName) -> SrvRdata {
        SrvRdata {
            priority,
            weight,
            port,
            target,
        }
    }
}