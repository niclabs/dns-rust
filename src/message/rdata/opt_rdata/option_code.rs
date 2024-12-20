use std::fmt;
#[derive(Clone, PartialEq, Debug, Hash, PartialOrd, Ord, Eq, Copy)]
/// Enum for the option code in an OPT Rdata
pub enum OptionCode {
    NSID,
    PADDING,
    UNKNOWN(u16),
    // added for rfc6975
    DAU,
    DHU,
    // added for rf8914
    EDE,
    N3U
}

impl From<OptionCode> for u16 {
    fn from(option_code: OptionCode) -> u16 {
        match option_code {
            OptionCode::NSID => 3,
            OptionCode::DAU => 5,
            OptionCode::DHU => 6,
            OptionCode::N3U => 7,
            OptionCode::PADDING => 12,
            OptionCode::EDE => 14,
            OptionCode::UNKNOWN(val) => val,
        }
    }
}

impl From<u16> for OptionCode {
    fn from(val: u16) -> OptionCode {
        match val {
            3 => OptionCode::NSID,
            5 => OptionCode::DAU,
            6 => OptionCode::DHU,
            7 => OptionCode::N3U,
            12 => OptionCode::PADDING,
            15 => OptionCode::EDE,
            _ => OptionCode::UNKNOWN(val),
        }
    }
}

impl From<&str> for OptionCode {
    fn from(val: &str) -> OptionCode {
        match val {
            "NSID" => OptionCode::NSID,
            "DAU" => OptionCode::DAU,
            "DHU" => OptionCode::DHU,
            "N3U" => OptionCode::N3U,
            "EDE" => OptionCode::EDE,
            "PADDING" => OptionCode::PADDING,
            _ => OptionCode::UNKNOWN(0),
        }
    }
}

impl Default for OptionCode {
    fn default() -> Self {
        OptionCode::UNKNOWN(0)
    }
}

impl fmt::Display for OptionCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            OptionCode::NSID => "NSID",
            OptionCode::DAU => "DAU",
            OptionCode::DHU => "DHU",
            OptionCode::N3U => "N3U",
            OptionCode::EDE => "EDE",
            OptionCode::PADDING => "PADDING",
            OptionCode::UNKNOWN(_) => "UNKNOWN",
        })
    }
}