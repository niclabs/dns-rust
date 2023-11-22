#[derive(Clone, PartialEq, Debug)]
/// Enum for the Class of a RR in a DnsMessage
pub enum Rclass {
    IN,
    CS,
    CH,
    HS,
    UNKNOWN(u16),
}

///Functions for the Rclass Enum
impl Rclass {
    ///Function to get the int equivalent of a class
    pub fn from_rclass_to_int(class: Rclass) -> u16{
        match class {
            Rclass::IN => 1,
            Rclass::CS => 2,
            Rclass::CH => 3,
            Rclass::HS => 4,
            Rclass::UNKNOWN(val) => val,
        }
    }

    ///Function to get an string representing the class
    pub fn from_rclass_to_str(class: Rclass) -> String{
        match class {
            Rclass::IN => String::from("IN"),
            Rclass::CS => String::from("CS"),
            Rclass::CH => String::from("CH"),
            Rclass::HS => String::from("HS"),
            Rclass::UNKNOWN(_val) => String::from("UNKNOWN CLASS")
        }
    }

    ///Function to get the Rclass from a value
    pub fn from_int_to_rclass(val:u16) -> Rclass{
        match val {
            1 => Rclass::IN,
            2 => Rclass::CS,
            3 => Rclass::CH,
            4 => Rclass::HS,
            _ => Rclass::UNKNOWN(val)
        }
    }

    ///Function to get the Rclass from a String
    pub fn from_str_to_rclass(rclass: &str) -> Rclass{
        match rclass {
            "IN" => Rclass::IN,
            "CS" => Rclass::CS,
            "CH" => Rclass::CH,
            "HS" => Rclass::HS,
            _ => Rclass::UNKNOWN(99)
        }
    }  
}

impl Default for Rclass {
    fn default() -> Self { Rclass::IN }
}