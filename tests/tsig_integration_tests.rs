use std::{collections::HashMap, net:: UdpSocket, thread, time::Duration};
use dns_rust::{domain_name::DomainName, message::{rdata::{tsig_rdata::TSigRdata, Rdata}, rrtype::Rrtype, DnsMessage},tsig::{process_tsig, sign_tsig}};
use dns_rust::tsig::tsig_algorithm::TsigAlgorithm;
use dns_rust::message::rcode::Rcode;
use dns_rust::message::rclass::Rclass;


///RFC 8945 TSIG tests
/*This tests verifies section 5.3:
   When a server has generated a response to a signed request, it signs
   the response using the same algorithm and key.  The server MUST NOT
   generate a signed response to a request if either the key is invalid
   (e.g., key name or algorithm name are unknown) or the MAC fails
   validation; see Section 5.3.2 for details of responding in these
   cases.

   It also MUST NOT generate a signed response to an unsigned request,
   except in the case of a response to a client's unsigned TKEY request
   if the secret key is established on the server side after the server
   processed the client's request.  Signing responses to unsigned TKEY
   requests MUST be explicitly specified in the description of an
   individual secret key establishment algorithm [RFC3645].

   The digest components used to generate a TSIG on a response are:

      Request MAC
      DNS Message (response)
      TSIG Variables (response) 
      
    DISCLAIMER: Como no hay un "NameServer" implementado, se probó la firma y verififcaciónde TSIG utilizando 
    threads y scokets. La idea central es tener un thread recibiendo datos de localhost, el cual tiene en alguna 
    parte guardados pares (key, name) que serían utilizados para la autenticación (en este caso, se guardaron los 
    pares en un hash_map. Como se guarden no es relevante para tsig, depende de la implementación del servidor. 
    Lo único importante es que podamos buscar la llave asociada a un cierto nombre, de acuerdo a la solicitud que 
    recibamos).
    En este test se verifica a nivel macro el correcto flujo  de TSIG: primero se envia a un host en localhost 
    una query firmada, el cual verifica la integridad de la query y responde con un respuesta firmada que será verificada.
*/
#[tokio::test]
async fn tsig_signature() {
    // the key to test tsig flow. The server should had the same key
    let key = b"1234567890";
    // global test variables
    let alg_name = TsigAlgorithm::HmacSha1;
    let fudge = 100;
    let time_signed = 12345678;
    let id = 6502; 
    let name = "nictest.cl";
    let mut dns_query_message =
            DnsMessage::new_query_message(
                DomainName::new_from_string(name.to_string()),
                Rrtype::A,
                Rclass::IN,
                0,
                false,
                id);
    //lista de algoritmos disponibles. En este caso, ambos host tendrán la misma
    let mut a_algs :Vec<(String, bool)>  = vec![];
    a_algs.push((String::from("hmac-sha1"),true));
    a_algs.push((String::from("hmac-sha256"),true));
    
    
    //Código para el servidor. Recibe un mensaje firmado, lo verifica y envía una repuesta autenticada, según lo descrito en el
    //RFC 8945. Este servidor tiene su propia lista de algoritmos disponibles y llaves asociadas a nombres de dominio
    fn host(){
        println!("I am a host");
        //la lista de algoritmos del host
        let mut list :Vec<(String, bool)>  = vec![];
        list.push((String::from("hmac-sha1"),true));
        list.push((String::from("hmac-sha256"),true));

        // se crean las llaves del servidor
        let key1 = b"1234567890";
        let key2 = b"1034567692";
        // se mapean las llaves anteriores a un nombre. Acá deberemos buscar el nombre de lo que se reciba para utilizar la llave correcta
        let mut keys = HashMap::new();
        keys.insert(DomainName::new_from_string("nictest.cl".to_string()), key1);
        keys.insert(DomainName::new_from_string("example.cl".to_string()), key2);

        //se recibiran datos de otro thread a través de un socket UDP
        let udp_socket = UdpSocket::bind("127.0.0.1:8002").expect("Failed to bind to address");
        let mut buf = [0; 512];
        
        match udp_socket.recv_from(&mut buf) {
        
        Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let mut data = DnsMessage::from_bytes(&buf[0..size]).unwrap();
                println!("The data is {:?}", data);
                let mut addit = data.get_additional();
                let rr = addit.pop().expect("No tsigrr");
                let mut tsig_rd = TSigRdata::new();
                // let mut can_sign = false;

                match rr.get_rdata() {
                    Rdata::TSIG(data) =>{
                        tsig_rd = data;
                    }
                    _ => {
                        //can_sign =  true;
                        println!("error: no TSIG rdata found!");
                    }
                }
                //se extraen las variables TSIG necesarias.
                let alg_name = tsig_rd.get_algorithm_name().get_name();
                let time =tsig_rd.get_time_signed();
                let fudge = tsig_rd.get_fudge();
                let mac = tsig_rd.get_mac();
                let name = rr.get_name();
                let key_name = name.clone().get_name();
                // se extrae la llave necesaria
                let key_found = keys[&name];

                //el servidor verifica la estructura del tsig recibido. Sumamos un pequeño delay al time para simular retraso
                let (_,error) = process_tsig(&data, key_found, key_name.clone(), time + 50, list, vec![]); 
                //se setea el aditional sin el ultimo resource record, para que sign_tsig lo regenere
                data.set_additional(addit);
                data.update_header_counters();
                // se firma el mensaje recibido con el digest de la respuesta. Notar que el vector final ahora no está vacío
                sign_tsig(&mut data, key_found,TsigAlgorithm::from(alg_name),fudge,time, key_name, mac);
                let response = &DnsMessage::to_bytes(&data);
                //se verifica que la request haya pasado proces_tsig
                assert_eq!(error,Rcode::NOERROR);
                
                // se envia la respuesta si lo anterior resultó ser correcto
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
                
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                
            }
        }
        
    }

    //Lanzamiento de threads
    println!("Starting server");
    let server_handle = thread::spawn(|| {
        host();  
        
    });
    thread::sleep(Duration::from_secs(2)); 

    // se instancia un socket cliente que enviará y  mensajes
    let client_sock = UdpSocket::bind("127.0.0.1:8001").expect("Nothing");
    // El cliente firma el mensaje para enviar al servidor. Se guarda el mac de la firma
    sign_tsig(&mut dns_query_message, key, alg_name, fudge, time_signed, name.to_string(), vec![]);
    let mac = dns_query_message.get_mac();
    let buf = dns_query_message.to_bytes();
    client_sock.send_to(&buf,"127.0.0.1:8002").unwrap();
    println!("Mensaje enviado");
    server_handle.join().unwrap();
    let mut buf = [0; 512];

    // Ahora el cliente verifica la respuesta recibida del servidor
    match client_sock.recv_from(&mut buf) {
        
        Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let data = DnsMessage::from_bytes(&buf[0..size]).unwrap();
                println!("The data is {:?}", data);

   
                // El cliente procesa la respuesta 
                let (answer, error ) = process_tsig(&data, key, name.to_string(), time_signed, a_algs, mac);
                // se verifica que el mensaje haya pasado process_tsig
                assert!(answer);
                assert_eq!(error,Rcode::NOERROR);
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                
            }
        }


}
   