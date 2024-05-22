pub mod dnssec_encryption;


use std::str::FromStr;
use crate::domain_name::DomainName;
use crate::message::class_qclass::Qclass;
use crate::message::DnsMessage;
use crate::message::rdata::opt_rdata::OptRdata;
use crate::message::rdata::Rdata;
use crate::message::resource_record::{FromBytes, ResourceRecord, ToBytes};
use crate::message::type_qtype::Qtype;
use crate::message::rcode;
use crate::message::rcode::Rcode;

const EDNS_VERSION: u8 = 0;
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

fn read_opt_rr(opt_rr: ResourceRecord) -> String {
    let data = opt_rr.get_ttl().to_be_bytes();
    let (e_rcode, version) = (data[0], data[1]);
    let z = u16::from_be_bytes([data[2], data[3]]);

    let do_bit = ((z & 0x8000) > 0) as u8 ;
    format!("OPT PSEUDO-RR\n\terror code: {e_rcode}\n\tversion: EDNS{version}\n\tuse dnssec: {do_bit}")
}

/*
   A security-aware resolver MUST include an EDNS ([RFC2671]) OPT
   pseudo-RR with the DO ([RFC3225]) bit set when sending queries.
*/
fn create_dns_message_with_dnssec(mut msg: DnsMessage) -> DnsMessage {
    // We create a opt rr with the do bit set to 1
    // with NOERR as rcode and EDNS0
    let rr = create_opt_rr(
                            rcode::Rcode::from_rcode_to_int(Rcode::NOERROR),
                            EDNS_VERSION,
                            true);

    let vec = vec![rr];
    msg.add_additionals(vec);
    msg
}

#[test]
fn see_dnssec_message() {
    let query = DnsMessage::new_query_message(
        DomainName::new_from_str("example.com"),
        Qtype::A,
        Qclass::ANY,
        1,
        true,
        2000
    );
    let query= create_dns_message_with_dnssec(query);
    assert_eq!(String::from_str
                   ("OPT PSEUDO-RR\n\terror code: 0\n\tversion: EDNS0\n\tuse dnssec: 1")
                   .expect("Not a utf8 str"),
               read_opt_rr(query.get_additional().pop().expect("No OPT Record!"))
    )
}