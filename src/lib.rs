pub mod client;
pub mod resolver_cache;
pub mod dns_cache;
pub mod domain_name;
pub mod message;
pub mod async_resolver;
pub mod truncated_dns_message;
pub mod tsig;
pub mod dnssec;
pub mod edns{
    pub mod opt_option;
    pub mod options {
        pub mod ede;
        pub mod zoneversion;
    }
}