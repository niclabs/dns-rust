use crate::dns_cache::DnsCache;
use crate::message::resource_record::ResourceRecord;

/// This struct represents a resolver query
pub struct ResolverQuery {
    sname: String,
    stype: u16,
    sclass: u16,
    op_code: u8,
    rd: bool,
    slist: Vec<ResourceRecord>,
    cache: DnsCache,
}

impl ResolverQuery {
    /// Creates a new ResolverQuery struct with default values
    ///
    /// # Examples
    /// '''
    /// let resolver_query = ResolverQuery::new();
    ///
    /// assert_eq!(resolver_query.sname, "".to_string());
    /// assert_eq!(resolver_query.stype, 0);
    /// assert_eq!(resolver_query.sclass, 0);
    /// assert_eq!(resolver_query.slist.len(), 0);
    /// assert_eq!(resolver_query.cache.clone().len(), 0);
    /// '''
    ///
    pub fn new() -> Self {
        let query = ResolverQuery {
            sname: "".to_string(),
            stype: 0 as u16,
            sclass: 0 as u16,
            op_code: 0 as u8,
            rd: false,
            slist: Vec::<ResourceRecord>::new(),
            cache: DnsCache::new(),
        };

        query
    }

    /*
    // Creates a new query dns message
    pub fn create_query_message(&self) -> DnsMessage {
        let sname = self.get_sname();
        let stype = self.get_stype();
        let sclass = self.get_sclass();

        let mut query_message = DnsMessage::new_query_message(sname, stype, sclass, op_code, rd);

        query_message
    }
    */
}

// Getters
impl ResolverQuery {
    /// Gets the sname
    pub fn get_sname(&self) -> String {
        self.sname.clone()
    }

    /// Gets the stype
    pub fn get_stype(&self) -> u16 {
        self.stype
    }

    /// Gets the sclass
    pub fn get_sclass(&self) -> u16 {
        self.sclass
    }

    /// Gets the op_code
    pub fn get_op_code(&self) -> u8 {
        self.op_code
    }

    /// Gets the recursion desired bit
    pub fn get_rd(&self) -> bool {
        self.rd
    }

    /// Gets the slist
    pub fn get_slist(&self) -> Vec<ResourceRecord> {
        self.slist.clone()
    }

    /// Gets the cache
    pub fn get_cache(&self) -> DnsCache {
        self.cache.clone()
    }
}

// Setters
impl ResolverQuery {
    /// Sets the sname attribute with a new value
    pub fn set_sname(&mut self, sname: String) {
        self.sname = sname;
    }

    /// Sets the stype attribute with a new value
    pub fn set_stype(&mut self, stype: u16) {
        self.stype = stype;
    }

    /// Sets the sclass attribute with a new value
    pub fn set_sclass(&mut self, sclass: u16) {
        self.sclass = sclass;
    }

    /// Sets the op_code attribute with a new value
    pub fn set_op_code(&mut self, op_code: u8) {
        self.op_code = op_code;
    }

    /// Sets the rd attribute with a new value
    pub fn set_rd(&mut self, rd: bool) {
        self.rd = rd;
    }

    /// Sets the slist attribute with a new value
    pub fn set_slist(&mut self, slist: Vec<ResourceRecord>) {
        self.slist = slist;
    }

    /// Sets the cache attribute with a new value
    pub fn set_cache(&mut self, cache: DnsCache) {
        self.cache = cache;
    }
}

mod test {
    use crate::resolver::resolver_query::ResolverQuery;
    use crate::dns_cache::DnsCache;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;

    #[test]
    fn constructor_test() {
        let resolver_query = ResolverQuery::new();

        assert_eq!(resolver_query.sname, "".to_string());
        assert_eq!(resolver_query.stype, 0);
        assert_eq!(resolver_query.sclass, 0);
        assert_eq!(resolver_query.slist.len(), 0);
        assert_eq!(resolver_query.cache.clone().len(), 0);
    }

    #[test]
    fn set_and_get_sname() {
        let mut resolver_query = ResolverQuery::new();

        assert_eq!(resolver_query.sname, "".to_string());

        resolver_query.set_sname("test.com".to_string());

        assert_eq!(resolver_query.get_sname(), "test.com".to_string());
    }

    #[test]
    fn set_and_get_stype() {
        let mut resolver_query = ResolverQuery::new();

        assert_eq!(resolver_query.stype, 0);

        resolver_query.set_stype(1);

        assert_eq!(resolver_query.get_stype(), 1);
    }

    #[test]
    fn set_and_get_sclass() {
        let mut resolver_query = ResolverQuery::new();

        assert_eq!(resolver_query.sclass, 0);

        resolver_query.set_sclass(1);

        assert_eq!(resolver_query.get_sclass(), 1);
    }

    #[test]
    fn set_and_get_op_code() {
        let mut resolver_query = ResolverQuery::new();

        assert_eq!(resolver_query.op_code, 0);

        resolver_query.set_op_code(1);

        assert_eq!(resolver_query.get_op_code(), 1);
    }

    #[test]
    fn set_and_get_rd() {
        let mut resolver_query = ResolverQuery::new();

        assert_eq!(resolver_query.rd, false);

        resolver_query.set_rd(true);

        assert_eq!(resolver_query.get_rd(), true);
    }

    #[test]
    fn set_and_get_slist() {
        let mut resolver_query = ResolverQuery::new();
        let mut slist: Vec<ResourceRecord> = Vec::new();

        assert_eq!(resolver_query.slist.len(), 0);

        let ip_address: [u8; 4] = [127, 0, 0, 0];
        let mut a_rdata = ARdata::new();

        a_rdata.set_address(ip_address);

        let rdata = Rdata::SomeARdata(a_rdata);
        let resource_record = ResourceRecord::new(rdata);

        slist.push(resource_record);
        resolver_query.set_slist(slist);

        assert_eq!(resolver_query.get_slist().len(), 1);
    }

    #[test]
    fn set_and_get_cache() {
        let mut resolver_query = ResolverQuery::new();
        let mut cache = DnsCache::new();

        assert_eq!(resolver_query.cache.len(), 0);

        let ip_address: [u8; 4] = [127, 0, 0, 0];
        let mut a_rdata = ARdata::new();

        a_rdata.set_address(ip_address);

        let rdata = Rdata::SomeARdata(a_rdata);
        let resource_record = ResourceRecord::new(rdata);

        cache.add("127.0.0.0".to_string(), resource_record);
        resolver_query.set_cache(cache);

        assert_eq!(resolver_query.get_cache().len(), 1);
    }
}
