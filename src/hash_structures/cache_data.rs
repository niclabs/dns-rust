pub mod type_rtype;

use crate::message::type_rtype::Rtype;
use crate::rr_cache::RRCache;
use crate::host_data;
use std::collections::HashMap;

///type to define the rtype of the cache data
type rtype = Rtype;

///type to denine the host data
type host_data = HostData;

///struct to define the cache data
#[derive(Clone)]
pub struct CacheData {
    pub cache_hash: HashMap<rtype, host_data>,
}

/// functions for the cache data