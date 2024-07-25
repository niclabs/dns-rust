use std::net::UdpSocket;
use std::time::SystemTime;
use std::vec;
use dns_rust::domain_name::DomainName;
use dns_rust::message::rdata::Rdata;
use dns_rust::message::DnsMessage;
use dns_rust::message::{rrtype::Rrtype, rclass::Rclass};
use dns_rust;
use base64;
use base64::Engine as _;
use dns_rust::tsig::{process_tsig, sign_tsig, TsigAlgorithm};
use std::io::{stdin, stdout, Write};
use std::{thread, time};



pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    let mut input = String::new();

    stdout().flush().expect("Failed to flush stdout!");
    stdin().read_line(&mut input).expect("Failed to read line");

    input.pop();

    return input;
}
const KEY: &[u8; 28] = b"7niAlAtSA70XRNgvlAB5m80ywDA=";
//const KEY: &[u8; 28] = b"8niAlAtSA70XRNgvlAB5m80ywDA=";

fn generate_tsig_a_query(domain :DomainName, id: u16, key_name: String, key: &[u8]) -> (DnsMessage, Vec<u8>) {
    let mut dnsmsg = DnsMessage::new_query_message(domain, Rrtype::A, Rclass::IN, 0, true, id);
    let mut header = dnsmsg.get_header();
    header.set_ad(true);
    dnsmsg.set_header(header);
    let alg_name = TsigAlgorithm::HmacSha1;
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let digest = sign_tsig(&mut dnsmsg, key, alg_name, 300, time, key_name, vec![]);
    return (dnsmsg, digest);
}


fn recv_without_dig(){
    let three_secs = time::Duration::from_secs(4);
    
    let key_bytes = base64::prelude::BASE64_STANDARD.decode(KEY).unwrap();
    let mut lista_alg = vec![];
    lista_alg.push((String::from("hmac-sha1"),true));
    let domain_to_query = DomainName::new_from_str("ns1.nictest");
    let shared_key_name = "weird.nictest".to_string();
    let socket_udp = UdpSocket::bind("192.168.100.2:8890").expect("Failed to bind to address");
    println!("----------------------------------------------------------------");
    input("Generemos un mensaje con TSIG, presione enter para continuar\n");
    let (dns_msg, mac) = generate_tsig_a_query(domain_to_query, 6502, shared_key_name.clone(), &key_bytes);

    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let (val, err) = process_tsig(&dns_msg, &key_bytes, shared_key_name.clone(), time, lista_alg.clone(), vec![]);
    println!("{}", &dns_msg);
    if !val {
        println!("Error en la validacion del mensaje");
        println!("{:?}", err);
        panic!("Error en la validacion del mensaje");
    }
    input("Presione enter para validar la consulta del cliente con tsig");
    println!("Validacion de la peticion OK! tsig_err {:?}", err);
    println!("----------------------------------------------------------------");
    input("Presione enter para enviar el mensaje al servidor");
    println!("Enviando el mensaje al servidor...");
    thread::sleep(three_secs);
    let test_bytes = dns_msg.to_bytes();
    socket_udp.send_to(&test_bytes, "192.168.100.3:53").unwrap();

    let mut buf = [0; 2000];
    let (s, _) = socket_udp.recv_from(& mut buf).unwrap();
    println!("Recibiendo respuesta del servidor\n");
    let bytes = &buf[0..s].to_vec();
    let response = DnsMessage::from_bytes(&bytes).expect("Parseo mal!");
    println!("{}\n", &response);
    input("Presione enter para validar la respuesta del servidor");
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let (val, err) = process_tsig(&response, &key_bytes, shared_key_name.clone(), time, lista_alg, mac);
    
    
    if !val {
        println!("Error en la validacion del mensaje");
        println!("tsig_error_code: {:?}", err);
    }
    println!("----------------------------------------------------------------");
}   

fn recv_dig() {
    let key_bytes = base64::prelude::BASE64_STANDARD.decode(KEY).unwrap();
    let mut lista_alg = vec![];
    lista_alg.push((String::from("hmac-sha1"),true));

    let socket_udp = UdpSocket::bind("127.0.0.1:8887").expect("Failed to bind to address");
    let socket_udp2 = UdpSocket::bind("192.168.100.2:8890").expect("Failed to bind to address");
    let mut buf = [0;1000];
    let (s, addr_in) = socket_udp.recv_from(&mut buf).unwrap();
    //println!("Llego un mensaje de largo {s}");
    let bytes = &buf[0..s].to_vec();
    let dnsmsg = DnsMessage::from_bytes(bytes).expect("Parseo mal!");

    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let mac = vec![];
    let (a, b)= process_tsig(&dnsmsg,  &key_bytes, "weird.nictest".to_string(), time, lista_alg.clone(), mac);

    println!("Verificando la query del cliente!");
    println!("bool: {:?} tsig_err: {:#?}", a, b);
    println!("{:#?}",&dnsmsg);
    println!("-----------------------------------------------------");

    // println!("{:#?}", dnsmsg.get_header());
    let rrs = dnsmsg.get_additional().pop().unwrap();
    let tsig = match rrs.get_rdata() {
        Rdata::TSIG(xd) => {
            xd
        },
        _ => panic!("xd")
    };


    let mac = tsig.get_mac();
    let test_bytes = dnsmsg.to_bytes();

    socket_udp2.send_to(&test_bytes, "192.168.100.3:53").unwrap();

    let mut buf2 = [0; 2000];
    let (s2, _) = socket_udp2.recv_from(& mut buf2).unwrap();
    let bytes2 = &buf2[0..s2].to_vec();
    let dnsmsg2 = DnsMessage::from_bytes(&bytes2[0..s2]).expect("Parseo mal!");

    // let mut response_dns_tsig_file = File::create("response_tsig_cliente.dns").unwrap();
    // response_dns_tsig_file.write_all(bytes2).expect("Error al escribir el archivo");

    let parsed_bytes = dnsmsg2.to_bytes();

    socket_udp.send_to(&parsed_bytes, addr_in).unwrap();
    //process_tsig(&dnsmsg2, key, key_name, time, available_algorithm, mac_to_process)


    //panic!();
    //let bytes = general_purpose::STANDARD.decode(key).unwrap();
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let (a, b)= process_tsig(&dnsmsg2,  &key_bytes, "weird.nictest".to_string(), time, lista_alg, mac);
    println!("Verificando la respuesta del servidor");
    println!("bool: {:?} tsig_err: {:#?}", a, b);
}

fn main() {
    //recv_dig();
    recv_without_dig()
}