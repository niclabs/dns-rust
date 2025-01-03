
mod ede_code {
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
                EdeCode::OtherErr => 0,
                EdeCode::UnsupDnskeyAlg => 1,
                EdeCode::UnsupDsDigest => 2,
                EdeCode::StaleAns => 3,
                EdeCode::ForgedAns => 4,
                EdeCode::DnssecIndet => 5,
                EdeCode::DnssecBogus => 6,
                EdeCode::SigExpired => 7,
                EdeCode::SigNotYetValid => 8,
                EdeCode::DnskeyMissing => 9,
                EdeCode::RrsigsMissing => 10,
                EdeCode::NoZoneKeyBit => 11,
                EdeCode::NsecMissing => 12,
                EdeCode::CachedErr => 13,
                EdeCode::NotReady => 14,
                EdeCode::Blocked => 15,
                EdeCode::Censored => 16,
                EdeCode::Filtered => 17,
                EdeCode::Prohibited => 18,
                EdeCode::StaleNxDomainAns => 19,
                EdeCode::NotAuth => 20,
                EdeCode::NotSupported => 21,
                EdeCode::NoReachableAuth => 22,
                EdeCode::NetErr => 23,
                EdeCode::InvalidData => 24,
                EdeCode::Unknown(val) => val,
            }
        }
    }

    impl From<u16> for EdeCode {
        fn from(val: u16) -> EdeCode {
            match val {
                0 => EdeCode::OtherErr,
                1 => EdeCode::UnsupDnskeyAlg,
                2 => EdeCode::UnsupDsDigest,
                3 => EdeCode::StaleAns,
                4 => EdeCode::ForgedAns,
                5 => EdeCode::DnssecIndet,
                6 => EdeCode::DnssecBogus,
                7 => EdeCode::SigExpired,
                8 => EdeCode::SigNotYetValid,
                9 => EdeCode::DnskeyMissing,
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
}

pub mod ede_optdata {
    use crate::edns::options::ede::ede_code::EdeCode;
    use crate::message::resource_record::{FromBytes, ToBytes};

    /*
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
        err_code: EdeCode,
        err_message: String,
    }

    impl EdeOptData {
        pub fn new(err_code: EdeCode, err_message: String) -> Self {
            EdeOptData { err_code, err_message }
        }
        pub fn get_err_code(&self) -> EdeCode {
            self.err_code.clone()
        }
        pub fn get_err_message(&self) -> String {
            self.err_message.clone()
        }
        pub fn set_err_code(&mut self, err_code: EdeCode) {
            self.err_code = err_code;
        }
        pub fn set_err_message(&mut self, err_message: String) {
            self.err_message = err_message;
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

            let mut err_code_bytes = self.err_code.to_bytes();
            res.append(&mut err_code_bytes);

            let mut msg_bytes = self.err_message.as_bytes().to_vec();
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
            assert_eq!(deserialized.get_err_code(), code);
            assert_eq!(deserialized.get_err_message(), msg);
        }

        #[test]
        fn test_to_from_bytes_unsupdnskeyalg() {
            let code = EdeCode::UnsupDnskeyAlg;
            let msg = "Clave DNS no soportada".to_string();

            let ede = EdeOptData::new(code, msg.clone());
            let serialized = ede.to_bytes();

            let deserialized = EdeOptData::from_bytes(&serialized).unwrap();
            assert_eq!(deserialized.get_err_code(), code);
            assert_eq!(deserialized.get_err_message(), msg);
        }

        #[test]
        fn test_to_from_bytes_staleans() {
            let code = EdeCode::StaleAns;
            let msg = "Respuesta obsoleta".to_string();

            let ede = EdeOptData::new(code, msg.clone());
            let serialized = ede.to_bytes();

            let deserialized = EdeOptData::from_bytes(&serialized).unwrap();
            assert_eq!(deserialized.get_err_code(), code);
            assert_eq!(deserialized.get_err_message(), msg);
        }

        #[test]
        fn test_to_from_bytes_forgedans() {
            let code = EdeCode::ForgedAns;
            let msg = "Respuesta falsificada".to_string();

            let ede = EdeOptData::new(code, msg.clone());
            let serialized = ede.to_bytes();

            let deserialized = EdeOptData::from_bytes(&serialized).unwrap();
            assert_eq!(deserialized.get_err_code(), code);
            assert_eq!(deserialized.get_err_message(), msg);
        }

        #[test]
        fn test_to_from_bytes_unknown() {
            // Probamos con un valor fuera de los enumerados estándar.
            let code = EdeCode::Unknown(999);
            let msg = "Error genérico".to_string();

            let mut ede = EdeOptData::new(code, msg.clone());
            ede.set_err_code(EdeCode::Unknown(1000));
            ede.set_err_message("Mensaje modificado".to_string());

            assert_eq!(ede.get_err_code(), EdeCode::Unknown(1000));
            assert_eq!(ede.get_err_message(), "Mensaje modificado");

            let serialized = ede.to_bytes();

            let deserialized = EdeOptData::from_bytes(&serialized).unwrap();

            assert_eq!(deserialized.get_err_code(), EdeCode::Unknown(1000));
            assert_eq!(deserialized.get_err_message(), "Mensaje modificado");
        }
    }
}