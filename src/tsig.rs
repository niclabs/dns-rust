//aquí debe ir todo lo relacionado a la implementación de tsig como módulo
use crate::domain_name::DomainName;
use crate::message::class_qclass::Qclass;
use crate::message::resource_record::{self, ResourceRecord, ToBytes};
use crate::message::type_qtype::Qtype;
use crate::message::{rdata::tsig_rdata::TSigRdata, DnsMessage};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::message::rdata::{rrsig_rdata, Rdata};
use crypto::hmac::Hmac as crypto_hmac;
use crypto::mac::Mac as crypto_mac;
use hmac::{Hmac, Mac};
use crypto::{sha1::Sha1,sha2::Sha256};
use std::str;
use crate::message::rdata::a_rdata::ARdata;
type HmacSha256 = Hmac<Sha256>;

//TODO: usar arreglar el funcionamiento del enum en sign_msg
enum TsigAlgorithm {
    HmacSha1,
    HmacSha256,
}
#[derive(PartialEq)]
#[derive(Debug)]
enum TsigErrorCode{
    NOERR = 0,
    BADSIG = 16,
    BADKEY = 17,
    BADTIME = 18,
    FORMERR = 1,

}

/*
#[doc = r"This functions signs creates the signature of a DnsMessage with  a  key in bytes and the algName that will be used to encrypt the key."]
fn sign_msg_old(mut query_msg:DnsMessage,key:&[u8], alg_name:TsigAlgorithm)->&[u8]{
    let mut dig_string:&str;
    let mut new_query_message = query_msg.clone();
    let mut mac_len:&str;
    let mut a_name: &str;
    let placeholder_hex: String; 
    let mut additional = query_msg.get_additional();
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

    //TODO: agregar los demas valores al dig_string !! Yo creo que deben llegar como argumentos
    let mut dig_string: String = format!("{}.\n51921\n1234\n{}\n{}\n1234\n0\n0",a_name, mac_len, dig_string);
    
    //se modifica el resource record para añadir el hmac
    let mut rr = additional.pop().expect("Empty Resource Record!");
    let mut rdata:Rdata= rr.get_rdata();
    match rdata {
        Rdata::TSIG(mut data) =>{
            let mut mac: Vec<u8> =data.to_bytes();
            data.set_mac(mac);
        }
        _ => {
            println!("Error: no valid rdata found!");
        }
    }
    let mut vec = vec![];
    vec.push(rr);
    query_msg.add_additionals(vec);
    return dig_string.as_bytes();
} */

// experimental
#[doc = r"This functions signs creates the signature of a DnsMessage with  a  key in bytes and the algName that will be used to encrypt the key."]
fn sign_tsig(query_msg: &mut DnsMessage, key: &[u8], alg_name: TsigAlgorithm, fudge: u16, time_signed: u64) -> Vec<u8> {
    let mut tsig_rd: TSigRdata = TSigRdata::new();
    let mut new_query_message = query_msg.clone();
    let original_id = query_msg.get_query_id();
    match alg_name {
        
        TsigAlgorithm::HmacSha1 => {
            
            let mut hasher = crypto_hmac::new(Sha1::new(), key);
            hasher.input(&new_query_message.to_bytes()[..]);
            let result = hasher.result();
            let mac = result.code();
            //Convertir los bytes brutos a una cadena hexadecimal
            let mac_size = 20;
            let a_name = "Hmac-Sha1".to_lowercase();
            let a_name = DomainName::new_from_string(a_name);
            tsig_rd.set_algorithm_name(a_name);
            tsig_rd.set_mac_size(mac_size);
            tsig_rd.set_mac(mac.to_vec());
            tsig_rd.set_fudge(fudge);
            tsig_rd.set_original_id(original_id);
            tsig_rd.set_time_signed(time_signed);
        },
        TsigAlgorithm::HmacSha256 => {
            let mut hasher = crypto_hmac::new(Sha256::new(), key);
            hasher.input(&new_query_message.to_bytes()[..]);
            let result = hasher.result();
            let mac = result.code();
            //Convertir los bytes brutos a una cadena hexadecimal
            let mac_size = 32;
            let a_name = "Hmac-Sha256".to_lowercase();
            let a_name = DomainName::new_from_string(a_name);
            tsig_rd.set_algorithm_name(a_name);
            tsig_rd.set_mac_size(mac_size);
            tsig_rd.set_mac(mac.to_vec());
            tsig_rd.set_fudge(fudge);
            tsig_rd.set_original_id(original_id);
            tsig_rd.set_time_signed(time_signed);
            
        },
        _ => {panic!("Error: Invalid algorithm")},
    }
    let rr_len = tsig_rd.to_bytes().len() as u16;
    let signature = tsig_rd.get_mac();
    let mut new_rr: ResourceRecord = ResourceRecord::new(Rdata::TSIG(tsig_rd));
    new_rr.set_rdlength(rr_len);
    let mut vec: Vec<ResourceRecord> = vec![];
    vec.push(new_rr);
    query_msg.add_additionals(vec);
    return signature;
}

//Revisa si el nombre de la llave es correcto
fn check_key(key_in_rr:String, key_name:String)-> bool {
    key_in_rr.eq(&key_name)
}

//Verifica que el algoritmo esté disponible, y además esté implementado
fn check_alg_name(alg_name:&String, alg_list: Vec<(String,bool)>) -> bool{
    let mut answer: bool = false;
    for (name,available) in alg_list {
        if name.eq(alg_name){
            if available {
                answer = true;
            }
        }
    }
    return answer
}

//Verifica que los mac sean iguales
fn check_mac(new_mac: Vec<u8>, mac: Vec<u8>) -> bool{
    if mac.len()!=new_mac.len(){
        return false
    }
    for i in 0..mac.len(){
        if new_mac[i]!=mac[i]{
            return false
        }
    }
    true
}

//Verifica el error de la sección 5.2.3 
fn check_time_values(mytime: u64,fudge: u16, time: u64) -> bool {
    let part1 = (time - (fudge as u64)) < mytime;
    let part2 = mytime < (time+(fudge as u64));
    part1 && part2
}

//RFC 8945 5.2 y 5.4
//verificar que existen los resource records que corresponden a tsig
//vector con resource records que son TSIG. Luego se Verifica si hay algún tsig rr
fn check_exists_tsig_rr(add_rec: &Vec<ResourceRecord>) -> bool {
    let filtered_tsig:Vec<_> = add_rec.iter()
                                .filter(|tsig| 
                                if let Rdata::TSIG(data) = tsig.get_rdata() {true}
                                else {false}).collect();

    filtered_tsig.len()==0
}


//Debe haber un único tsig
//Tsig RR debe ser el último en la sección adicional, y debe ser único2
fn check_last_one_is_tsig(add_rec: &Vec<ResourceRecord>) -> bool {
    let filtered_tsig:Vec<_> = add_rec.iter()
                                .filter(|tsig| 
                                if let Rdata::TSIG(data) = tsig.get_rdata() {true}
                                else {false}).collect();
    
    let islast = if let Rdata::TSIG(data) = add_rec[add_rec.len()-1].get_rdata() {false} else {true};

    filtered_tsig.len()>1 || islast
}


#[doc = r"This function process a tsig message, checking for errors in the DNS message"]
fn process_tsig(msg: &DnsMessage,key:&[u8], key_name: String, time: u64,  available_algorithm: Vec<(String, bool)>) -> (bool, TsigErrorCode) {
    let mut retmsg = msg.clone();
    let mut addit = retmsg.get_additional();
    //RFC 8945 5.2 y 5.4
    //verificar que existen los resource records que corresponden a tsig
    //vector con resource records que son TSIG. Luego se Verifica si hay algún tsig rr
    if check_exists_tsig_rr(&addit) {
        println!("RCODE 1: FORMERR");
        return (false, TsigErrorCode::FORMERR);
    }
    
    //Debe haber un único tsig
    //Tsig RR debe ser el último en la sección adicional, y debe ser único
    if check_last_one_is_tsig(&addit) {
        println!("RCODE 1: FORMERR");
        return (false, TsigErrorCode::FORMERR);
    }

    //sacar el último elemento del vector resource record, y disminuir elvalor de ARCOUNT
    let rr_copy = addit.pop().expect("No tsig rr");
    let mut tsig_rr_copy = TSigRdata::new();
    match rr_copy.get_rdata() {
        Rdata::TSIG(data) =>{
            tsig_rr_copy = data;
        }
        _ => {
            println!("error")
        }
    }
    let nuevo_len_arcount = addit.len() as u16;
    let mut new_header = retmsg.get_header();
    new_header.set_arcount(nuevo_len_arcount);
    retmsg.set_header(new_header);
    //RFC 8945 5.2.1
    let name_alg = tsig_rr_copy.get_algorithm_name().get_name();
    let key_in_rr = rr_copy.get_name().get_name();
    let flag = check_alg_name(&name_alg,available_algorithm);
    if !flag {
        println!("RCODE 9: NOAUTH\n TSIG ERROR 17: BADKEY");
        return (false, TsigErrorCode::BADKEY);
    }
    let cond1 = check_key(key_in_rr,key_name);
    if !cond1 {
        println!("RCODE 9: NOAUTH\n TSIG ERROR 17: BADKEY");
        return (false, TsigErrorCode::BADKEY);
    }
    //RFC 8945 5.2.2
    retmsg.set_additional(addit);
    let fudge = tsig_rr_copy.get_fudge();
    let time_signed = tsig_rr_copy.get_time_signed();
    let mac_received = tsig_rr_copy.get_mac();
    let mut new_alg_name: TsigAlgorithm = TsigAlgorithm::HmacSha1;
    match name_alg.as_str() {
        "hmac-sha1" => new_alg_name = TsigAlgorithm::HmacSha1,
        "hmac-sha256" => new_alg_name = TsigAlgorithm::HmacSha256,
        &_ => println!("not supported algorithm")
    }
    let new_mac = sign_tsig(&mut retmsg, key, new_alg_name, fudge, time_signed);
    
    let cond2 = check_mac(new_mac, mac_received);
    if !cond2 {
        println!("RCODE 9: NOAUTH\n TSIG ERROR 16: BADSIG");
        return (false, TsigErrorCode::BADSIG)
    }
    //let mytime = SystemTime::now().duration_since(UNIX_EPOCH).expect("no debería fallar el tiempo");
    let cond3 = check_time_values(time, fudge, time_signed);
    if !cond3 {
        println!("RCODE 9: NOAUTH\n TSIG ERROR 18: BADTIME");
        return (false, TsigErrorCode::BADTIME)
    }
    (true, TsigErrorCode::NOERR)

}

                                                            
fn tsig_proccesing_answer(answer_msg:DnsMessage){
    //procesar los errores 
    //new_answer_msg = answer_msg.clone()
}


//Sección de tests unitarios
//ToDo: Crear bien un test que funcione

#[test]

fn check_process_tsig_exists() {
    //Server process
    let mut response = DnsMessage::new_response_message(String::from("test.com"), "NS", "IN", 1, true, 1);
    //Client process
    let key_name:String = "".to_string();
    let mut lista :Vec<(String, bool)>  = vec![];
    let server_key = b"1234567890";
    lista.push((String::from("hmac-sha256"),true));
    let (answer, error) = process_tsig(& response, server_key, key_name, 21010, lista);
    assert!(!answer);
    assert_eq!(error,TsigErrorCode::FORMERR);
}

#[test]
fn check_process_tsig_exists2() {
    //Server process
    let mut response = DnsMessage::new_response_message(String::from("test.com"), "NS", "IN", 1, true, 1);
    let server_key = b"1234567890";
    let alg_name = TsigAlgorithm::HmacSha256;
    let alg_name2 = TsigAlgorithm::HmacSha256;
    let fudge = 300;
    let time_signed = 21000;
    sign_tsig(&mut response, server_key, alg_name, fudge, time_signed);
    sign_tsig(&mut response, server_key, alg_name2, fudge, time_signed);
    let mut response_capture = response.clone();
    //Client process
    let key_name:String = "".to_string();
    let mut lista :Vec<(String, bool)>  = vec![];
    lista.push((String::from("hmac-sha256"),true));
    let (answer, error) = process_tsig(& response_capture, server_key, key_name, 21010, lista);
    assert!(!answer);
    assert_eq!(error, TsigErrorCode::FORMERR);
}

#[test]
fn check_process_tsig_exists3(){
    //Server process
    let mut response = DnsMessage::new_response_message(String::from("test.com"), "NS", "IN", 1, true, 1);
    let server_key = b"1234567890";
    let alg_name = TsigAlgorithm::HmacSha256;
    let fudge = 300;
    let time_signed = 21000;
    sign_tsig(&mut response, server_key, alg_name, fudge, time_signed);
    //necesito agregar algo más en el additional
    let mut new_additional = Vec::<ResourceRecord>::new();
    let a_rdata5 = Rdata::A(ARdata::new());
    let rr5 = ResourceRecord::new(a_rdata5);
    new_additional.push(rr5);
    response.add_additionals(new_additional);
    let mut response_capture = response.clone();
    //Client process
    let key_name:String = "".to_string();
    let mut lista :Vec<(String, bool)>  = vec![];
    lista.push((String::from("hmac-sha256"),true));
    let (answer, error) = process_tsig(& response_capture, server_key, key_name, 21010, lista);
    assert!(!answer);
    assert_eq!(error, TsigErrorCode::FORMERR);
}
#[test]
fn check_process_tsig_alg_name() {
    //Server process
    let mut response = DnsMessage::new_response_message(String::from("test.com"), "NS", "IN", 1, true, 1);
    let server_key = b"1234567890";
    let alg_name = TsigAlgorithm::HmacSha256;
    let fudge = 300;
    let time_signed = 21000;
    sign_tsig(&mut response, server_key, alg_name, fudge, time_signed);
    let mut response_capture = response.clone();
    //Client process
    let key_name:String = "".to_string();
    let mut lista :Vec<(String, bool)>  = vec![];
    //suponemos que hmacsha256 no está disponible
    lista.push((String::from("hmac-sha1"),true));
    let (answer, error) = process_tsig(& response_capture, server_key, key_name, 21010, lista);
    assert!(!answer);
    assert_eq!(error,TsigErrorCode::BADKEY);
}
#[test]
fn check_process_tsig_alg_name2() {
    //Server process
    let mut response = DnsMessage::new_response_message(String::from("test.com"), "NS", "IN", 1, true, 1);
    let server_key = b"1234567890";
    let alg_name = TsigAlgorithm::HmacSha256;
    let fudge = 300;
    let time_signed = 21000;
    sign_tsig(&mut response, server_key, alg_name, fudge, time_signed);
    let mut response_capture = response.clone();
    //Client process
    let key_name:String = "".to_string();
    let mut lista :Vec<(String, bool)>  = vec![];
    //suponemos que reconocemos hmac-sha256, pero no está implementado
    lista.push((String::from("hmac-sha256"),false));
    let (answer, error) = process_tsig(& response_capture, server_key, key_name, 21010, lista);
    assert!(!answer);
    assert_eq!(error,TsigErrorCode::BADKEY);
}
#[test]
fn check_process_tsig_key(){
    //Server process
    let mut response = DnsMessage::new_response_message(String::from("test.com"), "NS", "IN", 1, true, 1);
    let server_key = b"1234567890";
    let alg_name = TsigAlgorithm::HmacSha256;
    let fudge = 300;
    let time_signed = 21000;
    sign_tsig(&mut response, server_key, alg_name, fudge, time_signed);
    let mut response_capture = response.clone();
    //Client process
    let key_name:String = "different".to_string();
    let mut lista :Vec<(String, bool)>  = vec![];
    //suponemos que reconocemos hmac-sha256, pero no está implementado
    lista.push((String::from("hmac-sha256"),false));
    let (answer, error) = process_tsig(& response_capture, server_key, key_name, 21010, lista);
    assert!(!answer);
    assert_eq!(error,TsigErrorCode::BADKEY);
}

#[test]
fn check_proces_tsig_badtime(){
 //Server process
 let mut response = DnsMessage::new_response_message(String::from("test.com"), "NS", "IN", 1, true, 1);
 let server_key = b"1234567890";
 let alg_name = TsigAlgorithm::HmacSha256;
 let fudge = 300;
 let time_signed = 21000;
 sign_tsig(&mut response, server_key, alg_name, fudge, time_signed);
 let mut response_capture = response.clone();
 //Client process
 let key_name:String = "".to_string();
 let mut lista :Vec<(String, bool)>  = vec![];
 //suponemos que reconocemos hmac-sha256, pero no está implementado
 lista.push((String::from("hmac-sha256"),true));
 let (answer, error) = process_tsig(& response_capture, server_key, key_name, 22010, lista);
 assert!(!answer);
 assert_eq!(error,TsigErrorCode::BADTIME);
}
#[test]
fn check_process_tsig() {
    //Server process
    let mut response = DnsMessage::new_response_message(String::from("test.com"), "NS", "IN", 1, true, 1);
    let server_key = b"1234567890";
    let alg_name = TsigAlgorithm::HmacSha256;
    let fudge = 300;
    let time_signed = 21000;
    sign_tsig(&mut response, server_key, alg_name, fudge, time_signed);
    let mut response_capture = response.clone();
    //Client process
    let key_name:String = "".to_string();
    let mut lista :Vec<(String, bool)>  = vec![];
    lista.push((String::from("hmac-sha256"),true));
    let (answer, error) = process_tsig(& response_capture, server_key, key_name, 21010, lista);
    assert!(answer);
    assert_eq!(error,TsigErrorCode::NOERR);
}
#[test]
fn check_signed_tsig() {
    let key = b"1234567890";
    let alg_name = TsigAlgorithm::HmacSha1;
    let fudge = 0;
    let time_signed = 0;
    let id = 6502; 
    let mut q = DnsMessage::new_query_message(
        DomainName::new_from_str("uchile.cl"),
        Qtype::A,
        Qclass::ANY, 
        0, 
        false,
        id
    );
    let q_for_mac = q.clone();
    
    let firma_a_comparar = sign_tsig(&mut q, key, alg_name, fudge, time_signed);

    let mut hasher = crypto_hmac::new(Sha1::new(), key);
    hasher.input(&q_for_mac.to_bytes()[..]);
    
    let result = hasher.result();
    let mac_to_cmp = result.code();

    let rr = q.get_additional().pop().expect("Should be a tsig");
    match rr.get_rdata() {
        Rdata::TSIG(data) => {
            assert_eq!(data.get_algorithm_name(), DomainName::new_from_str("hmac-sha1"));
            assert_eq!(data.get_time_signed(), time_signed);
            assert_eq!(data.get_fudge() , fudge);
            assert_eq!(data.get_mac_size(), 20);
            assert_eq!(data.get_original_id(), id);
            assert_eq!(data.get_error(), 0);
            assert_eq!(data.get_other_len(), 0);
            assert!(data.get_other_data().is_empty());
        },
        _ =>{
            assert!(false);
        }
    }
    println!("Comparando el mac");
    for i in 0..mac_to_cmp.len() {
        assert_eq!(mac_to_cmp[i], firma_a_comparar[i]);
    }
}
