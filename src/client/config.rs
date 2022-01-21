/* Configuration file for dns client */

pub static RESOLVER_IP_PORT: &'static str = "192.168.1.89:58396";
pub static CLIENT_IP_PORT: &'static str = "192.168.1.89:58397";

/* Dns query configuration */

pub static HOST_NAME: &'static str = "dcc.uchile.cl";
pub static QTYPE: u16 = 2;
pub static QCLASS: u16 = 1;
pub static TRANSPORT: &'static str = "UDP";
pub static TIMEOUT: u64 = 5;
pub static RECURSIVE_MODE = true; // default is recursive ?
