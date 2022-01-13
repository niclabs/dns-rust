use crate::message::rdata::Rdata;
use crate::name_server::zone::NSZone;

use chrono::Utc;

#[derive(Clone)]
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
}

impl ZoneRefresh {
    pub fn new(zone: NSZone) -> Self {
        let soa_rr = zone.get_rrs_by_type(6)[0].clone();
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
        };

        zone_refresh
    }
    // Compares the new serial with the old one, by Serial number arithmetic
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

    pub fn update_zone(&mut self, zone: NSZone) {
        let soa_rr = zone.get_rrs_by_type(6)[0].clone();
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

    pub fn get_refresh(&mut self) -> u32 {
        self.refresh
    }

    pub fn get_retry(&mut self) -> u32 {
        self.retry
    }

    pub fn get_expire(&mut self) -> u32 {
        self.expire
    }

    pub fn get_timestamp(&mut self) -> u32 {
        self.timestamp
    }

    pub fn get_last_fails(&mut self) -> bool {
        self.last_fails
    }
}
