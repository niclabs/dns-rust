use std::fmt;

#[derive(Clone, PartialEq, Debug, Hash, PartialOrd, Ord, Eq, Copy)]
/// Enum For the Type of a RR in a DnsMessage with an Rdata implementation
pub enum Rrtype {
    A,           // 1 - [RFC1035]
    NS,          // 2 - [RFC1035]
    CNAME,       // 5 - [RFC1035]
    SOA,         // 6 - [RFC1035]
    WKS,         // 11 - [RFC1035]
    PTR,         // 12 - [RFC1035]
    HINFO,       // 13 - [RFC1035]
    MINFO,       // 14 - [RFC1035]
    MX,          // 15 - [RFC1035]
    TXT,         // 16 - [RFC1035]
    RP,          // 17 - [RFC1183]
    AFSDB,       // 18 - [RFC1183][RFC5864]
    X25,         // 19 - [RFC1183]
    ISDN,        // 20 - [RFC1183]
    RT,          // 21 - [RFC1183]
    SIG,         // 24 - [RFC2536][RFC2931][RFC3110][RFC4034]
    KEY,         // 25 - [RFC2536][RFC2539][RFC3110][RFC4034]
    PX,          // 26 - [RFC2163]
    GPOS,        // 27 - [RFC1712]
    AAAA,        // 28 - [RFC3596]
    LOC,         // 29 - [RFC1876]
    SRV,         // 33 - [RFC2782]
    NAPTR,       // 35 - [RFC3403]
    KX,          // 36 - [RFC2230]
    CERT,        // 37 - [RFC4398]
    DNAME,       // 39 - [RFC6672]
    OPT,         // 41 - [RFC3225][RFC6891]
    APL,         // 42 - [RFC3123]
    DS,          // 43 - [RFC4034]
    SSHFP,       // 44 - [RFC4255]
    IPSECKEY,    // 45 - [RFC4025]
    RRSIG,       // 46 - [RFC4034]
    NSEC,        // 47 - [RFC4034][RFC9077]
    DNSKEY,      // 48 - [RFC4034]
    DHCID,       // 49 - [RFC4701]
    NSEC3,       // 50 - [RFC5155][RFC9077]
    NSEC3PARAM,  // 51 - [RFC5155]
    TLSA,        // 52 - [RFC6698]
    SMIMEA,      // 53 - [RFC8162]
    HIP,         // 55 - [RFC8005]
    CDS,         // 59 - [RFC7344]
    CDNSKEY,     // 60 - [RFC7344]
    OPENPGPKEY,  // 61 - [RFC7929]
    CSYNC,       // 62 - [RFC7477]
    ZONEMD,      // 63 - [RFC8976]
    SVCB,        // 64 - [RFC9460]
    HTTPS,       // 65 - [RFC9460]
    SPF,         // 99 - [RFC7208]
    NID,         // 104 - [RFC6742]
    L32,         // 105 - [RFC6742]
    L64,         // 106 - [RFC6742]
    LP,          // 107 - [RFC6742]
    EUI48,       // 108 - [RFC7043]
    EUI64,       // 109 - [RFC7043]
    TKEY,        // 249 - [RFC2930]
    TSIG,        // 250 - [RFC8945]
    IXFR,        // 251 - [RFC1995]
    AXFR,        // 252 - [RFC1035][RFC5936]
    MAILB,       // 253 - [RFC1035]
    MAILA,       // 254 - [RFC1035]
    ANY,         // 255 - [RFC1035][RFC6895][RFC8482]
    URI,         // 256 - [RFC7553]
    CAA,         // 257 - [RFC8659]
    AVC,         // 258 - [Wolfgang_Riedel]
    DOA,         // 259 - [draft-durand-doa-over-dns]
    AMTRELAY,    // 260 - [RFC8777]
    RESINFO,     // 261 - [RFC-ietf-add-resolver-info-13]
    UNKNOWN(u16),
}

impl From<Rrtype> for u16 {
    fn from(rrtype: Rrtype) -> u16 {
        match rrtype {
            Rrtype::A => 1,
            Rrtype::NS => 2,
            Rrtype::CNAME => 5,
            Rrtype::SOA => 6,
            Rrtype::WKS => 11,
            Rrtype::PTR => 12,
            Rrtype::HINFO => 13,
            Rrtype::MINFO => 14,
            Rrtype::MX => 15,
            Rrtype::TXT => 16,
            Rrtype::RP => 17,
            Rrtype::AFSDB => 18,
            Rrtype::X25 => 19,
            Rrtype::ISDN => 20,
            Rrtype::RT => 21,
            Rrtype::SIG => 24,
            Rrtype::KEY => 25,
            Rrtype::PX => 26,
            Rrtype::GPOS => 27,
            Rrtype::AAAA => 28,
            Rrtype::LOC => 29,
            Rrtype::SRV => 33,
            Rrtype::NAPTR => 35,
            Rrtype::KX => 36,
            Rrtype::CERT => 37,
            Rrtype::DNAME => 39,
            Rrtype::OPT => 41,
            Rrtype::APL => 42,
            Rrtype::DS => 43,
            Rrtype::SSHFP => 44,
            Rrtype::IPSECKEY => 45,
            Rrtype::RRSIG => 46,
            Rrtype::NSEC => 47,
            Rrtype::DNSKEY => 48,
            Rrtype::DHCID => 49,
            Rrtype::NSEC3 => 50,
            Rrtype::NSEC3PARAM => 51,
            Rrtype::TLSA => 52,
            Rrtype::SMIMEA => 53,
            Rrtype::HIP => 55,
            Rrtype::CDS => 59,
            Rrtype::CDNSKEY => 60,
            Rrtype::OPENPGPKEY => 61,
            Rrtype::CSYNC => 62,
            Rrtype::ZONEMD => 63,
            Rrtype::SVCB => 64,
            Rrtype::HTTPS => 65,
            Rrtype::SPF => 99,
            Rrtype::NID => 104,
            Rrtype::L32 => 105,
            Rrtype::L64 => 106,
            Rrtype::LP => 107,
            Rrtype::EUI48 => 108,
            Rrtype::EUI64 => 109,
            Rrtype::TKEY => 249,
            Rrtype::TSIG => 250,
            Rrtype::IXFR => 251,
            Rrtype::AXFR => 252,
            Rrtype::MAILB => 253,
            Rrtype::MAILA => 254,
            Rrtype::ANY => 255,
            Rrtype::URI => 256,
            Rrtype::CAA => 257,
            Rrtype::AVC => 258,
            Rrtype::DOA => 259,
            Rrtype::AMTRELAY => 260,
            Rrtype::RESINFO => 261,
            Rrtype::UNKNOWN(val) => val
        }
    }
}

impl From<u16> for Rrtype {
    fn from(val: u16) -> Rrtype {
        match val {
            1 => Rrtype::A,
            2 => Rrtype::NS,
            5 => Rrtype::CNAME,
            6 => Rrtype::SOA,
            11 => Rrtype::WKS,
            12 => Rrtype::PTR,
            13 => Rrtype::HINFO,
            14 => Rrtype::MINFO,
            15 => Rrtype::MX,
            16 => Rrtype::TXT,
            28 => Rrtype::AAAA,
            39 => Rrtype::DNAME,
            41 => Rrtype::OPT,
            43 => Rrtype::DS,
            46 => Rrtype::RRSIG,
            47 => Rrtype::NSEC,
            48 => Rrtype::DNSKEY,
            50 => Rrtype::NSEC3,
            51 => Rrtype::NSEC3PARAM,
            250 => Rrtype::TSIG,
            252 => Rrtype::AXFR,
            253 => Rrtype::MAILB,
            254 => Rrtype::MAILA,
            255 => Rrtype::ANY,
            _ => Rrtype::UNKNOWN(val),
        }
    }
}

impl From<&str> for Rrtype {
    fn from(rrtype: &str) -> Rrtype {
        match rrtype {
            "A" => Rrtype::A,
            "NS" => Rrtype::NS,
            "CNAME" => Rrtype::CNAME,
            "SOA" => Rrtype::SOA,
            "WKS" => Rrtype::WKS,
            "PTR" => Rrtype::PTR,
            "HINFO" => Rrtype::HINFO,
            "MINFO" => Rrtype::MINFO,
            "MX" => Rrtype::MX,
            "TXT" => Rrtype::TXT,
            "RP" => Rrtype::RP,
            "AFSDB" => Rrtype::AFSDB,
            "X25" => Rrtype::X25,
            "ISDN" => Rrtype::ISDN,
            "RT" => Rrtype::RT,
            "SIG" => Rrtype::SIG,
            "KEY" => Rrtype::KEY,
            "PX" => Rrtype::PX,
            "GPOS" => Rrtype::GPOS,
            "AAAA" => Rrtype::AAAA,
            "LOC" => Rrtype::LOC,
            "SRV" => Rrtype::SRV,
            "NAPTR" => Rrtype::NAPTR,
            "KX" => Rrtype::KX,
            "CERT" => Rrtype::CERT,
            "DNAME" => Rrtype::DNAME,
            "OPT" => Rrtype::OPT,
            "APL" => Rrtype::APL,
            "DS" => Rrtype::DS,
            "SSHFP" => Rrtype::SSHFP,
            "IPSECKEY" => Rrtype::IPSECKEY,
            "RRSIG" => Rrtype::RRSIG,
            "NSEC" => Rrtype::NSEC,
            "DNSKEY" => Rrtype::DNSKEY,
            "DHCID" => Rrtype::DHCID,
            "NSEC3" => Rrtype::NSEC3,
            "NSEC3PARAM" => Rrtype::NSEC3PARAM,
            "TLSA" => Rrtype::TLSA,
            "SMIMEA" => Rrtype::SMIMEA,
            "HIP" => Rrtype::HIP,
            "CDS" => Rrtype::CDS,
            "CDNSKEY" => Rrtype::CDNSKEY,
            "OPENPGPKEY" => Rrtype::OPENPGPKEY,
            "CSYNC" => Rrtype::CSYNC,
            "ZONEMD" => Rrtype::ZONEMD,
            "SVCB" => Rrtype::SVCB,
            "HTTPS" => Rrtype::HTTPS,
            "SPF" => Rrtype::SPF,
            "NID" => Rrtype::NID,
            "L32" => Rrtype::L32,
            "L64" => Rrtype::L64,
            "LP" => Rrtype::LP,
            "EUI48" => Rrtype::EUI48,
            "EUI64" => Rrtype::EUI64,
            "TKEY" => Rrtype::TKEY,
            "TSIG" => Rrtype::TSIG,
            "IXFR" => Rrtype::IXFR,
            "AXFR" => Rrtype::AXFR,
            "MAILB" => Rrtype::MAILB,
            "MAILA" => Rrtype::MAILA,
            "ANY" => Rrtype::ANY,
            "URI" => Rrtype::URI,
            "CAA" => Rrtype::CAA,
            "AVC" => Rrtype::AVC,
            "DOA" => Rrtype::DOA,
            "AMTRELAY" => Rrtype::AMTRELAY,
            "RESINFO" => Rrtype::RESINFO,
            _ => Rrtype::UNKNOWN(99),
        }
    }
}

impl Default for Rrtype {
    fn default() -> Self { Rrtype::A }
}

impl fmt::Display for Rrtype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Rrtype::A => "A",
            Rrtype::NS => "NS",
            Rrtype::CNAME => "CNAME",
            Rrtype::SOA => "SOA",
            Rrtype::PTR => "PTR",
            Rrtype::HINFO => "HINFO",
            Rrtype::MINFO => "MINFO",
            Rrtype::WKS => "WKS",
            Rrtype::MX => "MX",
            Rrtype::TXT => "TXT",
            Rrtype::AAAA => "AAAA",
            Rrtype::DNAME => "DNAME",
            Rrtype::OPT => "OPT",
            Rrtype::DS => "DS",
            Rrtype::RRSIG => "RRSIG",
            Rrtype::NSEC => "NSEC",
            Rrtype::DNSKEY => "DNSKEY",
            Rrtype::NSEC3 => "NSEC3",
            Rrtype::NSEC3PARAM => "NSEC3PARAM",
            Rrtype::TSIG => "TSIG",
            Rrtype::AXFR => "AXFR",
            Rrtype::MAILB => "MAILB",
            Rrtype::MAILA => "MAILA",
            Rrtype::ANY => "ANY",
            Rrtype::TLSA => "TLSA",
            Rrtype::ISDN => "ISDN",
            Rrtype::CAA => "CAA",
            Rrtype::RT => "RT",
            Rrtype::SIG => "SIG",
            Rrtype::KEY => "KEY",
            Rrtype::PX => "PX",
            Rrtype::GPOS => "GPOS",
            Rrtype::LOC => "LOC",
            Rrtype::SRV => "SRV",
            Rrtype::NAPTR => "NAPTR",
            Rrtype::KX => "KX",
            Rrtype::CERT => "CERT",
            Rrtype::SSHFP => "SSHFP",
            Rrtype::IPSECKEY => "IP",
            Rrtype::APL => "APL",
            Rrtype::DHCID => "DHCID",
            Rrtype::SMIMEA => "SMIMEA",
            Rrtype::HIP => "HIP",
            Rrtype::CDS => "CDS",
            Rrtype::CDNSKEY => "CDNSKEY",
            Rrtype::OPENPGPKEY => "OPENPGPKEY",
            Rrtype::CSYNC => "CSYNC",
            Rrtype::ZONEMD => "ZONEMD",
            Rrtype::SVCB => "SVCB",
            Rrtype::HTTPS => "HTTPS",
            Rrtype::SPF => "SPF",
            Rrtype::NID => "NID",
            Rrtype::L32 => "L32",
            Rrtype::L64 => "L64",
            Rrtype::LP => "LP",
            Rrtype::EUI48 => "EUI48",
            Rrtype::EUI64 => "EUI64",
            Rrtype::TKEY => "TKEY",
            Rrtype::IXFR => "IXFR",
            Rrtype::AVC => "AVC",
            Rrtype::DOA => "DOA",
            Rrtype::AMTRELAY => "AMTRELAY",
            Rrtype::RESINFO => "RESINFO",
            Rrtype::RP => "RP",
            Rrtype::X25 => "X25",
            Rrtype::URI => "URI",
            Rrtype::AFSDB => "AFSDB",
            Rrtype::UNKNOWN(_) => "UNKNOWN"
            // Add any other missing variants here
        })
    }
}