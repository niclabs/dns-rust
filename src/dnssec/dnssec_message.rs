use crate::domain_name::DomainName;
use crate::message::rclass::Rclass;
use crate::message::DnsMessage;
use crate::message::rdata::opt_rdata::OptRdata;
use crate::message::rdata::Rdata;
use crate::message::resource_record::{ResourceRecord};
use crate::message::rcode::Rcode;
use crate::message::rrtype::Rrtype;

const EDNS_VERSION: u8 = 0;
const REQUESTED_UDP_LEN: u16 = 4096;
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
fn create_opt_rr(capacity: u16 ,e_rcode :Rcode, version: u8, do_bit: bool) -> ResourceRecord {
    let opt_rdata = OptRdata::new();
    let rdata = Rdata::OPT(opt_rdata);
    let mut rr = ResourceRecord::new(rdata);

    let e_rcode = u8::from(e_rcode); 
    let do_val: u16 = if do_bit {0x8000} else {0x0};
    let ttl: u32 = (e_rcode as u32) << 24 | (version as u32) << 16| (do_val as u32);
    rr.set_ttl(ttl);
    rr.set_rclass(Rclass::UNKNOWN(capacity));
    //println!("EL ttl es: {:#05x?}", ttl);
    rr
}

fn read_opt_rr(opt_rr: ResourceRecord) -> (u16, Rcode, u8, bool) {
    let requested_udp_len = u16::from(opt_rr.get_rclass());
    let data = opt_rr.get_ttl().to_be_bytes();
    let (e_rcode, version) = (data[0], data[1]);
    let z = u16::from_be_bytes([data[2], data[3]]);

    let do_bit = ((z & 0x8000) > 0) as bool ;
    (requested_udp_len, Rcode::from(e_rcode), version, do_bit)
    //format!("OPT PSEUDO-RR\n\trequested_udp_len: {requested_udp_len}\n\terror code: {e_rcode}\n\tversion: EDNS{version}\n\tuse dnssec: {do_bit}")
}

/*
   A security-aware resolver MUST include an EDNS ([RFC2671]) OPT
   pseudo-RR with the DO ([RFC3225]) bit set when sending queries.
*/
fn create_dns_message_with_opt_and_do(mut msg: DnsMessage) -> DnsMessage {
    // We create a opt rr with the do bit set to 1
    // with NOERR as rcode and EDNS0
    let rr = create_opt_rr(REQUESTED_UDP_LEN,
                            Rcode::from(Rcode::NOERROR).into(),
                            EDNS_VERSION,
                            true);

    let vec = vec![rr];
    msg.add_additionals(vec);
    msg
}


fn add_opt_record_dns_message(msg: &mut DnsMessage, capacity: u16, e_rcode :Rcode, do_bit: bool) {
    let rr = create_opt_rr(capacity,
        e_rcode,
        EDNS_VERSION,
        do_bit);
    
    msg.add_additionals(vec![rr]);
    msg.update_header_counters();
}


fn add_opt_record_dns_message(msg: &mut DnsMessage, capacity: u16, e_rcode :Rcode, do_bit: bool) {
    let rr = create_opt_rr(capacity,
        e_rcode,
        EDNS_VERSION,
        do_bit);
    
    msg.add_additionals(vec![rr]);
    msg.update_header_counters();
}

#[test]
fn see_dnssec_message() {
    let mut query = DnsMessage::new_query_message(
        DomainName::new_from_str("example.com"),
        Rrtype::A,
        Rclass::IN,
        1,
        true,
        2000
    );
    add_opt_record_dns_message(&mut query, 4096, Rcode::NOERROR, true);
    let expected = (4096,Rcode::NOERROR,0,true); 
    assert_eq!(expected,
               read_opt_rr(query.get_additional().pop().expect("No OPT Record!"))
    )
}