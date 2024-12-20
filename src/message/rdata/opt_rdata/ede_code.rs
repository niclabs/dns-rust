use crate::message::resource_record::ToBytes;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EdeCode {
    OtherErr,           // 0
    UnsupDnskeyAlg,     // 1
    UnsupDsDigest,      // 2
    StaleAns,           // 3
    ForgedAns,          // 4
    DnssecIndet,        // 5
    DnssecBogus,        // 6
    SigExpired,         // 7
    SigNotYetValid,     // 8
    DnskeyMissing,      // 9
    RrsigsMissing,      // 10
    NoZoneKeyBit,       // 11
    NsecMissing,        // 12
    CachedErr,          // 13
    NotReady,           // 14
    Blocked,            // 15
    Censored,           // 16
    Filtered,           // 17
    Prohibited,         // 18
    StaleNxDomainAns,   // 19
    NotAuth,            // 20
    NotSupported,       // 21
    NoReachableAuth,    // 22
    NetErr,             // 23
    InvalidData,        // 24
    Unknown(u16),
}

impl From<EdeCode> for u16 {
    fn from(code: EdeCode) -> u16 {
        match code {
            EdeCode::OtherErr           => 0,
            EdeCode::UnsupDnskeyAlg     => 1,
            EdeCode::UnsupDsDigest      => 2,
            EdeCode::StaleAns           => 3,
            EdeCode::ForgedAns          => 4,
            EdeCode::DnssecIndet        => 5,
            EdeCode::DnssecBogus        => 6,
            EdeCode::SigExpired         => 7,
            EdeCode::SigNotYetValid     => 8,
            EdeCode::DnskeyMissing      => 9,
            EdeCode::RrsigsMissing      => 10,
            EdeCode::NoZoneKeyBit       => 11,
            EdeCode::NsecMissing        => 12,
            EdeCode::CachedErr          => 13,
            EdeCode::NotReady           => 14,
            EdeCode::Blocked            => 15,
            EdeCode::Censored           => 16,
            EdeCode::Filtered           => 17,
            EdeCode::Prohibited         => 18,
            EdeCode::StaleNxDomainAns   => 19,
            EdeCode::NotAuth            => 20,
            EdeCode::NotSupported       => 21,
            EdeCode::NoReachableAuth    => 22,
            EdeCode::NetErr             => 23,
            EdeCode::InvalidData        => 24,
            EdeCode::Unknown(val)  => val,
        }
    }
}

impl ToBytes for EdeCode {
    fn to_bytes(&self) -> Vec<u8> {
        u16::from(self).to_be_bytes().to_vec()
    }
}
