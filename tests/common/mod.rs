
use if_addrs::{get_if_addrs, Interface};
use std::sync::mpsc;
use dns_rust::{
//     config::RESOLVER_IP_PORT,
//     // config::{CHECK_MASTER_FILES, MASTER_FILES, NAME_SERVER_IP, SBELT_ROOT_IPS},
//     config::{ SBELT_ROOT_IPS},
//     // name_server::{master_file::MasterFile, zone::NSZone},
    resolver::{Resolver},
    message::{DnsMessage,
        rdata::Rdata},
};



pub fn run_resolver_for_testing(resolver_ip_port: &str,sbelt_root_ips: &'static [&'static str]) {
    // Channels
    let (add_sender_udp, add_recv_udp) = mpsc::channel();
    let (delete_sender_udp, delete_recv_udp) = mpsc::channel();
    let (add_sender_tcp, add_recv_tcp) = mpsc::channel();
    let (delete_sender_tcp, delete_recv_tcp) = mpsc::channel();
    // let (add_sender_ns_udp, _) = mpsc::channel();
    // let (delete_sender_ns_udp, _) = mpsc::channel();
    // let (add_sender_ns_tcp, _) = mpsc::channel();
    // let (delete_sender_ns_tcp, _) = mpsc::channel();
    let (update_cache_sender_udp, rx_update_cache_udp) = mpsc::channel();
    let (update_cache_sender_tcp, rx_update_cache_tcp) = mpsc::channel();
    // let (update_cache_sender_ns_udp, _) = mpsc::channel();
    // let (update_cache_sender_ns_tcp, _) = mpsc::channel();

    // let (_, rx_update_zone_udp) = mpsc::channel();
    // let (_, rx_update_zone_tcp) = mpsc::channel();

    
        // Resolver Initialize
        let mut resolver = Resolver::new(
            add_sender_udp.clone(),
            delete_sender_udp.clone(),
            add_sender_tcp.clone(),
            delete_sender_tcp.clone(),
            // add_sender_ns_udp.clone(),
            // delete_sender_ns_udp.clone(),
            // add_sender_ns_tcp.clone(),
            // delete_sender_ns_tcp.clone(),
            update_cache_sender_udp.clone(),
            update_cache_sender_tcp.clone(),
            // update_cache_sender_ns_udp.clone(),
            // update_cache_sender_ns_tcp.clone(),
        );

        resolver.set_initial_configuration(resolver_ip_port, sbelt_root_ips);

        // Run Resolver
        resolver.run_resolver(
            add_recv_udp,
            delete_recv_udp,
            add_recv_tcp,
            delete_recv_tcp,
            rx_update_cache_udp,
            rx_update_cache_tcp,
            // rx_update_zone_udp,
            // rx_update_zone_tcp,
        );
   
}


pub fn qtype_a_example(dns_response: DnsMessage){
    //header values
    let header = dns_response.get_header();
    let answers = dns_response.get_answer();
    let answer_count = header.get_ancount();

    if answer_count > 0 {
        println!("si recibio answer");
        let answer = &answers[0];
        let ip = match answer.get_rdata() {
            Rdata::SomeARdata(val) => val.get_string_address(),
            _ => "".to_string(),
        };

        assert_eq!(ip, "93.184.216.34");
    } else {
        println!("no answers")
    }
}

pub fn qtype_hinfo_example_no_answer(dns_response: DnsMessage){

    //dns message
    let header = dns_response.get_header();
    let question = dns_response.get_question();
    let answers = dns_response.get_answer();
    let authority  = dns_response.get_authority();

    //Header Section
    let qr = header.get_qr();
    let op_code = header.get_op_code();
    let rcode = header.get_rcode();
    let ancount = header.get_ancount();
    let nscount = header.get_nscount();

    assert_eq!(rcode, 0); //FIXME: 2 -> name server failure ??? a veces retorna ????
    assert_eq!(ancount,0);
    assert_eq!(qr,true);
    assert_eq!(op_code,0);
    assert_eq!(nscount,1);

    //Question Section
    let qname = question.get_qname().get_name();
    let qtype = question.get_qtype();
    let qclass = question.get_qclass();

    assert_eq!(qname, "example.com");
    assert_eq!(qclass, 1);
    assert_eq!(qtype, 13);

    //Answer Section
    let answer_len = answers.len();
    assert_eq!(answer_len, 0);
    
    //Authority Section   
    let authority_len = authority.len();
    let soa_name = authority[0].get_name().get_name();
    
    assert_eq!(authority_len, 1);
    assert_eq!(soa_name, "example.com");

    
    // ;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 2953
    // ;; flags: qr rd ra; QUERY: 1, ANSWER: 0, AUTHORITY: 0, ADDITIONAL: 1

    // ;; OPT PSEUDOSECTION:
    // ; EDNS: version: 0, flags:; udp: 512
    // ;; QUESTION SECTION:
    // ;exaple.com.                    IN      HINFO

    // ;; Query time: 71 msec
    // ;; SERVER: 8.8.8.8#53(8.8.8.8)
    // ;; WHEN: vie abr 14 00:46:14 -04 2023
    // ;; MSG SIZE  rcvd: 39

}


pub fn qtype_asterisk_example(dns_response: DnsMessage){
    //TODO:revisar con whireshark

    //dns message
    let header = dns_response.get_header();
    let question = dns_response.get_question();
    let answers = dns_response.get_answer();

    //Header Section
    let qr = header.get_qr();
    let op_code = header.get_op_code();
    // let rcode = header.get_rcode();
    // let ancount = header.get_ancount();
    let nscount = header.get_nscount();

    // assert_eq!(rcode, 0);
    assert_eq!(qr,true);
    assert_eq!(op_code,0);
    assert_eq!(nscount,0);
    // assert_eq!(ancount,2); //FIXME: 0

    //Question Section
    let qname = question.get_qname().get_name();
    let qtype = question.get_qtype();
    let qclass = question.get_qclass();


    assert_eq!(qname, "example.com");
    assert_eq!(qclass, 1);
    assert_eq!(qtype, 255);

    //Answer Section
    let answer_len = answers.len();
    assert_eq!(answer_len, 2);
    

    // ;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 38227
    // ;; flags: qr rd ra ad; QUERY: 1, ANSWER: 2, AUTHORITY: 0, ADDITIONAL: 1

    // ;; OPT PSEUDOSECTION:
    // ; EDNS: version: 0, flags:; udp: 512
    // ;; QUESTION SECTION:
    // ;example.com.			IN	ANY

    // ;; ANSWER SECTION:
    // example.com.		21592	IN	A	93.184.216.34
    // example.com.		21592	IN	RRSIG	A 8 2 86400 20230504141337 20230413073308 8050 example.com. IRArbCOGPrIkRTpfeFZv09u7aUrCHftTXbA78T8qXaqfi6EzSv//jXp9 i4Jjpltqm0/1Zt7FafaMLfbwUYp1qAjbnse4GX1IpWnRo79ajDgO4z6+ JlIIiNKNdVj4xi3wIOR/GbAQJG5VZGkecWng4GH4fEZOOnqO1bjQmE1Q ZYc=

    // ;; Query time: 7 msec
    // ;; SERVER: 8.8.8.8#53(8.8.8.8)
    // ;; WHEN: vie abr 14 02:59:32 -04 2023
    // ;; MSG SIZE  rcvd: 227


}

pub fn qtype_asterisk_test(dns_response: DnsMessage){
    //TODO:revisar con whireshark

    //dns message
    let header = dns_response.get_header();
    let question = dns_response.get_question();
    let answers = dns_response.get_answer();
    let authority  = dns_response.get_authority();

    //Header Section
    let qr = header.get_qr();
    let op_code = header.get_op_code();
    let rcode = header.get_rcode();
    let ancount = header.get_ancount();
    let nscount = header.get_nscount();

    assert_eq!(rcode, 0);
    assert_eq!(ancount,0);
    assert_eq!(qr,true);
    assert_eq!(op_code,0);
    assert_eq!(nscount,0);
    assert_eq!(ancount,0);

    //Question Section
    let qname = question.get_qname().get_name();
    let qtype = question.get_qtype();
    let qclass = question.get_qclass();


    assert_eq!(qname, "test");
    assert_eq!(qclass, 1);
    assert_eq!(qtype, 255);

    //Answer Section
    let answer_len = answers.len();
    assert_eq!(answer_len, 0);
    
    //Authority Section   
    let authority_len = authority.len();
    let soa_name = authority[0].get_name().get_name();
    
    assert_eq!(authority_len, 1);
    assert_eq!(soa_name, ".");

    // ;; ->>HEADER<<- opcode: QUERY, status: NXDOMAIN, id: 11354
    // ;; flags: qr rd ra ad; QUERY: 1, ANSWER: 0, AUTHORITY: 1, ADDITIONAL: 1
    
    // ;; OPT PSEUDOSECTION:
    // ; EDNS: version: 0, flags:; udp: 512
    // ;; QUESTION SECTION:
    // ;test.				IN	ANY
    
    // ;; AUTHORITY SECTION:
    // .			86397	IN	SOA	a.root-servers.net. nstld.verisign-grs.com. 2023041400 1800 900 604800 86400
    
    // ;; Query time: 8 msec
    // ;; SERVER: 8.8.8.8#53(8.8.8.8)
    // ;; WHEN: vie abr 14 03:30:55 -04 2023
    // ;; MSG SIZE  rcvd: 108
}

#[allow(dead_code)]
pub fn qtype_mx_example(dns_response: DnsMessage){
    //Dns Message
    let header = dns_response.get_header();
    let question = dns_response.get_question();

    //Header
    let op_code = header.get_op_code();
    let rd = header.get_rd();
    let ancount = header.get_ancount();
    let nscount = header.get_nscount();

    assert_eq!(rd, false);    
    assert_eq!(op_code, 0);
    assert_eq!(ancount,1);
    assert_eq!(nscount,1);

    //Question Section
    let qname = question.get_qname().get_name();
    let qtype = question.get_qtype();
    let qclass = question.get_qclass();

    assert_eq!(qname, "example.com");
    assert_eq!(qclass, 1);
    assert_eq!(qtype, 15);

    //Authority Section
    let authority_count = header.get_nscount();
    assert_eq!(authority_count, 0);


    // ;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 28287
    // ;; flags: qr rd ra ad; QUERY: 1, ANSWER: 1, AUTHORITY: 0, ADDITIONAL: 1

    // ;; OPT PSEUDOSECTION:
    // ; EDNS: version: 0, flags:; udp: 512
    // ;; QUESTION SECTION:
    // ;example.com.			IN	MX

    // ;; ANSWER SECTION:
    // example.com.		16435	IN	MX	0 .

    // ;; Query time: 7 msec
    // ;; SERVER: 8.8.8.8#53(8.8.8.8)
    // ;; WHEN: vie abr 14 04:00:31 -04 2023
    // ;; MSG SIZE  rcvd: 55


}

pub fn qtype_ns_example(dns_response: DnsMessage){
    //FIXME:falla aveces cuando corro el cliente con el resolver de google

    //Dns Message
    let header = dns_response.get_header();
    let question = dns_response.get_question();

    //Header
    let op_code = header.get_op_code();
    let rd = header.get_rd();
    let ancount = header.get_ancount();
    let nscount = header.get_nscount();
    let rcode = header.get_rcode();

    assert_eq!(rd, false); 
    assert_eq!(rcode,0);   
    assert_eq!(op_code, 0);
    assert_eq!(ancount,2);
    assert_eq!(nscount,0);

    //Question Section
    let qname = question.get_qname().get_name();
    let qtype = question.get_qtype();
    let qclass = question.get_qclass();

    assert_eq!(qname, "example.com");
    assert_eq!(qclass, 1);
    assert_eq!(qtype, 2);

    //Authority Section
    let authority_count = header.get_nscount();
    assert_eq!(authority_count, 0);

    //Answer Section
    let answers = dns_response.get_answer();
    for answer in answers {
        match answer.get_rdata() {
            Rdata::SomeNsRdata(val) => {
                let name = val.get_nsdname().get_name();
                println!("{}", name);
                assert!(name == "a.iana-servers.net" || name == "b.iana-servers.net");
            }
            _ => {
                "".to_string();
            }
        };
    }


    // ;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 41521
    // ;; flags: qr rd ra ad; QUERY: 1, ANSWER: 2, AUTHORITY: 0, ADDITIONAL: 1
    
    // ;; OPT PSEUDOSECTION:
    // ; EDNS: version: 0, flags:; udp: 512
    // ;; QUESTION SECTION:
    // ;example.com.			IN	NS
    
    // ;; ANSWER SECTION:
    // example.com.		14787	IN	NS	a.iana-servers.net.
    // example.com.		14787	IN	NS	b.iana-servers.net.
    
    // ;; Query time: 11 msec
    // ;; SERVER: 8.8.8.8#53(8.8.8.8)
    // ;; WHEN: vie abr 14 04:15:34 -04 2023
    // ;; MSG SIZE  rcvd: 88
    

}

pub fn qtype_cname_example(dns_response: DnsMessage){
    //TODO: No esta hecho 


    // //dns message
    // let header = dns_response.get_header();
    // let question = dns_response.get_question();
    // let answers = dns_response.get_answer();
    // // let authority  = dns_response.get_authority();

    // //Header Section
    // let qr = header.get_qr();
    // let op_code = header.get_op_code();
    // let rcode = header.get_rcode();
    // let ancount = header.get_ancount();
    // let nscount = header.get_nscount();
    // let qdcount = header.get_qdcount();
    // // let arcount = header.get_arcount();

    // //aveces falla con 2
    // assert_eq!(rcode, 0); //FIXME: 2 -> name server failure ??? a veces retorna ????
    // assert_eq!(ancount,2);
    // assert_eq!(qr,true);
    // assert_eq!(op_code,0);
    // assert_eq!(nscount,1);
    // assert_eq!(qdcount,1);
    

    // //Question Section
    // let qname = question.get_qname().get_name();
    // let qtype = question.get_qtype();
    // let qclass = question.get_qclass();

    // assert_eq!(qname, "example.com");
    // assert_eq!(qclass, 1);
    // assert_eq!(qtype, 16);

    // //Answer Section
    // let answer_len = answers.len();
    // assert_eq!(answer_len, 2);


    // if answer_len > 0 {
    //     println!("si recibio answer");
    //     let answer = &answers[0];
    //     let text = match answer.get_rdata() {
    //         Rdata::SomeARdata(val) => val.get_string_address(),
    //         _ => "".to_string(),
    //     };

    //     assert!(text == "" || text== "1");
    // } else {
    //     println!("no answers")
    // }
    

}

pub fn qtype_soa_example(dns_response: DnsMessage){

    //dns message
    let header = dns_response.get_header();
    let question = dns_response.get_question();
    let answers = dns_response.get_answer();
    // let authority  = dns_response.get_authority();

    //Header Section
    let qr = header.get_qr();
    let op_code = header.get_op_code();
    let rcode = header.get_rcode();
    let ancount = header.get_ancount();
    // let nscount = header.get_nscount();
    // let qdcount = header.get_qdcount();
    // let arcount = header.get_arcount();

    //aveces falla con 2
    assert_eq!(rcode, 0); //FIXME: 2 -> name server failure ??? a veces retorna ???? para el client_test
    assert_eq!(ancount,1);
    assert_eq!(qr,true);
    assert_eq!(op_code,0);
    

    //Question Section
    let qname = question.get_qname().get_name();
    let qtype = question.get_qtype();
    let qclass = question.get_qclass();

    assert_eq!(qname, "example.com");
    assert_eq!(qclass, 1);
    assert_eq!(qtype, 6);

    //Answer Section
    let answer_len = answers.len();
    assert_eq!(answer_len, 1);

    //Answer Section
    let answers = dns_response.get_answer();
    
    for answer in answers {
        match answer.get_rdata() {
            Rdata::SomeSoaRdata(val) => {
                let mname  = val.get_mname().get_name();
                let rname  = val.get_rname().get_name();
                let refresh = val.get_refresh();
                let retry = val.get_retry();
                let expire = val.get_expire();
                let minimun = val.get_minimum();
                
                assert_eq!(mname,"ns.icann.org");
                assert_eq!(rname,"noc.dns.icann.org");
                assert_eq!(refresh,7200);
                assert_eq!(retry,3600);   
                assert_eq!(expire,1209600);
                assert_eq!(minimun,3600);
            }
            _ => {
                "".to_string();
            }
        };
    }
    

    // ;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 23223
    // ;; flags: qr rd ra ad; QUERY: 1, ANSWER: 1, AUTHORITY: 0, ADDITIONAL: 1
    
    // ;; OPT PSEUDOSECTION:
    // ; EDNS: version: 0, flags:; udp: 512
    // ;; QUESTION SECTION:
    // ;example.com.			IN	SOA
    
    // ;; ANSWER SECTION:
    // example.com.		3600	IN	SOA	ns.icann.org. noc.dns.icann.org. 2022091258 7200 3600 1209600 3600
    
    // ;; Query time: 152 msec
    // ;; SERVER: 8.8.8.8#53(8.8.8.8)
    // ;; WHEN: dom abr 16 14:11:18 -04 2023
    // ;; MSG SIZE  rcvd: 96
    

}

pub fn qtype_wks_example(dns_response: DnsMessage){
    //TODO: No esta hecho 

    // //dns message
    // let header = dns_response.get_header();
    // let question = dns_response.get_question();
    // let answers = dns_response.get_answer();
    // // let authority  = dns_response.get_authority();

    // //Header Section
    // let qr = header.get_qr();
    // let op_code = header.get_op_code();
    // let rcode = header.get_rcode();
    // let ancount = header.get_ancount();
    // let nscount = header.get_nscount();
    // let qdcount = header.get_qdcount();
    // // let arcount = header.get_arcount();

    
    // assert_eq!(rcode, 0); 
    // assert_eq!(ancount,2);
    // assert_eq!(qr,true);
    // assert_eq!(op_code,0);
    // assert_eq!(nscount,1);
    // assert_eq!(qdcount,1);
    

    // //Question Section
    // let qname = question.get_qname().get_name();
    // let qtype = question.get_qtype();
    // let qclass = question.get_qclass();

    // assert_eq!(qname, "example.com");
    // assert_eq!(qclass, 1);
    // assert_eq!(qtype, 16);

    // //Answer Section
    // let answer_len = answers.len();
    // assert_eq!(answer_len, 2);


}    

 pub fn qtype_ptr_example(dns_response: DnsMessage){

    //TODO: No esta hecho 

    // //dns message
    // let header = dns_response.get_header();
    // let question = dns_response.get_question();
    // let answers = dns_response.get_answer();
    // // let authority  = dns_response.get_authority();

    // //Header Section
    // let qr = header.get_qr();
    // let op_code = header.get_op_code();
    // let rcode = header.get_rcode();
    // let ancount = header.get_ancount();
    // let nscount = header.get_nscount();
    // let qdcount = header.get_qdcount();
    // // let arcount = header.get_arcount();

    // //aveces falla con 2
    // assert_eq!(rcode, 0); 
    // assert_eq!(ancount,2);
    // assert_eq!(qr,true);
    // assert_eq!(op_code,0);
    // assert_eq!(nscount,1);
    // assert_eq!(qdcount,1);
    

    // //Question Section
    // let qname = question.get_qname().get_name();
    // let qtype = question.get_qtype();
    // let qclass = question.get_qclass();

    // assert_eq!(qname, "example.com");
    // assert_eq!(qclass, 1);
    // assert_eq!(qtype, 16);

    // //Answer Section
    // let answer_len = answers.len();
    // assert_eq!(answer_len, 2);


 }  

pub fn qtype_hinfo_example(dns_response: DnsMessage){
    //TODO: No está hecha


    //dns message
    // let header = dns_response.get_header();
    // let question = dns_response.get_question();
    // let answers = dns_response.get_answer();
    // let authority  = dns_response.get_authority();

    //Header Section
    // let qr = header.get_qr();
    // let op_code = header.get_op_code();
    // let rcode = header.get_rcode();
    // let ancount = header.get_ancount();
    // let nscount = header.get_nscount();
    // let qdcount = header.get_qdcount();
    // let arcount = header.get_arcount();

    // //aveces falla con 2
    // assert_eq!(rcode, 0); //FIXME: 2 -> name server failure ??? a veces retorna ????
    // assert_eq!(ancount,2);
    // assert_eq!(qr,true);
    // assert_eq!(op_code,0);
    // assert_eq!(nscount,1);
    // assert_eq!(qdcount,1);
    

    //Question Section
    // let qname = question.get_qname().get_name();
    // let qtype = question.get_qtype();
    // let qclass = question.get_qclass();

    // assert_eq!(qname, "???");
    // assert_eq!(qclass, 1);
    // assert_eq!(qtype, ??);

    //Answer Section
    // let answer_len = answers.len();
    // assert_eq!(answer_len, ??);

    

}  

// pub fn qtype_hinfo_example(dns_response: DnsMessage){
   //FIXME:aveces falla , con el error rcode = 2 server failure ???


    // //dns message
    // let header = dns_response.get_header();
    // let question = dns_response.get_question();
    // let answers = dns_response.get_answer();
    // // let authority  = dns_response.get_authority();

    // //Header Section
    // let qr = header.get_qr();
    // let op_code = header.get_op_code();
    // let rcode = header.get_rcode();
    // let ancount = header.get_ancount();
    // let nscount = header.get_nscount();
    // let qdcount = header.get_qdcount();
    // // let arcount = header.get_arcount();

    // //aveces falla con 2
    // assert_eq!(rcode, 0); //FIXME: 2 -> name server failure ??? a veces retorna ????
    // assert_eq!(ancount,2);
    // assert_eq!(qr,true);
    // assert_eq!(op_code,0);
    // assert_eq!(nscount,1);
    // assert_eq!(qdcount,1);
    

    // //Question Section
    // let qname = question.get_qname().get_name();
    // let qtype = question.get_qtype();
    // let qclass = question.get_qclass();

    // assert_eq!(qname, "example.com");
    // assert_eq!(qclass, 1);
    // assert_eq!(qtype, 16);

    // //Answer Section
    // let answer_len = answers.len();
    // assert_eq!(answer_len, 2);


    // if answer_len > 0 {
    //     println!("si recibio answer");
    //     let answer = &answers[0];
    //     let text = match answer.get_rdata() {
    //         Rdata::SomeARdata(val) => val.get_string_address(),
    //         _ => "".to_string(),
    //     };

    //     assert!(text == "" || text== "1");
    // } else {
    //     println!("no answers")
    // }
    

// }  

pub fn qtype_txt_example(dns_response: DnsMessage){
    //FIXME:aveces falla , con el error rcode = 2 server failure ???


    //dns message
    let header = dns_response.get_header();
    let question = dns_response.get_question();
    let answers = dns_response.get_answer();
    // let authority  = dns_response.get_authority();

    //Header Section
    let qr = header.get_qr();
    let op_code = header.get_op_code();
    let rcode = header.get_rcode();
    let ancount = header.get_ancount();
    let nscount = header.get_nscount();
    let qdcount = header.get_qdcount();
    // let arcount = header.get_arcount();

    //aveces falla con 2
    assert_eq!(rcode, 0); //FIXME: 2 -> name server failure ??? a veces retorna ????
    assert_eq!(ancount,2);
    assert_eq!(qr,true);
    assert_eq!(op_code,0);
    assert_eq!(nscount,1);
    assert_eq!(qdcount,1);
    

    //Question Section
    let qname = question.get_qname().get_name();
    let qtype = question.get_qtype();
    let qclass = question.get_qclass();

    assert_eq!(qname, "example.com");
    assert_eq!(qclass, 1);
    assert_eq!(qtype, 16);

    //Answer Section
    let answer_len = answers.len();
    assert_eq!(answer_len, 2);


    if answer_len > 0 {
        println!("si recibio answer");
        let answer = &answers[0];
        let text = match answer.get_rdata() {
            Rdata::SomeARdata(val) => val.get_string_address(),
            _ => "".to_string(),
        };

        assert!(text == "v=spf1 -all" || text== "wgyf8z8cgvm2qmxpnbnldrcltvk4xqfn");
    } else {
        println!("no answers")
    }
    
    // ;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 54163
    // ;; flags: qr rd ra ad; QUERY: 1, ANSWER: 2, AUTHORITY: 0, ADDITIONAL: 1
    
    // ;; OPT PSEUDOSECTION:
    // ; EDNS: version: 0, flags:; udp: 512
    // ;; QUESTION SECTION:
    // ;example.com.			IN	TXT
    
    // ;; ANSWER SECTION:
    // example.com.		18291	IN	TXT	"v=spf1 -all"
    // example.com.		18291	IN	TXT	"wgyf8z8cgvm2qmxpnbnldrcltvk4xqfn"
    
    // ;; Query time: 7 msec
    // ;; SERVER: 8.8.8.8#53(8.8.8.8)
    // ;; WHEN: dom abr 16 13:38:47 -04 2023
    // ;; MSG SIZE  rcvd: 109
    
}  

pub fn get_interface() -> Result<Interface,&'static str> {

    if let Ok(addrs) = get_if_addrs() {
        let default_interface = addrs
            .iter()
            .find(|&addr| !addr.is_loopback())
            .ok_or("No interface found")?;
        return Ok(default_interface.clone());
    }

    return Err("No interface found");
}
