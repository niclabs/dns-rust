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

#[doc = r"This function a signed message with the key provided"]
fn process_tsig(msg: DnsMessage, key: &[u8;32]) -> DnsMessage {
    let mut retmsg = msg.clone();
    let mut addit = retmsg.get_additional();
    println!("{:#?}",addit);
    let rr = addit.pop().expect("no data in adittional");

    retmsg.set_additional(vec![]);
    let mut old_head = retmsg.get_header();
    old_head.set_arcount(0);
    retmsg.set_header(old_head); 
    let digest = keyed_hash(key, &retmsg.to_bytes()[..]);
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
    new_answer_msg = answer_msg.clone()
}


//Sección de tests unitarios
//ToDo: Revisar
#[test]
fn ptsig_test(){
    let sock: UdpSocket = UdpSocket::bind("127.0.0.1:8001").expect("xd");
    let mut buf:[u8; 4096] = [0; 4096];

    loop {
        match sock.recv_from(& mut buf) {
            Ok((size, addr)) => {
                println!("Llego una peticion de {:?}", addr);
                let msg = DnsMessage::from_bytes(&buf[0..size]).expect("Leyo re mal");
                println!("soy un mensaje {:#?}", msg);
                let response = process_tsig(msg);
                let response = response.to_bytes();
                
                break;
                //sock.send_to(&response[0..size], addr).expect("Fallo al enviar");
            },
            Err(e) => {
                println!("{}",e);
            }
        }
    }
}

