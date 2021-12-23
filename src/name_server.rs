pub mod master_file;
pub mod zone;

#[derive(Clone)]
/// Structs that represents a name server
pub struct NameServer {
    zones: HashMap<String, NSZone>,
    cache: DnsCache,
}

impl NameServer {
    /// Creates a new name server
    pub fn new() -> Self {
        let name_server = NameServer {
            zones: HashMap::<String, NSZone>::new(),
            cache: DnsCache::new(),
        };

        name_server
    }
}

// Getters
impl NameServer {
    // Gets the zones data from the name server
    pub fn get_zones(&self) -> HashMap<String, NSZone> {
        self.zones.clone()
    }

    // Gets the cache from the name server
    pub fn get_cache(&self) -> DnsCache {
        self.cache.clone()
    }
}

// Setters
impl NameServer {
    // Sets the zones with a new value
    pub fn set_zones(&mut self, zones: HashMap<String, NSZone>) {
        self.zones = zones;
    }

    // Sets the cache with a new cache
    pub fn set_cache(&mut self, cache: DnsCache) {
        self.cache = cache;
    }
}
