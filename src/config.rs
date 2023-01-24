/* Configuration file for dns */

/* ----------- Options ---------------------*/
// CacheMaxSize: max size for the cache (in number of domain names saved)

// ------------- Resolver Config --------------------
pub static RESOLVER_IP_PORT: &'static str = "127.0.0.1:58396";

// Add at least 2 root servers and 2 host server (for local network).
pub static SBELT_ROOT_IPS: [&str;3] = ["198.41.0.4", "199.9.14.201", "192.33.4.12"];

// Queries quantity for each query, before the resolver panic in a Temporary Error
pub static QUERIES_FOR_CLIENT_REQUEST: u16 = 50;
// Uses cache or not
pub static USE_CACHE: bool = true;
// --------------------------------------------------

// ------------- NameServer Config -------------------
pub static NAME_SERVER_IP: &'static str = "127.0.0.1"; // "192.168.1.89"
pub static MASTER_FILES: [(&str,&str );1] = [("1034-scenario-6.1-root.txt", "")];
pub static RECURSIVE_AVAILABLE: bool = true; // recursive name server available as default
                                             // ---------------------------------------------------
pub static CHECK_MASTER_FILES: bool = true; // checks validity of master files as default