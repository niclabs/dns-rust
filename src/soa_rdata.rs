use crate::domain_name::DomainName;
use crate::resource_record::{FromBytes, ToBytes};

#[derive(Clone)]
/// An struct that represents the rdata for soa type
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// /                     MNAME                     /
/// /                                               /
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// /                     RNAME                     /
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    SERIAL                     |
/// |                                               |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    REFRESH                    |
/// |                                               |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                     RETRY                     |
/// |                                               |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    EXPIRE                     |
/// |                                               |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    MINIMUM                    |
/// |                                               |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///
pub struct SoaRdata {
    // DomainName of the name server that was the
    // original or primary source of data for this zone.
    mname: DomainName,

    // A DomainName which specifies the mailbox of the
    // person responsible for this zone.
    rname: DomainName,

    // The unsigned 32 bit version number of the original copy
    // of the zone.  Zone transfers preserve this value.  This
    // value wraps and should be compared using sequence space
    // arithmetic.
    serial: u32,

    // A 32 bit time interval before the zone should be
    // refreshed.
    refresh: u32,

    // A 32 bit time interval that should elapse before a
    // failed refresh should be retried.
    retry: u32,

    // A 32 bit time value that specifies the upper limit on
    // the time interval that can elapse before the zone is no
    // longer authoritative.
    expire: u32,

    // The unsigned 32 bit minimum TTL field that should be
    // exported with any RR from this zone.
    minimum: u32,
}

impl ToBytes for SoaRdata {
    /// Return a vec of bytes that represents the soa rdata
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        let mname_bytes = self.get_mname().to_bytes();

        for byte in mname_bytes.as_slice() {
            bytes.push(*byte);
        }

        let rname_bytes = self.get_rname().to_bytes();

        for byte in rname_bytes.as_slice() {
            bytes.push(*byte);
        }

        // Serial
        let serial_first_byte = self.get_first_serial_byte();
        bytes.push(serial_first_byte);

        let serial_second_byte = self.get_second_serial_byte();
        bytes.push(serial_second_byte);

        let serial_third_byte = self.get_third_serial_byte();
        bytes.push(serial_third_byte);

        let serial_fourth_byte = self.get_fourth_serial_byte();
        bytes.push(serial_fourth_byte);

        // Refresh
        let refresh_first_byte = self.get_first_refresh_byte();
        bytes.push(refresh_first_byte);

        let refresh_second_byte = self.get_second_refresh_byte();
        bytes.push(refresh_second_byte);

        let refresh_third_byte = self.get_third_refresh_byte();
        bytes.push(refresh_third_byte);

        let refresh_fourth_byte = self.get_fourth_refresh_byte();
        bytes.push(refresh_fourth_byte);

        // Retry
        let retry_first_byte = self.get_first_retry_byte();
        bytes.push(retry_first_byte);

        let retry_second_byte = self.get_second_retry_byte();
        bytes.push(retry_second_byte);

        let retry_third_byte = self.get_third_retry_byte();
        bytes.push(retry_third_byte);

        let retry_fourth_byte = self.get_fourth_retry_byte();
        bytes.push(retry_fourth_byte);

        // Expire
        let expire_first_byte = self.get_first_expire_byte();
        bytes.push(expire_first_byte);

        let expire_second_byte = self.get_second_expire_byte();
        bytes.push(expire_second_byte);

        let expire_third_byte = self.get_third_expire_byte();
        bytes.push(expire_third_byte);

        let expire_fourth_byte = self.get_fourth_expire_byte();
        bytes.push(expire_fourth_byte);

        // Minimum
        let minimum_first_byte = self.get_first_minimum_byte();
        bytes.push(minimum_first_byte);

        let minimum_second_byte = self.get_second_minimum_byte();
        bytes.push(minimum_second_byte);

        let minimum_third_byte = self.get_third_minimum_byte();
        bytes.push(minimum_third_byte);

        let minimum_fourth_byte = self.get_fourth_minimum_byte();
        bytes.push(minimum_fourth_byte);

        bytes
    }
}

impl FromBytes<SoaRdata> for SoaRdata {
    /// Creates a new SoaRdata from an array of bytes
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut soa_rdata = SoaRdata::new();

        let mname, bytes_without_mname = DomainName::from_bytes(bytes);
        let rname, bytes_without_rname = DomainName::from_bytes(bytes_without_mname);
        
        soa_rdata.set_mname(mname);
        soa_rdata.set_rname(rname);

        let serial = soa_rdata.set_serial_from_bytes(bytes_without_rname[0..4]);
        let refresh = soa_rdata.set_refresh_from_bytes(bytes_without_rname[4..8]);
        let retry = soa_rdata.set_retry_from_bytes(bytes_without_rname[8..12]);
        let expire = soa_rdata.set_expire_from_bytes(bytes_without_rname[12..16]);
        let minimum = soa_rdata.set_minimum_from_bytes(bytes_without_rname[16..20]);

        soa_rdata
    }
}
