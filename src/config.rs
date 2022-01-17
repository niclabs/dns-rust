/* Configuration file for dns resolver */

/* ----------- Options ---------------------*/
// Resolver: client or server side
// Transport : TCP or UDP
// SLIST : use or not
// CacheMaxSize: max size for the cache (in number of domain names saved)

pub static RESOLVER_IP_PORT: &'static str = "192.168.1.89:58396";
pub static NAME_SERVER_IP: &'static str = "192.168.1.89";
pub static QUERIES_FOR_CLIENT_REQUEST: u16 = 20;

pub static SBELT_ROOT_IPS: [&str; 3] = ["198.41.0.4", "199.9.14.201", "192.33.4.12"];

pub static MASTER_FILES: [&str; 1] = ["test.txt"];
