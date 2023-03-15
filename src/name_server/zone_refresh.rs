use crate::message::rdata::Rdata;
use crate::name_server::zone::NSZone;

use chrono::Utc;

#[derive(Clone, PartialEq, Debug)]
// An struct that represents zone refresh data for a zone
pub struct ZoneRefresh {
    zone: NSZone,
    ip_address_for_refresh_zone: String,
    serial: u32,
    refresh: u32,
    retry: u32,
    expire: u32,
    timestamp: u32,
    last_fails: bool,
    last_serial_check: u32,
}

impl ZoneRefresh {
    pub fn new(zone: NSZone) -> Self {
        let soa_rr = zone.get_zone_nodes().get_rrs_by_type(6)[0].clone();

        let soa_rdata = match soa_rr.get_rdata() {
            Rdata::SomeSoaRdata(val) => val,
            _ => unreachable!(),
        };

        let ip_address_refresh = zone.get_ip_address_for_refresh_zone();
        let now = Utc::now();
        let timestamp = now.timestamp() as u32;

        let zone_refresh = ZoneRefresh {
            zone: zone,
            ip_address_for_refresh_zone: ip_address_refresh,
            serial: soa_rdata.get_serial(),
            refresh: soa_rdata.get_refresh(),
            retry: soa_rdata.get_retry(),
            expire: soa_rdata.get_expire(),
            timestamp: timestamp,
            last_fails: false,
            last_serial_check: timestamp,
        };

        zone_refresh
    }
    /// Compares the new serial with the old one, by Serial number arithmetic
    // https://en.wikipedia.org/wiki/Serial_number_arithmetic
    pub fn new_serial_greater_than_old(&self, serial: u32) -> bool {
        let old_serial = self.get_serial();

        if (serial < old_serial && old_serial - serial > 2 ^ (32 - 1))
            || (serial > old_serial && serial - old_serial < 2 ^ (32 - 1))
        {
            return true;
        } else {
            return false;
        }
    }

    ///Update itself with the values of the SOA RR in the received NSZone
    pub fn update_zone_refresh(&mut self, zone: NSZone) {
        let soa_rr = zone.get_zone_nodes().get_rrs_by_type(6)[0].clone();
        let soa_rdata = match soa_rr.get_rdata() {
            Rdata::SomeSoaRdata(val) => val,
            _ => unreachable!(),
        };

        let serial = soa_rdata.get_serial();
        let refresh = soa_rdata.get_refresh();
        let retry = soa_rdata.get_retry();
        let expire = soa_rdata.get_expire();
        let now = Utc::now();
        let now_timestamp = now.timestamp() as u32;

        self.set_serial(serial);
        self.set_refresh(refresh);
        self.set_retry(retry);
        self.set_expire(expire);
        self.set_zone(zone);
        self.set_last_fails(false);
        self.set_timestamp(now_timestamp);
    }
}

// Setters
impl ZoneRefresh {
    pub fn set_zone(&mut self, zone: NSZone) {
        self.zone = zone;
    }

    pub fn set_ip_address_for_refresh_zone(&mut self, ip_address_for_refresh_zone: String) {
        self.ip_address_for_refresh_zone = ip_address_for_refresh_zone;
    }

    pub fn set_serial(&mut self, serial: u32) {
        self.serial = serial;
    }

    pub fn set_refresh(&mut self, refresh: u32) {
        self.refresh = refresh;
    }

    pub fn set_retry(&mut self, retry: u32) {
        self.retry = retry;
    }

    pub fn set_expire(&mut self, expire: u32) {
        self.expire = expire;
    }

    pub fn set_timestamp(&mut self, timestamp: u32) {
        self.timestamp = timestamp;
    }

    pub fn set_last_fails(&mut self, last_fails: bool) {
        self.last_fails = last_fails;
    }

    pub fn set_last_serial_check(&mut self, last_serial_check: u32) {
        self.last_serial_check = last_serial_check;
    }
}

// Getters
impl ZoneRefresh {
    pub fn get_zone(&self) -> NSZone {
        self.zone.clone()
    }

    pub fn get_ip_address_for_refresh_zone(&self) -> String {
        self.ip_address_for_refresh_zone.clone()
    }

    pub fn get_serial(&self) -> u32 {
        self.serial
    }

    pub fn get_refresh(&self) -> u32 {
        self.refresh
    }

    pub fn get_retry(&self) -> u32 {
        self.retry
    }

    pub fn get_expire(&self) -> u32 {
        self.expire
    }

    pub fn get_timestamp(&self) -> u32 {
        self.timestamp
    }

    pub fn get_last_fails(&self) -> bool {
        self.last_fails
    }

    pub fn get_last_serial_check(&self) -> u32 {
        self.last_serial_check
    }
}

#[cfg(test)]
mod zone_refresh_test {

    use super::ZoneRefresh;
    use crate::domain_name::DomainName;
    use crate::message::rdata::soa_rdata::SoaRdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;
    use crate::name_server::zone::NSZone;

    use chrono::Utc;

    #[test]
    //TODO revisar práctica 1
    fn constructor() {
        let mut ns_zone = NSZone::new();
        let name = String::from("example.com");
        ns_zone.set_name(name);
        let ip = String::from("200.89.76.36");
        ns_zone.set_ip_address_for_refresh_zone(ip);

        let mut value = Vec::<ResourceRecord>::new();
        let mut soa_rdata = Rdata::SomeSoaRdata(SoaRdata::new());
        let mut mname_domain_name = DomainName::new();
        let domain_name_name = String::from("ns.primaryserver.com");
        mname_domain_name.set_name(domain_name_name);
        let mut rname_domain_name = DomainName::new();
        let rname_name = String::from("admin.example.com");
        rname_domain_name.set_name(rname_name);
        match soa_rdata {
            Rdata::SomeSoaRdata(ref mut val) => {
                val.set_mname(mname_domain_name);
                val.set_rname(rname_domain_name);
                val.set_serial(1111111111 as u32)
            }
            _ => unreachable!(),
        }
        let resource_record = ResourceRecord::new(soa_rdata);
        value.push(resource_record);

        // added to fix the initialization in all tests
        let mut top_node = ns_zone.get_zone_nodes();
        top_node.set_value(value);
        ns_zone.set_zone_nodes(top_node);
        // fails when tries to initialize zone refresh(fixed)

        let zone_refresh = ZoneRefresh::new(ns_zone);
        let some_timestamp = Utc::now().timestamp() as u32;
        let expected_name = String::from("example.com");
        assert_eq!(zone_refresh.zone.get_name(), expected_name);
        let expected_ip = String::from("200.89.76.36");

        assert_eq!(zone_refresh.ip_address_for_refresh_zone, expected_ip);
        assert_eq!(zone_refresh.serial, 1111111111 as u32);
        assert_eq!(zone_refresh.refresh, 0 as u32);
        assert_eq!(zone_refresh.retry, 0 as u32);
        assert_eq!(zone_refresh.expire, 0 as u32);
        assert_eq!(zone_refresh.timestamp, some_timestamp);
        assert_eq!(zone_refresh.last_fails, false);
        assert_eq!(zone_refresh.last_serial_check, some_timestamp);
    }

    #[test]
    fn set_and_get_zone() {
        let mut ns_zone_1 = NSZone::new();
        let origin = String::from("example.com");
        ns_zone_1.set_name(origin);
        let ns_zone_2 = NSZone::new();

        let mut value = Vec::<ResourceRecord>::new();
        let soa_rdata = Rdata::SomeSoaRdata(SoaRdata::new());
        let resource_record = ResourceRecord::new(soa_rdata);
        value.push(resource_record);

        ns_zone_1.get_zone_nodes().set_value(value.clone());
        ns_zone_2.get_zone_nodes().set_value(value.clone());
        let mut top_node = ns_zone_1.get_zone_nodes();
        top_node.set_value(value);
        ns_zone_1.set_zone_nodes(top_node);

        let mut zone_refresh = ZoneRefresh::new(ns_zone_1);
        let expected_name1 = String::from("example.com");
        let real_name1 = zone_refresh.get_zone().get_name();
        assert_eq!(real_name1, expected_name1);
        
        zone_refresh.set_zone(ns_zone_2);
        let expected_name2 = String::from("");
        let real_name2 = zone_refresh.get_zone().get_name();
        assert_eq!(real_name2, expected_name2);
    }

    #[test]
    fn set_and_get_ip_address_for_refresh_zone() {
        let mut ns_zone = NSZone::new();
        let mut value = Vec::<ResourceRecord>::new();

        let soa_rdata = Rdata::SomeSoaRdata(SoaRdata::new());
        let resource_record = ResourceRecord::new(soa_rdata);
        value.push(resource_record);
        ns_zone.get_zone_nodes().set_value(value.clone());
        let mut top_node = ns_zone.get_zone_nodes();
        top_node.set_value(value);
        ns_zone.set_zone_nodes(top_node);

        let mut zone_refresh = ZoneRefresh::new(ns_zone);
        let mut real_ip = zone_refresh.get_ip_address_for_refresh_zone();
        assert_eq!(real_ip, String::from(""));

        let ip = String::from("200.89.76.36");
        zone_refresh.set_ip_address_for_refresh_zone(ip.clone());
        real_ip = zone_refresh.get_ip_address_for_refresh_zone();
        assert_eq!(real_ip, ip);
    }

    #[test]
    fn set_and_get_serial() {
        let mut ns_zone = NSZone::new();
        let mut value = Vec::<ResourceRecord>::new();
        
        let soa_rdata = Rdata::SomeSoaRdata(SoaRdata::new());
        let resource_record = ResourceRecord::new(soa_rdata);
        value.push(resource_record);
        ns_zone.get_zone_nodes().set_value(value.clone());
        let mut top_node = ns_zone.get_zone_nodes();
        top_node.set_value(value);
        ns_zone.set_zone_nodes(top_node);

        let mut zone_refresh = ZoneRefresh::new(ns_zone);
        let zero = 0 as u32;
        assert_eq!(zone_refresh.get_serial(), zero);

        let number = 1111111111 as u32;
        zone_refresh.set_serial(number.clone());
        assert_eq!(zone_refresh.get_serial(), number);
    }

    #[test]
    fn set_and_get_refresh() {
        let mut ns_zone = NSZone::new();
        let mut value = Vec::<ResourceRecord>::new();
        let soa_rdata = Rdata::SomeSoaRdata(SoaRdata::new());
        let resource_record = ResourceRecord::new(soa_rdata);
        value.push(resource_record);

        ns_zone.get_zone_nodes().set_value(value.clone());
        let mut top_node = ns_zone.get_zone_nodes();
        top_node.set_value(value);
        ns_zone.set_zone_nodes(top_node);

        let mut zone_refresh = ZoneRefresh::new(ns_zone);
        let zero = 0 as u32;
        assert_eq!(zone_refresh.get_refresh(), zero);

        let number = 86400 as u32;
        zone_refresh.set_refresh(number.clone());
        assert_eq!(zone_refresh.get_refresh(), number);
    }

    #[test]
    fn set_and_get_retry() {
        let mut ns_zone = NSZone::new();
        let mut value = Vec::<ResourceRecord>::new();
        let soa_rdata = Rdata::SomeSoaRdata(SoaRdata::new());
        let resource_record = ResourceRecord::new(soa_rdata);
        value.push(resource_record);

        ns_zone.get_zone_nodes().set_value(value.clone());
        let mut top_node = ns_zone.get_zone_nodes();
        top_node.set_value(value);
        ns_zone.set_zone_nodes(top_node);

        let mut zone_refresh = ZoneRefresh::new(ns_zone);
        let zero = 0 as u32;
        assert_eq!(zone_refresh.get_retry(), zero);

        let number = 7200 as u32;
        zone_refresh.set_retry(number.clone());
        assert_eq!(zone_refresh.get_retry(), number);
    }

    #[test]
    fn set_and_get_expire() {
        let mut ns_zone = NSZone::new();
        let mut value = Vec::<ResourceRecord>::new();
        let soa_rdata = Rdata::SomeSoaRdata(SoaRdata::new());
        let resource_record = ResourceRecord::new(soa_rdata);
        value.push(resource_record);

        ns_zone.get_zone_nodes().set_value(value.clone());
        let mut top_node = ns_zone.get_zone_nodes();
        top_node.set_value(value);
        ns_zone.set_zone_nodes(top_node);

        let mut zone_refresh = ZoneRefresh::new(ns_zone);
        let zero = 0 as u32;
        assert_eq!(zone_refresh.get_expire(), zero);
        
        let number = 4000000 as u32;
        zone_refresh.set_expire(number.clone());
        assert_eq!(zone_refresh.get_expire(), number);
    }

    #[test]
    fn set_and_get_timestamp() {
        let mut ns_zone = NSZone::new();
        let mut value = Vec::<ResourceRecord>::new();
        let soa_rdata = Rdata::SomeSoaRdata(SoaRdata::new());
        let resource_record = ResourceRecord::new(soa_rdata);
        value.push(resource_record);

        ns_zone.get_zone_nodes().set_value(value.clone());
        let mut top_node = ns_zone.get_zone_nodes();
        top_node.set_value(value);
        ns_zone.set_zone_nodes(top_node);

        let mut zone_refresh = ZoneRefresh::new(ns_zone);
        let some_timestamp = Utc::now().timestamp() as u32;
        assert_eq!(zone_refresh.get_timestamp(), some_timestamp);

        let another_timestamp = some_timestamp - 1;
        zone_refresh.set_timestamp(another_timestamp.clone());
        assert_eq!(zone_refresh.get_timestamp(), another_timestamp);
    }

    #[test]
    fn set_and_get_last_fails() {
        let mut ns_zone = NSZone::new();
        let mut value = Vec::<ResourceRecord>::new();

        let soa_rdata = Rdata::SomeSoaRdata(SoaRdata::new());
        let resource_record = ResourceRecord::new(soa_rdata);
        value.push(resource_record);
        ns_zone.get_zone_nodes().set_value(value.clone());
        let mut top_node = ns_zone.get_zone_nodes();
        top_node.set_value(value);
        ns_zone.set_zone_nodes(top_node);

        let mut zone_refresh = ZoneRefresh::new(ns_zone);
        assert_eq!(zone_refresh.get_last_fails(), false);

        zone_refresh.set_last_fails(true);
        assert_eq!(zone_refresh.get_last_fails(), true);
    }

    #[test]
    fn set_and_get_last_serial_check() {
        let mut ns_zone = NSZone::new();
        let mut value = Vec::<ResourceRecord>::new();

        let soa_rdata = Rdata::SomeSoaRdata(SoaRdata::new());
        let resource_record = ResourceRecord::new(soa_rdata);
        value.push(resource_record);
        ns_zone.get_zone_nodes().set_value(value.clone());
        let mut top_node = ns_zone.get_zone_nodes();
        top_node.set_value(value);
        ns_zone.set_zone_nodes(top_node);

        let mut zone_refresh = ZoneRefresh::new(ns_zone);
        let some_timestamp = Utc::now().timestamp() as u32;
        assert_eq!(zone_refresh.get_last_serial_check(), some_timestamp);

        let another_timestamp = some_timestamp - 1;
        zone_refresh.set_last_serial_check(another_timestamp.clone());
        assert_eq!(zone_refresh.get_last_serial_check(), another_timestamp);
    }

    #[test]
    fn new_serial_greater_than_old() {
        let mut ns_zone = NSZone::new();
        let mut value = Vec::<ResourceRecord>::new();
        let soa_rdata = Rdata::SomeSoaRdata(SoaRdata::new());
        let resource_record = ResourceRecord::new(soa_rdata);

        value.push(resource_record);
        ns_zone.get_zone_nodes().set_value(value.clone());
        let mut top_node = ns_zone.get_zone_nodes();
        top_node.set_value(value);
        ns_zone.set_zone_nodes(top_node);
        let mut zone_refresh = ZoneRefresh::new(ns_zone);

        zone_refresh.set_serial(111111111 as u32);
        assert_eq!(
            zone_refresh.new_serial_greater_than_old(4294967295 as u32),
            false
        );
        assert_eq!(
            zone_refresh.new_serial_greater_than_old(111111112 as u32),
            true
        );
        assert_eq!(
            zone_refresh.new_serial_greater_than_old(111111110 as u32),
            false
        );
        zone_refresh.set_serial(4294967295 as u32);
        assert_eq!(
            zone_refresh.new_serial_greater_than_old(111111111 as u32),
            true
        );
    }

    #[test]
    //TODO revisar práctica 1
    fn update_zone() {
        let mut ns_zone = NSZone::new();
        let mut value_1 = Vec::<ResourceRecord>::new();
        let mut value_2 = Vec::<ResourceRecord>::new();
        let soa_rdata_1 = Rdata::SomeSoaRdata(SoaRdata::new());
        let resource_record_1 = ResourceRecord::new(soa_rdata_1);
        let mut soa_rdata_2 = Rdata::SomeSoaRdata(SoaRdata::new());
        match soa_rdata_2 {
            Rdata::SomeSoaRdata(ref mut val) => {
                val.set_expire(4000000 as u32);
                val.set_retry(7200 as u32);
                val.set_serial(1111111111 as u32)
            }
            _ => unreachable!(),
        }
        let resource_record_2 = ResourceRecord::new(soa_rdata_2);
        value_1.push(resource_record_1);
        value_2.push(resource_record_2);

        ns_zone.get_zone_nodes().set_value(value_1.clone());
        let mut top_node1 = ns_zone.get_zone_nodes();
        top_node1.set_value(value_1);
        ns_zone.set_zone_nodes(top_node1);

        let mut zone_refresh = ZoneRefresh::new(ns_zone.clone());
        assert_eq!(zone_refresh.get_serial(), 0 as u32);
        assert_eq!(zone_refresh.get_retry(), 0 as u32);
        assert_eq!(zone_refresh.get_expire(), 0 as u32);

        ns_zone.get_zone_nodes().set_value(value_2.clone());
        let mut top_node2 = ns_zone.get_zone_nodes();
        top_node2.set_value(value_2);
        ns_zone.set_zone_nodes(top_node2);
        zone_refresh.update_zone_refresh(ns_zone.clone());

        assert_eq!(zone_refresh.get_serial(), 1111111111 as u32);
        assert_eq!(zone_refresh.get_retry(), 7200 as u32);
        assert_eq!(zone_refresh.get_expire(), 4000000 as u32);
    }

    #[test]
    fn update_zone_refresh_same_data() {
        let mut ns_zone = NSZone::new();
        let mut rr_value = Vec::<ResourceRecord>::new();
        let soa_rdata = Rdata::SomeSoaRdata(SoaRdata::new());
        let resource_record = ResourceRecord::new(soa_rdata);
        
        rr_value.push(resource_record);

        ns_zone.get_zone_nodes().set_value(rr_value.clone());
        let mut top_node = ns_zone.get_zone_nodes();
        top_node.set_value(rr_value);
        ns_zone.set_zone_nodes(top_node);

        let mut zone_refresh = ZoneRefresh::new(ns_zone.clone());
        let zero = 0 as u32;
        assert_eq!(zone_refresh.get_serial(), zero.clone());
        assert_eq!(zone_refresh.get_retry(), zero.clone());
        assert_eq!(zone_refresh.get_expire(), zero.clone());

        let serial_init_state = zone_refresh.get_serial();
        let retry_init_state = zone_refresh.get_retry();
        let expire_init_state = zone_refresh.get_expire();

        //add nothing
        zone_refresh.update_zone_refresh(ns_zone.clone());

        assert_eq!(zone_refresh.get_serial(), serial_init_state);
        assert_eq!(zone_refresh.get_retry(), retry_init_state);
        assert_eq!(zone_refresh.get_expire(), expire_init_state);
    }
}
