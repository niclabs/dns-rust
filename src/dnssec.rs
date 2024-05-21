pub mod dnssec_encryption;


use crate::domain_name::DomainName;
use crate::message::class_qclass::Qclass;
use crate::message::DnsMessage;
use crate::message::rdata::opt_rdata::OptRdata;
use crate::message::rdata::Rdata;
use crate::message::resource_record::{FromBytes, ResourceRecord, ToBytes};
use crate::message::type_qtype::Qtype;


/*
The mechanism chosen for the explicit notification of the ability of
the client to accept (if not understand) DNSSEC security RRs is using
the most significant bit of the Z field on the EDNS0 OPT header in
the query.  This bit is referred to as the "DNSSEC OK" (DO) bit.  In
the context of the EDNS0 OPT meta-RR, the DO bit is the first bit of
the third and fourth bytes of the "extended RCODE and flags" portion
of the EDNS0 OPT meta-RR, structured as follows:

            +0 (MSB)                +1 (LSB)
     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
  0: |   EXTENDED-RCODE      |       VERSION         |
     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
  2: |DO|                    Z                       |
     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
*/
// [0x12, 0x0, 0x80, 0x0]
fn create_opt_rr(e_rcode :u8, version: u8, do_bit: bool) -> ResourceRecord {
    let opt_rdata = OptRdata::new();
    let rdata = Rdata::OPT(opt_rdata);
    let mut rr = ResourceRecord::new(rdata);

    let do_val: u16 = if do_bit {0x8000} else {0x0};
    let ttl: u32 = (e_rcode as u32) << 24 | (version as u32) << 16| (do_val as u32);
    rr.set_ttl(ttl);
    println!("EL ttl es: {:#05x?}", ttl);
    rr
}
