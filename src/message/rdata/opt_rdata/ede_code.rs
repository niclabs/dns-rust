#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

impl From<u16> for EdeCode {
    fn from(val: u16) -> EdeCode {
        match val {
            0  => EdeCode::OtherErr,
            1  => EdeCode::UnsupDnskeyAlg,
            2  => EdeCode::UnsupDsDigest,
            3  => EdeCode::StaleAns,
            4  => EdeCode::ForgedAns,
            5  => EdeCode::DnssecIndet,
            6  => EdeCode::DnssecBogus,
            7  => EdeCode::SigExpired,
            8  => EdeCode::SigNotYetValid,
            9  => EdeCode::DnskeyMissing,
            10 => EdeCode::RrsigsMissing,
            11 => EdeCode::NoZoneKeyBit,
            12 => EdeCode::NsecMissing,
            13 => EdeCode::CachedErr,
            14 => EdeCode::NotReady,
            15 => EdeCode::Blocked,
            16 => EdeCode::Censored,
            17 => EdeCode::Filtered,
            18 => EdeCode::Prohibited,
            19 => EdeCode::StaleNxDomainAns,
            20 => EdeCode::NotAuth,
            21 => EdeCode::NotSupported,
            22 => EdeCode::NoReachableAuth,
            23 => EdeCode::NetErr,
            24 => EdeCode::InvalidData,
            val => EdeCode::Unknown(val),
        }
    }
}


impl EdeCode {

    pub fn to_u16(&self) -> u16 {
        u16::from(*self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < 2 {
            return Err("EdeCode must be 2 bytes long");
        }
        Ok(EdeCode::from(u16::from_be_bytes([bytes[0], bytes[1]])))
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        u16::from(*self).to_be_bytes().to_vec()
    }
}
