use crate::tsig::tsig_algorithm::TsigAlgorithm;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DnssecAlgorithm {
    RESERVED,
    RSAMD5,
    DH,
    DSA,
    ECC,
    RSASHA1,
    UNKNOWN(u8)
}

impl From<DnssecAlgorithm> for u8 {
    fn from(alg: DnssecAlgorithm) -> u8 {
        match alg {
            DnssecAlgorithm::RESERVED => 0,
            DnssecAlgorithm::RSAMD5 => 1,
            DnssecAlgorithm::DH => 2,
            DnssecAlgorithm::DSA => 3,
            DnssecAlgorithm::ECC => 4,
            DnssecAlgorithm::RSASHA1 => 5,
            DnssecAlgorithm::UNKNOWN(other) => other
        }
    }
}

impl From<u8> for DnssecAlgorithm {
    fn from(code: u8) -> DnssecAlgorithm {
        match code {
            0 => DnssecAlgorithm::RESERVED,
            1 => DnssecAlgorithm::RSAMD5,
            2 => DnssecAlgorithm::DH,
            3 => DnssecAlgorithm::DSA,
            4 => DnssecAlgorithm::ECC,
            5 => DnssecAlgorithm::RSASHA1,
            other => DnssecAlgorithm::UNKNOWN(other)
        }
    }
}