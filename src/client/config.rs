/* Configuration file for dns client */

pub static RESOLVER_IP_PORT: &'static str = "127.0.0.1:58396";
pub static CLIENT_IP_PORT: &'static str = "192.168.1.89:58397"; // "127.0.0.1:58397";

/* Dns query configuration */

pub static HOST_NAME: &'static str = "cl";
pub static QTYPE: u16 = 1; // searching for RR type "A"
pub static QCLASS: u16 = 1; // class type "IN"
pub static TRANSPORT: &'static str = "TCP";
pub static TIMEOUT: u64 = 10;
