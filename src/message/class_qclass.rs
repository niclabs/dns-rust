#[derive(Clone, PartialEq, Debug)]
/// Enum for the Class of a RR in a DnsMessage
pub enum Qclass {
    IN,
    CS,
    CH,
    HS,
    ANY,
    UNKNOWN(u16),
}

///Functions for the Rclass Enum
impl Qclass {
    ///Function to get the int equivalent of a class
    pub fn from_qclass_to_int(class: Qclass) -> u16{
        match class {
            Qclass::IN => 1,
            Qclass::CS => 2,
            Qclass::CH => 3,
            Qclass::HS => 4,
            Qclass::ANY => 255,
            Qclass::UNKNOWN(val) => val,
        }
    }

    ///Function to get an string representing the class
    pub fn from_qclass_to_str(class: Qclass) -> String{
        match class {
            Qclass::IN => String::from("IN"),
            Qclass::CS => String::from("CS"),
            Qclass::CH => String::from("CH"),
            Qclass::HS => String::from("HS"),
            Qclass::ANY => String::from("ANY"),
            Qclass::UNKNOWN(_val) => String::from("UNKNOWN CLASS")
        }
    }

    ///Function to get the Qclass from a value
    pub fn from_int_to_qclass(val:u16) -> Qclass{
        match val {
            1 => Qclass::IN,
            2 => Qclass::CS,
            3 => Qclass::CH,
            4 => Qclass::HS,
            255 => Qclass::ANY,
            _ => Qclass::UNKNOWN(val)
        }
    }

    ///Function to get the Qclass from a String
    pub fn from_str_to_qclass(qclass: &str) -> Qclass{
        match qclass {
            "IN" => Qclass::IN,
            "CS" => Qclass::CS,
            "CH" => Qclass::CH,
            "HS" => Qclass::HS,
            "ANY" => Qclass::ANY,
            _ => Qclass::UNKNOWN(99)
        }
    }  
}

impl Default for Qclass {
    fn default() -> Self { Qclass::IN }
}