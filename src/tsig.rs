//aquí debe ir todo lo relacionado a la implementación de tsig como módulo
use crate::domain_name::DomainName;
use crate::message::class_qclass::Qclass;
use crate::message::type_qtype::Qtype;
use crate::message::{rdata::tsig_rdata::TSigRdata, DnsMessage};
use crate::message::rdata::Rdata;
use sha2::Sha256;
use sha1::Sha1;

enum TsigAlgorithm {
    HmacSha1,
    HmacSha256,
}

/// This functions signs a DnsMessage with a key in bytes, and the 
/// algName will be used to select the algorithm to encrypt the key.
fn sign_msg(query_msg:DnsMessage,key:Bytes, alg_name:TsigAlgorithm)->Bytes{
    let mut new_query_msg = query_msg.clone();
    let mut dig_string = "";
    let mut mac_len = "";
    let placeholder_hex: String; 
    match alg_name {
        TsigAlgorithm::HmacSha1 => {
            let mut hasher = crypto_hmac::new(Sha1::new(), key);
            hasher.input(&new_query_message.to_bytes()[..]);
            let result = hasher.result();
            let placeholder = result.code();
            //Convertir los bytes brutos a una cadena hexadecimal
            placeholder_hex = placeholder.iter().map(|b| format!("{:02x}", b)).collect();
            dig_string = &placeholder_hex;
            mac_len = "20";
        },
        TsigAlgorithm::HmacSha256 => {
            let mut hasher = HmacSha256::new_from_slice(key) .expect("HMAC algoritms can take keys of any size");
            hasher.update(&dns_query_message.to_bytes()[..]);
            let result = hasher.finalize();

            let code_bytes = result.into_bytes();
            placeholder_hex = hex::encode(code_bytes);
            dig_string = &placeholder_hex;
            mac_len = "32";
            
        },
        _ => {panic!("Error")},
    }
    //TODO: agregar los demas valores al msg_bytes !! Yo creo que deben llegar como argumentos
    msg_bytes = format!("{}.\n51921\n1234\n{}\n{}\n1234\n0\n0",alg_name, mac_len, dig_string);
    return msg_bytes;
}
                                                            
fn tsig_proccesing_answer(answer_msg:DnsMessage)->Bool{
    //procesar los errores 
    new_answer_msg = answer_msg.clone();
}