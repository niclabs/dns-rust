//aquí debe ir todo lo relacionado a la implementación de tsig como módulo
use crate::domain_name::DomainName;
use crate::message::class_qclass::Qclass;
use crate::message::type_qtype::Qtype;
use crate::message::{rdata::tsig_rdata::TSigRdata, DnsMessage};
use std::os::unix::process;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::message::rdata::Rdata;
use sha2::Sha256;
use crypto::hmac::Hmac as crypto_hmac;
use crypto::mac::Mac as crypto_mac;
use hmac::{Hmac, Mac};
use crypto::sha1::Sha1;
use bytes::Bytes;

type HmacSha256 = Hmac<Sha256>;

//TODO: usar arreglar el funcionamiento del enum en sign_msg
enum TsigAlgorithm {
    HmacSha1,
    HmacSha256,
}

#[doc = r"This functions signs a DnsMessage with a key in bytes, and the algName will be used to select the algorithm to encrypt the key."]
fn sign_msg(query_msg:DnsMessage,key:&[u8], alg_name:TsigAlgorithm)->&[u8]{
    let mut new_query_message = query_msg.clone();
    let mut dig_string:&str;
    let mut mac_len:&str;
    let mut a_name: &str;
    let placeholder_hex: String; 
    //TODO: cambiar el match pattern
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
            a_name = "Hmac-Sha1";
        },
        TsigAlgorithm::HmacSha256 => {
            let mut hasher = HmacSha256::new_from_slice(key).expect("HMAC algoritms can take keys of any size");
            hasher.update(&new_query_message.to_bytes()[..]);
            let result = hasher.finalize();

            let code_bytes = result.into_bytes();
            placeholder_hex = hex::encode(code_bytes);
            dig_string = &placeholder_hex;
            mac_len = "32";
            a_name = "Hmac-Sha256";
            
        },
        _ => {panic!("Error: Invalid algorithm")},
    }
    //TODO: agregar los demas valores al msg_bytes !! Yo creo que deben llegar como argumentos
    let mut msg_bytes: String = format!("{}.\n51921\n1234\n{}\n{}\n1234\n0\n0",a_name, mac_len, dig_string);
    return msg_bytes.as_bytes();
}


#[doc = r"This function process a tsig message, checking for errors in the DNS message"]
fn process_tsig(msg: DnsMessage, key: &[u8;32]) -> DnsMessage {
    let mut retmsg = msg.clone();
    let mut addit = retmsg.get_additional();
    println!("{:#?}",addit);

    //sacar el último elemento del vector resource record
    let rr = addit.pop().expect("No additional data");
    let time =SystemTime::now().duration_since(UNIX_EPOCH).expect("no time").as_secs();
    //vector con resource records que son TSIG
    let filtered_tsig:Vec<_> = addit.iter().filter(|tsig| if let Rdata::TSIG(data) = tsig.get_rdata() {true} else {false}).collect();
    let x = if let Rdata::TSIG(data) = addit[addit.len()-1].get_rdata() {true} else {false};
    let rdata = rr.get_rdata();
    let mut time_signed_v = 0;
    let mut fudge = 0;
    let mut rcode =0;
    let mut key = String::from("");
    //Verificar rdata
    match rdata {
        Rdata::TSIG(data) =>{
            time_signed_v = data.get_time_signed();
            fudge = data.get_fudge();
            rcode = data.get_error();
            for elem in data.get_mac(){
                key+=&elem.to_string();
            }
        }
        _ => {
            println!("Bad resource record");
            //TODO: ver/añadir el error del print anterior, especificado en el RFC 8945
            let error_msg = DnsMessage::format_error_msg();
            return error_msg;
        }
        
    }
    //verificar que existen los resource records que corresponden a tsig
    if filtered_tsig.len()>1 || x{
        let error_msg = DnsMessage::format_error_msg();
        return error_msg;
    }
    

    //Verificación de los tiempos de emisión y recepción + fudge del mensaje
    // Según lo especificado en el RFC 8945 5.2.3 time Check and Error Handling
    if (time_signed_v-(fudge as u64))>time || time>(time_signed_v+(fudge as u64)) {
        let mut error_msg = DnsMessage::format_error_msg();
        error_msg.get_header().set_rcode(9);
        let str_whitespaces = "l\n0\n0\n0\n0\n0\n18\n0";
        let resource_record = TSigRdata::rr_from_master_file(
            str_whitespaces.split_whitespace(),
            56, 
            "IN",
            String::from("uchile.cl"),
            String::from("uchile.cl"),
            );
        //TODO: agregar un 6 al campo Other Data de TSig Rdata
        let mut vec = vec![];
        vec.push(resource_record);
        error_msg.add_additionals(vec);
        //TODO: agregar log y añadir el error TSIG 18: BADTIME
        println!("RCODE 9: NOAUTH\n TSIG ERROR 18: BADTIME");
        return error_msg
    }

    retmsg.set_additional(vec![]);
    let mut old_head = retmsg.get_header();
    old_head.set_arcount(0);
    retmsg.set_header(old_head); 
    let algorithm = TsigAlgorithm::HmacSha256;
    //se verifica la autenticidad de la firma
    let digest = sign_msg(retmsg, key, algorithm);
    let digest_string = digest.to_string();
    println!("El dig stirng es: {:#?}" , digest_string);
    let binding = digest.as_bytes();
    let rdata = rr.get_rdata();
    match rdata {
        Rdata::TSIG(tsigrdata) => {
            for i in 0..32 {
                if tsigrdata.get_mac()[i] != binding.clone()[i] {
                    panic!("Wrong signature!");
                }
            }
        },
        _ => {panic!("Bad request")}
    } 

    retmsg
}

                                                            
fn tsig_proccesing_answer(answer_msg:DnsMessage){
    //procesar los errores 
    //new_answer_msg = answer_msg.clone()
}


//Sección de tests unitarios
//ToDo: Crear bien un test que funcione
#[test]
fn ptsig_test(){
    let my_key = b"1201102391287592dsjshno039U021J";
    let alg: TsigAlgorithm = HmacSha256;
    let mut dns_example_msg =     
        DnsMessage::new_query_message(
        DomainName::new_from_string("uchile.cl".to_string()),
        Qtype::A,
        Qclass::IN,
        0,
        false,
        1);
    //prueba de la firma
    let signature = sign_msg(dns_example_msg, my_key, alg);
    //prueba de process_tsig
    let processed_msg = process_tsig(dns_example_msg, my_key);
}
