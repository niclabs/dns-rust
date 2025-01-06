use crate::message::rdata::opt_rdata::ede_code::EdeCode;
use crate::message::resource_record::{FromBytes, ToBytes};

/*
Extended DNS Error (EDE) information in DNS messages. The option is structured as follows:
                                             1   1   1   1   1   1
     0   1   2   3   4   5   6   7   8   9   0   1   2   3   4   5
   +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
0: |                            OPTION-CODE                        |
   +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
2: |                           OPTION-LENGTH                       |
   +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
4: | INFO-CODE                                                     |
   +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
6: / EXTRA-TEXT ...                                                /
   +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
*/
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EdeOptData {
    info_code: EdeCode,
    extra_text: String,
}

impl EdeOptData {
    pub fn new(err_code: EdeCode, err_message: String) -> Self {
        EdeOptData{ info_code: err_code, extra_text: err_message }
    }
    pub fn get_info_code(&self) -> EdeCode {
        self.info_code.clone()
    }
    pub fn get_extra_text(&self) -> String {
        self.extra_text.clone()
    }
    pub fn set_info_code(&mut self, err_code: EdeCode) {
        self.info_code = err_code;
    }
    pub fn set_extra_text(&mut self, err_message: String) {
        self.extra_text = err_message;
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < 2 {
            return Err("Not enough bytes to parse EdeCode");
        }

        let err_code = EdeCode::from_bytes(&bytes[0..2])
            .map_err(|_| "Error parsing EdeCode")?;
        let err_message = String::from_utf8(bytes[2..].to_vec())
            .map_err(|_| "Error parsing UTF-8 for err_message")?;

        Ok(EdeOptData::new(err_code, err_message))
    }
}

impl ToBytes for EdeOptData {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = vec![];

        let mut err_code_bytes = self.info_code.to_bytes();
        res.append(&mut err_code_bytes);

        let mut msg_bytes = self.extra_text.as_bytes().to_vec();
        res.append(&mut msg_bytes);

        res
    }
}


#[cfg(test)]
mod edetests {
    use super::*;

    #[test]
    fn test_to_from_bytes_othererr() {
        let code = EdeCode::OtherErr;
        let msg = "Mensaje de prueba para OtherErr".to_string();

        let ede = EdeOptData::new(code, msg.clone());
        let serialized = ede.to_bytes();

        let deserialized = EdeOptData::from_bytes(&serialized).unwrap();
        assert_eq!(deserialized.get_info_code(), code);
        assert_eq!(deserialized.get_extra_text(), msg);
    }

    #[test]
    fn test_to_from_bytes_unsupdnskeyalg() {
        let code = EdeCode::UnsupDnskeyAlg;
        let msg = "Clave DNS no soportada".to_string();

        let ede = EdeOptData::new(code, msg.clone());
        let serialized = ede.to_bytes();

        let deserialized = EdeOptData::from_bytes(&serialized).unwrap();
        assert_eq!(deserialized.get_info_code(), code);
        assert_eq!(deserialized.get_extra_text(), msg);
    }

    #[test]
    fn test_to_from_bytes_staleans() {
        let code = EdeCode::StaleAns;
        let msg = "Respuesta obsoleta".to_string();

        let ede = EdeOptData::new(code, msg.clone());
        let serialized = ede.to_bytes();

        let deserialized = EdeOptData::from_bytes(&serialized).unwrap();
        assert_eq!(deserialized.get_info_code(), code);
        assert_eq!(deserialized.get_extra_text(), msg);
    }

    #[test]
    fn test_to_from_bytes_forgedans() {
        let code = EdeCode::ForgedAns;
        let msg = "Respuesta falsificada".to_string();

        let ede = EdeOptData::new(code, msg.clone());
        let serialized = ede.to_bytes();

        let deserialized = EdeOptData::from_bytes(&serialized).unwrap();
        assert_eq!(deserialized.get_info_code(), code);
        assert_eq!(deserialized.get_extra_text(), msg);
    }

    #[test]
    fn test_to_from_bytes_unknown() {
        // Probamos con un valor fuera de los enumerados estándar.
        let code = EdeCode::Unknown(999);
        let msg = "Error genérico".to_string();

        let mut ede = EdeOptData::new(code, msg.clone());
        ede.set_info_code(EdeCode::Unknown(1000));
        ede.set_extra_text("Mensaje modificado".to_string());

        assert_eq!(ede.get_info_code(), EdeCode::Unknown(1000));
        assert_eq!(ede.get_extra_text(), "Mensaje modificado");

        let serialized = ede.to_bytes();

        let deserialized = EdeOptData::from_bytes(&serialized).unwrap();

        assert_eq!(deserialized.get_info_code(), EdeCode::Unknown(1000));
        assert_eq!(deserialized.get_extra_text(), "Mensaje modificado");
    }
}