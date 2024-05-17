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

type HmacSha256 = Hmac<Sha256>;

//TODO: usar arreglar el funcionamiento del enum en sign_msg
enum TsigAlgorithm {
    HmacSha1,
    HmacSha256,
}
enum TsigErrorCode{
    BADSIG = 16,
    BADDKEY = 17,
    BADTIME = 18,

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
fn sign_tsig(mut query_msg: DnsMessage, key: &[u8], alg_name: TsigAlgorithm, fudge: u16, time_signed: u64) -> Vec<u8> {
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
            let a_name = "Hmac-Sha1".to_uppercase();
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
            let a_name = "Hmac-Sha256".to_uppercase();
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

//TODO: terminar función keycheck
fn check_key(alg_name: String,key_in_rr:String,key_name:String,flag_check_alg:bool)-> bool {
    let mut answer = true; 

    if !key_in_rr.eq(&key_name) {
        answer=false;
    }
    return answer
}

//Verifica que el algoritmo esté disponible, y además esté implementado
fn check_alg_name(alg_name:String, alg_list: Vec<(String,bool)>) -> bool{
    let mut answer: bool = false;
    for (name,available) in alg_list {
        if name.eq(&alg_name){
            if available {
                answer = true;
            }
        }
    }
    return answer
}

//RFC 8945 5.2 y 5.4
//verificar que existen los resource records que corresponden a tsig
//vector con resource records que son TSIG. Luego se Verifica si hay algún tsig rr
fn check_exists_tsig_rr(add_rec: &Vec<ResourceRecord>) -> bool {
    let filtered_tsig:Vec<_> = add_rec.iter()
                                .filter(|tsig| 
                                if let Rdata::TSIG(data) = tsig.get_rdata() {true}
                                else {false}).collect();

    (filtered_tsig.len()==0)
}


//Debe haber un único tsig
//Tsig RR debe ser el último en la sección adicional, y debe ser único2
fn check_last_one_is_tsig(add_rec: &Vec<ResourceRecord>) -> bool {
    let filtered_tsig:Vec<_> = add_rec.iter()
                                .filter(|tsig| 
                                if let Rdata::TSIG(data) = tsig.get_rdata() {true}
                                else {false}).collect();
    
    let islast = if let Rdata::TSIG(data) = add_rec[add_rec.len()-1].get_rdata() {false} else {true};

    (filtered_tsig.len()>1 || islast)
}


#[doc = r"This function process a tsig message, checking for errors in the DNS message"]
fn process_tsig(msg: DnsMessage, key_name: String, time: u64,  available_algorithm: Vec<(String, bool)>) -> bool {
    let mut retmsg = msg.clone();
    let mut addit = retmsg.get_additional();
    
    
    //RFC 8945 5.2 y 5.4
    //verificar que existen los resource records que corresponden a tsig
    //vector con resource records que son TSIG. Luego se Verifica si hay algún tsig rr
    if check_exists_tsig_rr(&addit) {
        println!("RCODE 1: FORMERR");
        return false;
    }
    
    //Debe haber un único tsig
    //Tsig RR debe ser el último en la sección adicional, y debe ser único
    if check_last_one_is_tsig(&addit) {
        println!("RCODE 1: FORMERR");
        return false;
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
    let new_header = msg.get_header();
    new_header.set_arcount(nuevo_len_arcount);
    msg.set_header(new_header);
    //RFC 8945 5.2.1
    let name_alg = tsig_rr_copy.get_algorithm_name().get_name();
    let key_in_rr = rr_copy.get_name().get_name();
    let flag = check_alg_name(name_alg,available_algorithm);
    let cond1 = check_key(name_alg,key_in_rr,key_name,flag);
    if cond1 {
        println!("RCODE 9: NOAUTH\n TSIG ERROR 17: BADKEY");
        return false;
    }
    //TODO: hacer los demas checkeos
    //let cond2 = check_mac();
    //let cond3 = check_time_values();
    //let cond4 = check_truncation_policy();

    //let rdata = rr.get_rdata();
    let mut time_signed_v = 0;
    let mut fudge = 0;
    //let mut rcode =0;
    //let mut n_key = String::from("");
    ////Verificar rdata
    //match rdata {
    //    Rdata::TSIG(data) =>{
    //        time_signed_v = data.get_time_signed();
    //        fudge = data.get_fudge();
    //        rcode = data.get_error();
    //        for elem in data.get_mac(){
    //            n_key+=&elem.to_string();
    //        }
    //    }
    //    _ => {
    //        println!("Bad resource record");
    //        //TODO: ver/añadir el error del print anterior, especificado en el RFC 8945
    //        let error_msg = DnsMessage::format_error_msg();
    //        return error_msg;
    //    }
    //    
    //}


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
    //TODO: extraer los siguientes valores del rdata del mensaje a verificar
    let msg_time=  SystemTime::now().duration_since(UNIX_EPOCH).expect("no existo").as_secs();
    let msg_fudge = 1000;
    //se verifica la autenticidad de la firma
    let digest = sign_tsig(retmsg.clone(), key, algorithm,msg_time as u16,msg_fudge);
    let digest_string = &digest;
    println!("El dig stirng es: {:#?}" , digest_string);
    let binding = &digest;
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
    let my_key = b"1201102391287592dsjshno039U021Jg";
    let my_short_key = b"1201102391287592dsjs";
    let alg: TsigAlgorithm = TsigAlgorithm::HmacSha256;
    let mut dns_example_msg =     
        DnsMessage::new_query_message(
        DomainName::new_from_string("uchile.cl".to_string()),
        Qtype::A,
        Qclass::IN,
        0,
        false,
        1);
    let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("no existo").as_secs();
    //prueba de la firma. sign_msg calcula la firma, la añade al resource record del mensaje y retorna una copia
    let _signature = sign_tsig(dns_example_msg.clone(), my_key, alg,1000,time );
    let _sha1signature = sign_tsig(dns_example_msg.clone(),my_short_key, TsigAlgorithm::HmacSha1,1000, time);
    let signature = match str::from_utf8(&_signature){
        Ok(v) =>v,
        Err(e) =>panic!("Invalid  UTF-( sequence: {}",e)
    };
    let sha1signature = match str::from_utf8(&_sha1signature){
        Ok(v) =>v,
        Err(e) =>panic!("Invalid  UTF-( sequence: {}",e)
    };
    println!("SHA-256: {}",signature);
    println!("SHA-1: {}",sha1signature);

    
    //prueba de process_tsig (la idea es usar la firma anterior, añadirla a dns_example_msg y verificarla con my_key)
    //let _processed_msg = process_tsig(dns_example_msg, my_key);

}
