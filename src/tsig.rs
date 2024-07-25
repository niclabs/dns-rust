pub mod tsig_algorithm;

use crypto::mac::MacResult;
use crate::domain_name::DomainName;
use std::time::SystemTime;
use crate::message::rclass::Rclass;
use crate::message::resource_record::{ResourceRecord, ToBytes};

use crate::message::{rdata::tsig_rdata::TSigRdata, DnsMessage,};
use crate::message::rdata::Rdata;
use crypto::hmac::Hmac as crypto_hmac;
use crypto::mac::Mac as crypto_mac;
use crypto::{sha1::Sha1,sha2::Sha256};
use tsig_algorithm::TsigAlgorithm;
use crate::message::rcode::Rcode;


//TODO: Encontrar alguna manera de pasar una referencia Digest u Hmac de un algoritmo no especificado
// función auxiliar para evitar la redundancia de código en sign_tsig
fn set_tsig_rd(name: String, original_id: u16, result: MacResult,
               fudge: u16, time_signed: u64, mac_size: u16) -> TSigRdata{
    let mut tsig_rd: TSigRdata = TSigRdata::new();
    let mac = result.code();

    //Convertir los bytes brutos a una cadena hexadecimal
    let a_name = name.to_lowercase();
    let a_name = DomainName::new_from_string(a_name);
    //añadir los valores correspondientes al tsig_rd
    tsig_rd.set_algorithm_name(a_name);
    tsig_rd.set_mac_size(mac_size);
    tsig_rd.set_mac(mac.to_vec());
    tsig_rd.set_fudge(fudge);
    tsig_rd.set_original_id(original_id);
    tsig_rd.set_time_signed(time_signed);

    return tsig_rd;
}
//TODO: crear una función para simplificar la extracción de bits paa simplificar código
// This function extracts the digest 
#[doc = r"This function recives a DNS message and generate the digest da. Requested by RFC 8945 4.3.3 "]
pub fn get_digest_request(mac: Vec<u8> ,dns_msg: Vec<u8>, tsig_rr: ResourceRecord) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];

    if mac.len() != 0 {
        let mac_len = mac.len() as u16;
        let bytes_mac_len = mac_len.to_be_bytes();
        res.push(bytes_mac_len[0]);
        res.push(bytes_mac_len[1]);
        res.extend(mac.clone());
    }
    res.extend(dns_msg.clone());
    let tsig_rdata = tsig_rr.get_rdata();
    res.extend(tsig_rr.get_name().to_bytes());
    //The below shifts are meant to correctly retreive theby
    //processing TSIG RR
    let rclass_bytes: u16 = Rclass::from(tsig_rr.get_rclass()).into();
    let rclass_ubyte = (rclass_bytes >> 8) as u8;
    let rclass_lbyte = rclass_bytes as u8;
    res.push(rclass_ubyte);
    res.push(rclass_lbyte);

    let rclass_ttl:  u32 = tsig_rr.get_ttl();
    let r_ttl1 = (rclass_ttl >> 24) as u8;
    let r_ttl2 = (rclass_ttl >> 16) as u8;
    let r_ttl3 = (rclass_ttl >>  8) as u8;
    let r_ttl4 = rclass_ttl as u8;
    res.push(r_ttl1);
    res.push(r_ttl2);
    res.push(r_ttl3);
    res.push(r_ttl4);

    //processing TSIG RDATA
    let tsig_rd = match tsig_rdata {
        Rdata::TSIG(tsig_rd) => tsig_rd,
        _ => panic!()
    };
    let a_name = tsig_rd.get_algorithm_name().to_bytes();
    // Remember that time_signed is u48
    let tsig_rd_time_signed: u64 = tsig_rd.get_time_signed();
    let tsig_rd_fudge: u16 = tsig_rd.get_fudge();
    let tsig_rd_error: u16= tsig_rd.get_error();
    let tsig_rd_other_len: u16 =  tsig_rd.get_other_len();
    let tsig_rd_other_data = tsig_rd.get_other_data();

    res.extend(a_name);

    let time_s1 = (tsig_rd_time_signed >> 40) as u8;
    let time_s2 = (tsig_rd_time_signed >> 32) as u8;
    let time_s3 = (tsig_rd_time_signed >> 24) as u8;
    let time_s4 = (tsig_rd_time_signed >> 16) as u8;
    let time_s5 = (tsig_rd_time_signed >> 8) as u8;
    let time_s6 = (tsig_rd_time_signed) as u8;
    res.push(time_s1);
    res.push(time_s2);
    res.push(time_s3);
    res.push(time_s4);
    res.push(time_s5);
    res.push(time_s6);

    let fudge1 = (tsig_rd_fudge >> 8) as u8;
    let fudge2 = (tsig_rd_fudge) as u8;
    res.push(fudge1);
    res.push(fudge2);

    let error1 = (tsig_rd_error >> 8) as u8;
    let error2 = (tsig_rd_error) as u8;
    res.push(error1);
    res.push(error2);

    let otherl1 = (tsig_rd_other_len >> 8) as u8;
    let otherl2 = (tsig_rd_other_len) as u8;
    res.push(otherl1);
    res.push(otherl2);

    res.extend(tsig_rd_other_data);

    return res;
}

fn digest(bytes: Vec<u8>, tsig_algorithm: TsigAlgorithm, key: Vec<u8>) -> Vec<u8>{
    match tsig_algorithm {
        TsigAlgorithm::HmacSha1 => {

            //new_query_message.push();
            let mut hasher = crypto_hmac::new(Sha1::new(), &key);
            hasher.input(&bytes[..]);
            hasher.result().code().to_vec()
        },
        TsigAlgorithm::HmacSha256 => {
            let mut hasher = crypto_hmac::new(Sha256::new(), &key);
            hasher.input(&bytes[..]);
            hasher.result().code().to_vec()
        }
        TsigAlgorithm::UNKNOWN(a) => {
            panic!("Unknown algorithm {}", a);
        }
    }
}

//RFC 8945, section 5.1
#[doc = r"This function creates the signature of a DnsMessage with  a  key in bytes and the algName that will be used to encrypt the key."]
pub fn sign_tsig(query_msg: &mut DnsMessage, key: &[u8], alg_name: TsigAlgorithm,
                 fudge: u16, time_signed: u64, key_name: String, mac_request: Vec<u8>) -> Vec<u8> {
    let tsig_rd: TSigRdata;
    let new_query_message = query_msg.clone();
    let original_id = query_msg.get_query_id();
    let alg_name_str = String::from(alg_name.clone());
    let tsig_rr= set_tsig_vars(alg_name_str.as_str(), key_name.as_str(),
                               time_signed, fudge);
    let digest_comp = get_digest_request(mac_request, new_query_message.to_bytes(),
                                         tsig_rr);
    match alg_name {
        
        TsigAlgorithm::HmacSha1 => {

            //new_query_message.push();
            let mut hasher = crypto_hmac::new(Sha1::new(), key);
            hasher.input(&digest_comp[..]);
            let result = hasher.result();
            tsig_rd = set_tsig_rd( 
                "hmac-sha1".to_lowercase(), 
                original_id,
                result,
                fudge, 
                time_signed,
                 20);
            
        },
        TsigAlgorithm::HmacSha256 => {
            let mut hasher = crypto_hmac::new(Sha256::new(), key);
            hasher.input(&digest_comp[..]);
            let result = hasher.result();
            tsig_rd = set_tsig_rd( 
                "hmac-sha256".to_lowercase(),
                original_id,
                result,
                fudge, 
                time_signed,
                32);
            
        },
        TsigAlgorithm::UNKNOWN(a) => {
            panic!("Unknown algorithm {}", a);
        }
    }
    let rr_len = tsig_rd.to_bytes().len() as u16;
    let signature = tsig_rd.get_mac();
    let mut new_rr: ResourceRecord = ResourceRecord::new(Rdata::TSIG(tsig_rd));
    new_rr.set_name(DomainName::new_from_string(key_name));
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
                                if let Rdata::TSIG(_) = tsig.get_rdata() {true}
                                else {false}).collect();

    filtered_tsig.len()==0
}


//Debe haber un único tsig
//Tsig RR debe ser el último en la sección adicional, y debe ser único2
fn check_last_one_is_tsig(add_rec: &Vec<ResourceRecord>) -> bool {
    let filtered_tsig:Vec<_> = add_rec.iter()
                                .filter(|tsig| 
                                if let Rdata::TSIG(_) = tsig.get_rdata() {true}
                                else {false}).collect();
    
    let islast = if let Rdata::TSIG(_) = add_rec[add_rec.len()-1].get_rdata() {false} else {true};

    filtered_tsig.len()>1 || islast
}

#[doc = r"This function process a tsig message, checking for errors in the DNS message"]
pub fn process_tsig(msg: &DnsMessage, key:&[u8], key_name: String, time: u64,
                    available_algorithm: Vec<(String, bool)>, mac_to_process: Vec<u8>) -> (bool, Rcode) {
    let mut retmsg = msg.clone();
    let mut addit = retmsg.get_additional();
    //RFC 8945 5.2 y 5.4
    //verificar que existen los resource records que corresponden a tsig
    //vector con resource records que son TSIG. Luego se Verifica si hay algún tsig rr
    if check_exists_tsig_rr(&addit) {
        println!("RCODE 1: FORMERR");
        return (false, Rcode::FORMERR);
    }
    
    //Debe haber un único tsig
    //Tsig RR debe ser el último en la sección adicional, y debe ser único
    if check_last_one_is_tsig(&addit) {
        println!("RCODE 1: FORMERR");
        return (false, Rcode::FORMERR);
    }

    //sacar el último elemento del vector resource record, y disminuir elvalor de ARCOUNT
    let rr_copy = addit.pop().expect("No tsig rr");
    let tsig_rr_copy: TSigRdata;
    match rr_copy.get_rdata() {
        Rdata::TSIG(data) =>{
            tsig_rr_copy = data;
        }
        _ => {
            println!("error");
            unimplemented!("TODO: error code if last rr is not tsig; FORMERR")
        }
    }

    //RFC 8945 5.2.1
    let key_in_rr = rr_copy.get_name().get_name();
    let name_alg = tsig_rr_copy.get_algorithm_name().get_name();

    let flag = check_alg_name(&name_alg, available_algorithm);
    if !flag {
        println!("RCODE 9: NOAUTH\n TSIG ERROR 17: BADKEY");
        return (false, Rcode::BADKEY);
    }

    let cond1 = check_key(key_in_rr.clone(), key_name.clone());
    if !cond1 {
        println!("RCODE 9: NOAUTH\n TSIG ERROR 17: BADKEY");
        println!("key in rr: {:?} key given {:?}", key_in_rr, key_name);
        return (false, Rcode::BADKEY);
    }

    //RFC 8945 5.2.2
    //retmsg.set_additional(addit);
    let fudge = tsig_rr_copy.get_fudge();
    let time_signed = tsig_rr_copy.get_time_signed();
    let mac_received = tsig_rr_copy.get_mac();
    let mut new_alg_name: TsigAlgorithm = TsigAlgorithm::HmacSha1;
    match name_alg.as_str() {
        "hmac-sha1" => new_alg_name = TsigAlgorithm::HmacSha1,
        "hmac-sha256" => new_alg_name = TsigAlgorithm::HmacSha256,
        &_ => println!("Not supported algorithm")
    }

    //let nuevo_len_arcount = addit.len() as u16;
    //let mut new_header = retmsg.get_header();
    //new_header.set_arcount(nuevo_len_arcount);
    //retmsg.set_header(new_header);
    retmsg.set_additional(addit);
    retmsg.update_header_counters();

    // This gets the bytes to use the function and generate the digest
    let bytes_to_hash = get_digest_request(mac_to_process, retmsg.to_bytes(), rr_copy);

    let new_mac = digest(bytes_to_hash, new_alg_name, key.to_vec());

    let cond2 = check_mac(new_mac, mac_received);

    if !cond2 {
        println!("RCODE 9: NOAUTH\n TSIG ERROR 16: BADSIG");
        return (false, Rcode::BADSIG)
    }
    //let mytime = SystemTime::now().duration_since(UNIX_EPOCH).expect("no debería fallar el tiempo");
    let cond3 = check_time_values(time, fudge, time_signed);
    if !cond3 {
        println!("RCODE 9: NOAUTH\n TSIG ERROR 18: BADTIME");
        return (false, Rcode::BADTIME)
    }
    (true, Rcode::NOERROR)

}

pub fn immediate_process_tsig(msg: &DnsMessage, key:&[u8], key_name: String,
    available_algorithm: Vec<(String, bool)>, mac_to_process: Vec<u8>) -> (bool, Rcode) {
    
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    process_tsig(msg, key, key_name, time, available_algorithm, mac_to_process)
}

//Auxiliar function to create the TSIG variables and resource recrods
#[doc= r"This function helps to set create a partial TSIG resource record on  a DNS query"]
fn set_tsig_vars(alg_name: &str, name: &str, time_signed: u64, fudge: u16) -> ResourceRecord{
    //TSIG Variables
    // TSIG RDATA
    let mut tsig_rd: TSigRdata = TSigRdata::new();
    tsig_rd.set_algorithm_name(DomainName::new_from_str(alg_name));
    tsig_rd.set_time_signed(time_signed);
    tsig_rd.set_fudge(fudge);
    tsig_rd.set_error(0);
    tsig_rd.set_other_len(0);
    // TSIG RR
    let mut tsig_rr = ResourceRecord::new(Rdata::TSIG(tsig_rd));
    tsig_rr.set_name(DomainName::new_from_str(name));
    //tsig_rr.set_rclass(Rclass::ANY);
    tsig_rr.set_ttl(0);

    return tsig_rr
}                                                 

//Sección de tests unitarios

#[cfg(test)]
mod tsig_test {
    use super::*;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rrtype::Rrtype;

    #[test]
    fn check_process_tsig_exists() {
        //Server process
        let response = DnsMessage::new_response_message(String::from("test.com"), "NS", "IN", 1, true, 1);
        //Client process
        let key_name:String = "".to_string();
        let mut lista :Vec<(String, bool)>  = vec![];
        let server_key = b"1234567890";
        lista.push((String::from("hmac-sha256"),true));
        let (answer, error) = process_tsig(& response, server_key, key_name, 21010, lista, vec![]);
        assert!(!answer);
        assert_eq!(error,Rcode::FORMERR);
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
        let key_name = "".to_string();

        sign_tsig(&mut response, server_key, alg_name, fudge, time_signed, key_name.clone(), vec![]);
        let mut response_capture = response.clone();
        sign_tsig(&mut response_capture, server_key, alg_name2, fudge, time_signed, key_name.clone(), vec![]);
        //Client process
        let key_name:String = "".to_string();
        let mut lista :Vec<(String, bool)>  = vec![];
        lista.push((String::from("hmac-sha256"),true));
        let (control_answer, _) = process_tsig(& response, server_key, key_name.clone(),21010, lista.clone(), vec![]);
        assert!(control_answer);
        let (answer, error) = process_tsig(& response_capture, server_key, key_name, 21010, lista, vec![]);
        assert!(!answer);
        assert_eq!(error, Rcode::FORMERR);
    }

    // verificar que no se haya añadido otro resource record en el additionals luego de añadir un tsig_rr
    #[test]
    fn check_process_tsig_exists3(){
        //Server process
        let mut response = DnsMessage::new_response_message(String::from("test.com"), "NS", "IN", 1, true, 1);
        let server_key = b"1234567890";
        let alg_name = TsigAlgorithm::HmacSha256;
        let fudge = 300;
        let time_signed = 21000;
        let key_name = "";
        //se crea un rr TSIG que se añadirá en adittionals
        sign_tsig(&mut response, server_key, alg_name, fudge, time_signed, key_name.to_string(), vec![]);

        //se agrega otro resource record en el additional...
        let mut new_additional = Vec::<ResourceRecord>::new();
        let a_rdata5 = Rdata::A(ARdata::new());
        let rr5 = ResourceRecord::new(a_rdata5);
        new_additional.push(rr5);
        response.add_additionals(new_additional);
        let response_capture = response.clone();

        //Client process
        let key_name:String = "".to_string();
        let mut lista :Vec<(String, bool)>  = vec![];
        lista.push((String::from("hmac-sha256"),true));
        let (answer, error) = process_tsig(& response_capture, server_key, key_name, 21010, lista, vec![]);
        assert!(!answer);
        assert_eq!(error, Rcode::FORMERR);
    }
    #[test]
    fn check_process_tsig_alg_name() {
        //Server process
        let mut response = DnsMessage::new_response_message(String::from("test.com"), "NS", "IN", 1, true, 1);
        let server_key = b"1234567890";
        let alg_name = TsigAlgorithm::HmacSha256;
        let fudge = 300;
        let time_signed = 21000;
        let key_name = "".to_string();
        sign_tsig(&mut response, server_key, alg_name, fudge, time_signed, key_name, vec![]);
        let response_capture = response.clone();
        //Client process
        let key_name:String = "".to_string();
        let mut lista :Vec<(String, bool)>  = vec![];
        //suponemos que hmacsha256 no está disponible
        lista.push((String::from("hmac-sha1"),true));
        let (answer, error) = process_tsig(& response_capture, server_key, key_name, 21010, lista, vec![]);
        assert!(!answer);
        assert_eq!(error,Rcode::BADKEY);
    }
    #[test]
    fn check_process_tsig_alg_name2() {
        //Server process
        let mut response = DnsMessage::new_response_message(String::from("test.com"), "NS", "IN", 1, true, 1);
        let server_key = b"1234567890";
        let alg_name = TsigAlgorithm::HmacSha256;
        let fudge = 300;
        let time_signed = 21000;
        let key_name = "".to_string();
        sign_tsig(&mut response, server_key, alg_name, fudge, time_signed, key_name, vec![]);
        let response_capture = response.clone();
        //Client process
        let key_name:String = "".to_string();
        let mut lista :Vec<(String, bool)>  = vec![];
        //suponemos que reconocemos hmac-sha256, pero no está implementado
        lista.push((String::from("hmac-sha256"),false));
        let (answer, error) = process_tsig(& response_capture, server_key, key_name, 21010, lista, vec![]);
        assert!(!answer);
        assert_eq!(error,Rcode::BADKEY);
    }
    #[test]
    fn check_process_tsig_key(){
        //Server process
        let mut response = DnsMessage::new_response_message(String::from("test.com"), "NS", "IN", 1, true, 1);
        let server_key = b"1234567890";
        let alg_name = TsigAlgorithm::HmacSha256;
        let fudge = 300;
        let time_signed = 21000;
        let key_name = "".to_string();
        sign_tsig(&mut response, server_key, alg_name, fudge, time_signed, key_name, vec![]);
        let response_capture = response.clone();
        //Client process
        let key_name:String = "different".to_string();
        let mut lista :Vec<(String, bool)>  = vec![];
        //suponemos que reconocemos hmac-sha256, pero no está implementado
        lista.push((String::from("hmac-sha256"),false));
        let (answer, error) = process_tsig(& response_capture, server_key, key_name, 21010, lista, vec![]);
        assert!(!answer);
        assert_eq!(error,Rcode::BADKEY);
    }
    //TODO: completar este test, hay cosas que faltan por implementar
    #[test]
    fn check_process_tsig_badsign(){
        // Se establece un DnsMessage de prueba. Lo firmaremos, alteraremos la firma generada y esperamos recibir un error BADSIGN
        let mut msg1 = DnsMessage::new_response_message(String::from("test.com"), "NS", "IN", 1, true, 1);
        let key = b"1234567890";
        let alg_name = TsigAlgorithm::HmacSha1;
        let fudge = 1000;
        let time_signed = 210000000;
        let key_name = "".to_string();
        // se firma el mensaje con algoritmo SHA-1
        sign_tsig(& mut msg1, key, alg_name, fudge, time_signed, key_name, vec![]);
        let mut lista :Vec<(String, bool)>  = vec![];
        lista.push((String::from("hmac-sha1"),true));
        lista.push((String::from("hmac-sha256"),true));
        // se verifica que el mensaje está firmado, pero se usa otra key
        let key_name = "".to_string();
        let key2 = b"12345678909";
        let (_, error) = process_tsig(&mut msg1, key2, key_name, time_signed,lista, vec![]);
        assert_eq!(error,Rcode::BADSIG);
    }
    #[test]
    fn check_proces_tsig_badtime(){
        //Server process
        let mut response = DnsMessage::new_response_message(String::from("test.com"), "NS", "IN", 1, true, 1);
        let server_key = b"1234567890";
        let alg_name = TsigAlgorithm::HmacSha256;
        let fudge = 300;
        let time_signed = 21000;
        let key_name = "".to_string();
        sign_tsig(&mut response, server_key, alg_name, fudge, time_signed, key_name, vec![]);
        let response_capture = response.clone();
        //Client process
        let key_name:String = "".to_string();
        let mut lista :Vec<(String, bool)>  = vec![];
        //suponemos que reconocemos hmac-sha256, pero no está implementado
        lista.push((String::from("hmac-sha256"),true));
        let (answer, error) = process_tsig(& response_capture, server_key, key_name,
                                        22010, lista, vec![]);
        assert!(!answer);
        assert_eq!(error,Rcode::BADTIME);
    }
    #[test]
    fn check_process_tsig() {
        //sender process
        let mut response = DnsMessage::new_response_message(String::from("test.com"), "NS", "IN", 1, true, 1);
        let server_key = b"1234567890";
        let alg_name = TsigAlgorithm::HmacSha256;
        let fudge = 300;
        let time_signed = 21000;
        let key_name = "".to_string();
        sign_tsig(&mut response, server_key, alg_name, fudge, time_signed, key_name, vec![]);
        let response_capture = response.clone();
        //recv process
        let key_name:String = "".to_string();
        let mut lista :Vec<(String, bool)>  = vec![];
        lista.push((String::from("hmac-sha256"),true));
        let (answer, error) = process_tsig(& response_capture, server_key, key_name,
                                        21010, lista, vec![]);
        assert!(answer);
        assert_eq!(error,Rcode::NOERROR);
    }
    //Unitary test to verify that the signer function is working properly
    #[test]
    fn check_signed_tsig() {
        let key = b"1234567890";
        let alg_name = TsigAlgorithm::HmacSha1;
        let fudge = 0;
        let time_signed = 0;
        let id = 6502; 
        let name: String = "".to_string();
        let domain = DomainName::new_from_str("uchile.cl");
        //DNS message
        let mut q = DnsMessage::new_query_message(
            domain.clone(),
            Rrtype::A,
            Rclass::ANY,
            0, 
            false,
            id
        );
        //partial TSIG Resource record verify the signing process
        let tsig_rr = set_tsig_vars(String::from(alg_name.clone()).as_str(), &name, time_signed, fudge);
        let q_for_mac = q.clone();
        //creation of the signature to compare
        let firma_a_comparar = sign_tsig(&mut q, key, alg_name, fudge, time_signed, name, vec![]);
        // creation of the signature digest
        let dig_for_mac = get_digest_request(vec![],q_for_mac.to_bytes(), tsig_rr);
        let mut hasher = crypto_hmac::new(Sha1::new(), key);
        hasher.input(&dig_for_mac[..]);
        
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
}