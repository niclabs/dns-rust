use std::fmt;
#[derive(Clone, PartialEq, Debug, Hash, PartialOrd, Ord, Eq, Copy)]
/// Enum for the option code in an OPT Rdata
pub enum OptionCode {
    NSID,
    PADDING,
    UNKNOWN(u16),
}

impl From<OptionCode> for u16 {
    fn from(option_code: OptionCode) -> u16 {
        match option_code {
            OptionCode::NSID => 3,
            OptionCode::PADDING => 12,
            OptionCode::UNKNOWN(val) => val,
        }
    }
}

impl From<u16> for OptionCode {
    fn from(val: u16) -> OptionCode {
        match val {
            3 => OptionCode::NSID,
            12 => OptionCode::PADDING,
            _ => OptionCode::UNKNOWN(val),
        }
    }
}

impl From<&str> for OptionCode {
    fn from(val: &str) -> OptionCode {
        match val {
            "NSID" => OptionCode::NSID,
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
            OptionCode::PADDING => "PADDING",
            OptionCode::UNKNOWN(_) => "UNKNOWN",
        })
    }
}