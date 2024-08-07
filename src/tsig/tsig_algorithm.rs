
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TsigAlgorithm {
    HmacSha1,
    HmacSha256,
    UNKNOWN(String),
}

impl From<TsigAlgorithm> for String {
    fn from(alg: TsigAlgorithm) -> String {
        match alg {
            TsigAlgorithm::HmacSha1 => "hmac-sha1".to_string(),
            TsigAlgorithm::HmacSha256 => "hmac-sha256".to_string(),
            TsigAlgorithm::UNKNOWN(s) => s,
        }
    }
}

impl From<String> for TsigAlgorithm {
    fn from(name: String) -> TsigAlgorithm {
        match name {
            name if name == "hmac-sha1" => TsigAlgorithm::HmacSha1,
            name if name == "hmac-sha256" => TsigAlgorithm::HmacSha256,
            _ => TsigAlgorithm::UNKNOWN(name),
        }
    }
}