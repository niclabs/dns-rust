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


#[allow(dead_code)]
pub fn run_resolver_for_testing(resolver_ip_port: &str,sbelt_root_ips:&'static [&'static str]) {
    // Channels
    let (add_sender_udp, add_recv_udp) = mpsc::channel();
    let (delete_sender_udp, delete_recv_udp) = mpsc::channel();
    let (add_sender_tcp, add_recv_tcp) = mpsc::channel();
    let (delete_sender_tcp, delete_recv_tcp) = mpsc::channel();
    let (update_cache_sender_udp, rx_update_cache_udp) = mpsc::channel();
    let (update_cache_sender_tcp, rx_update_cache_tcp) = mpsc::channel();
    
        // Resolver Initialize
        let mut resolver = Resolver::new(
            add_sender_udp.clone(),
            delete_sender_udp.clone(),
            add_sender_tcp.clone(),
            delete_sender_tcp.clone(),
            update_cache_sender_udp.clone(),
            update_cache_sender_tcp.clone(),
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
        );
   
}

#[allow(dead_code)]
pub fn qtype_cname_bytes(dns_response: Vec<u8>){
    // println!("{:?}",dns_response);

    //HEADER SECTION
    let id = u16::from_be_bytes([dns_response[0], dns_response[1]]); 
    let qdcount = u16::from_be_bytes([dns_response[4], dns_response[5]]);
    let ancount = u16::from_be_bytes([dns_response[6], dns_response[7]]);
    let nscount = u16::from_be_bytes([dns_response[8], dns_response[9]]);
    let arcount = u16::from_be_bytes([dns_response[10], dns_response[11]]);
    
    assert_eq!(id ,3310);
    assert_eq!(qdcount ,1);
    assert_eq!(ancount ,1);
    assert_eq!(nscount ,0);
    assert_eq!(arcount ,1);
    println!("ASSERTS HEADER OK\n");

    //QUESTION SECTION
    let (qname,index_end_qname) = get_domain_name(dns_response.clone(), 12);    
    let type_question = u16::from_be_bytes([dns_response[index_end_qname], dns_response[index_end_qname+1]]);
    let class_question = u16::from_be_bytes([dns_response[index_end_qname+2], dns_response[index_end_qname+3]]);

    assert_eq!(qname,"mail.yahoo.com.");
    assert_eq!(type_question,5);
    assert_eq!(class_question,1);
    println!("ASSERTS QUESTION OK\n");

    //ANSWER SECTION
    let (name,index_end) = get_domain_name(dns_response.clone(), 32); 
    let type_answer = u16::from_be_bytes([dns_response[index_end+1 ], dns_response[index_end+2]]);
    let class_answer = u16::from_be_bytes([dns_response[index_end+3], dns_response[index_end+4]]);
    // let ttl_answer = u32::from_be_bytes([dns_response[index_end+5],dns_response[index_end+6],dns_response[index_end+7],dns_response[index_end+8]]); 
    let rdlength_answer = u16::from_be_bytes([dns_response[index_end+9],dns_response[index_end+10]]); 
    let (cname,_) = get_domain_name(dns_response.clone(), index_end+11);
    

    assert_eq!(name, "mail.yahoo.com.");
    assert_eq!(type_answer,5);
    assert_eq!(class_answer,1);
    assert_eq!(rdlength_answer,27);
    assert_eq!(cname,"edge.gycpi.b.yahoodns.net.");
    
    println!("ASSERTS ANSWER OK\n");


    
}

#[allow(dead_code)]
pub fn qtype_a_example_bytes(dns_response: Vec<u8>){

    //HEADER SECTION
    let id = u16::from_be_bytes([dns_response[0], dns_response[1]]); 
    // let flags = u16::from_be_bytes([dns_response[2], dns_response[3]]); 
    let qdcount = u16::from_be_bytes([dns_response[4], dns_response[5]]);
    let ancount = u16::from_be_bytes([dns_response[6], dns_response[7]]);
    // let nscount = u16::from_be_bytes([dns_response[8], dns_response[9]]);
    // let arcount = u16::from_be_bytes([dns_response[10], dns_response[11]]);
    
    assert_eq!(id ,34321);
    assert_eq!(qdcount ,1);
    assert_eq!(ancount ,1);
    // assert_eq!(nscount ,0);
    // assert_eq!(arcount ,1);
    println!("ASSERTS HEADER OK\n");

    //QUESTION SECTION
    let (qname,_) = get_domain_name(dns_response.clone(), 12);    
    let type_question = u16::from_be_bytes([dns_response[25], dns_response[26]]);
    let class_question = u16::from_be_bytes([dns_response[27], dns_response[28]]);
  
    assert_eq!(qname,"example.com.");
    assert_eq!(type_question,1);
    assert_eq!(class_question,1);
    println!("ASSERTS QUESTION OK\n");

    //ANSWER SECTION
    let (name,index_end_name) = get_domain_name(dns_response.clone(), 29); 
    let type_answer = u16::from_be_bytes([dns_response[index_end_name], dns_response[index_end_name+1]]);
    let class_answer = u16::from_be_bytes([dns_response[index_end_name+2], dns_response[index_end_name+3]]);
    let ttl_answer = u32::from_be_bytes([dns_response[index_end_name+4],dns_response[index_end_name+5],dns_response[index_end_name+6],dns_response[index_end_name+7]]); 
    let rdlength_answer = u16::from_be_bytes([dns_response[index_end_name+8],dns_response[index_end_name+9]]); 
    let rdata_length = (rdlength_answer+10) as usize + index_end_name;
    let rdata_answer =  &dns_response[index_end_name+10..rdata_length]; 

    assert_eq!(name, "example.com.");
    assert_eq!(type_answer,1);
    assert_eq!(class_answer,1);
    assert_eq!(rdlength_answer,4);
    assert_eq!(rdata_answer,vec![93,184,216,34]);
    assert!(ttl_answer <= 1209600);
    println!("ASSERTS ANSWER OK\n");

}

#[allow(dead_code)]
pub fn qtype_txt_example_bytes(dns_response: Vec<u8>){

    //HEADER SECTION
    let id = u16::from_be_bytes([dns_response[0], dns_response[1]]); 
    // let flags = u16::from_be_bytes([dns_response[2], dns_response[3]]); 
    let qdcount = u16::from_be_bytes([dns_response[4], dns_response[5]]);
    let ancount = u16::from_be_bytes([dns_response[6], dns_response[7]]);
    // let nscount = u16::from_be_bytes([dns_response[8], dns_response[9]]);
    // let arcount = u16::from_be_bytes([dns_response[10], dns_response[11]]);
    
    assert_eq!(id ,34321);
    assert_eq!(qdcount ,1);
    assert_eq!(ancount ,2);
    // assert_eq!(nscount ,0);
    // assert_eq!(arcount ,1);
    println!("ASSERTS HEADER OK\n");

    //QUESTION SECTION 
    let (qname ,index_end_qname ) = get_domain_name(dns_response.clone(), 12); 
    let type_question = u16::from_be_bytes([dns_response[index_end_qname],
                                                 dns_response[index_end_qname+1]]);
    let class_question = u16::from_be_bytes([dns_response[index_end_qname+2], 
                                                 dns_response[index_end_qname+3]]);
  
    assert_eq!(qname,"example.com.");
    assert_eq!(type_question,16);
    assert_eq!(class_question,1);
    println!("ASSERTS QUESTION OK\n");

    //ANSWER SECTION
    let mut index_init_answer = 29usize ;
    for i in 0..ancount{ //0  y 1
        
        let (name, index_end_name) = get_domain_name(dns_response.clone(), index_init_answer);

        
        let type_answer = u16::from_be_bytes([dns_response[index_end_name], dns_response[index_end_name+1]]);
        let class_answer = u16::from_be_bytes([dns_response[index_end_name+2], dns_response[index_end_name+3]]);
        let ttl_answer = u32::from_be_bytes([dns_response[index_end_name+4],dns_response[index_end_name+5],
                                                    dns_response[index_end_name+6],dns_response[index_end_name+7]]); 
        let rdlength_answer = u16::from_be_bytes([dns_response[index_end_name+8],dns_response[index_end_name+9]]); 
        let end_index_rdata = index_end_name + 10 +(rdlength_answer as usize);
        let rdata_txt = String::from_utf8_lossy(&dns_response[index_end_name+10..end_index_rdata]); 
        
        index_init_answer += (rdlength_answer+12) as usize ;
        assert_eq!(name,"example.com.");
        assert_eq!(type_answer,16);
        assert_eq!(class_answer,1);
        assert!(ttl_answer <= 1209600);
        
        if i == 0 {
            assert_eq!(rdlength_answer,12);
            assert_eq!(rdata_txt,"\u{b}v=spf1 -all");
            
        }
        else {
            assert_eq!(rdlength_answer,33);
            assert_eq!(rdata_txt," wgyf8z8cgvm2qmxpnbnldrcltvk4xqfn");
        }

    }
}

#[allow(dead_code)]
pub fn qtype_ns_example_bytes(dns_response: Vec<u8>){
    
        //HEADER SECTION
        let id = u16::from_be_bytes([dns_response[0], dns_response[1]]); 
        let qdcount = u16::from_be_bytes([dns_response[4], dns_response[5]]);
        let ancount = u16::from_be_bytes([dns_response[6], dns_response[7]]);
        let nscount = u16::from_be_bytes([dns_response[8], dns_response[9]]);
        // let arcount = u16::from_be_bytes([dns_response[10], dns_response[11]]);
        
        assert_eq!(id ,13839);
        assert_eq!(qdcount ,1);
        assert_eq!(ancount ,2);
        assert_eq!(nscount ,0);
        // assert_eq!(arcount ,1);
        println!("ASSERTS HEADER OK\n");
    
        //QUESTION SECTION 
        let (qname,index_end_qname) = get_domain_name(dns_response.clone(), 12); 
        let type_question = u16::from_be_bytes([dns_response[index_end_qname],
                                                     dns_response[index_end_qname+1]]);
        let class_question = u16::from_be_bytes([dns_response[index_end_qname+2], 
                                                     dns_response[index_end_qname+3]]);
      
        assert_eq!(qname,"example.com.");
        assert_eq!(type_question,2);
        assert_eq!(class_question,1);
        println!("ASSERTS QUESTION OK\n");

        //ANSWER SECTION
        let mut index_init_answer: usize  = 29usize;
        for i in 0..ancount{         
            let (name,index_end_name) = get_domain_name(dns_response.clone(), index_init_answer) ;
            let type_answer = u16::from_be_bytes([dns_response[index_end_name], dns_response[index_end_name+1]]);
            let class_answer = u16::from_be_bytes([dns_response[index_end_name+2], dns_response[index_end_name+3]]);
            let ttl_answer = u32::from_be_bytes([dns_response[index_end_name+4],dns_response[index_end_name+5],
                                                        dns_response[index_end_name+6],dns_response[index_end_name+7]]); 
            let rdlength_answer = u16::from_be_bytes([dns_response[index_end_name+8],dns_response[index_end_name+9]]); 
            let (domain_name, index_end) = get_domain_name(dns_response.clone(), index_end_name+10 );
            index_init_answer = index_end;
 
            assert_eq!(type_answer,2);
            assert_eq!(class_answer,1);
            assert!(ttl_answer <= 1209600);
            assert_eq!(name,"example.com.");
            assert!(rdlength_answer == 20 || rdlength_answer ==4);
            assert!(domain_name=="a.iana-servers.net." || domain_name=="b.iana-servers.net.");

    
    }
}

#[allow(dead_code)]
pub fn qtype_any_example_bytes(dns_response: Vec<u8>){

    //HEADER SECTION
    let id = u16::from_be_bytes([dns_response[0], dns_response[1]]); 
    let qdcount = u16::from_be_bytes([dns_response[4], dns_response[5]]);
    let ancount = u16::from_be_bytes([dns_response[6], dns_response[7]]);
    let nscount = u16::from_be_bytes([dns_response[8], dns_response[9]]);
    let arcount = u16::from_be_bytes([dns_response[10], dns_response[11]]);
    
    assert_eq!(id ,13839); //change
    assert_eq!(qdcount ,1);
    assert_eq!(ancount ,3);
    assert_eq!(nscount ,0);
    assert_eq!(arcount ,1);
    println!("ASSERTS HEADER OK\n");

    //QUESTION SECTION 
    let (qname,index_end_qname) = get_domain_name(dns_response.clone(), 12); 
    let type_question = u16::from_be_bytes([dns_response[index_end_qname],
                                                    dns_response[index_end_qname+1]]);
    let class_question = u16::from_be_bytes([dns_response[index_end_qname+2], 
                                                    dns_response[index_end_qname+3]]);
    
    assert_eq!(qname,"example.com.");
    assert_eq!(type_question,255);
    assert_eq!(class_question,1);
    println!("ASSERTS QUESTION OK\n");

}


#[allow(dead_code)]
pub fn qtype_a_example_bytes_cache(dns_response: Vec<u8>){
    
    //HEADER SECTION
    let id = u16::from_be_bytes([dns_response[0], dns_response[1]]); 
    let flags = u16::from_be_bytes([dns_response[2], dns_response[3]]); 
    //1000000110100000
    let aa:u16 = (flags & 0b0000010000000000) >> 10;
    
    assert_eq!(id ,34321);
    assert_eq!(aa, 1);
    // println!("flags -> {} - {}",flags,aa);
    println!("ASSERTS HEADER OK\n");

}



#[allow(dead_code)]
pub fn qtype_soa_example_bytes(dns_response: Vec<u8>){

    //HEADER SECTION
    let id = u16::from_be_bytes([dns_response[0], dns_response[1]]); 
    // let flags = u16::from_be_bytes([dns_response[2], dns_response[3]]); 
    let qdcount = u16::from_be_bytes([dns_response[4], dns_response[5]]);
    let ancount = u16::from_be_bytes([dns_response[6], dns_response[7]]);
    // let nscount = u16::from_be_bytes([dns_response[8], dns_response[9]]);
    // let arcount = u16::from_be_bytes([dns_response[10], dns_response[11]]);
    
    assert_eq!(id ,34321);
    assert_eq!(qdcount ,1);
    assert_eq!(ancount ,1);
    // assert_eq!(nscount ,0);
    // assert_eq!(arcount ,1);
    println!("ASSERTS HEADER OK\n");

    //QUESTION SECTION 
    let (qname ,index_end_qname) =get_domain_name(dns_response.clone(), 12);  
    let type_question = u16::from_be_bytes([dns_response[index_end_qname],
                                                 dns_response[index_end_qname+1]]);
    let class_question = u16::from_be_bytes([dns_response[index_end_qname+2], 
                                                 dns_response[index_end_qname+3]]);
  
    assert_eq!(qname,"example.com.");
    assert_eq!(type_question,6);
    assert_eq!(class_question,1);
    println!("ASSERTS QUESTION OK\n");


    //ANSWER SECTION
    let (name, index_end_name) = get_domain_name(dns_response.clone(), 29);
    let type_answer = u16::from_be_bytes([dns_response[index_end_name], dns_response[index_end_name+1]]);
    let class_answer = u16::from_be_bytes([dns_response[index_end_name+2], dns_response[index_end_name+3]]);
    let ttl_answer = u32::from_be_bytes([dns_response[index_end_name+4],dns_response[index_end_name+5],dns_response[index_end_name+6],dns_response[index_end_name+7]]); 
    let rdlength_answer = u16::from_be_bytes([dns_response[index_end_name+8],dns_response[index_end_name+9]]); 
    let (mname,end_mname) = get_domain_name(dns_response.clone(),index_end_name+10); 
    let (rname,index_end_rname ) = get_domain_name(dns_response.clone(),end_mname );

    let serial = u32::from_be_bytes([dns_response[index_end_rname +1],dns_response[index_end_rname +2],dns_response[index_end_rname +3],dns_response[index_end_rname +4]]); 
    let refresh = u32::from_be_bytes([dns_response[index_end_rname +5],dns_response[index_end_rname +6],dns_response[index_end_rname +7],dns_response[index_end_rname +8]]); 
    let retry =u32::from_be_bytes([dns_response[index_end_rname +9],dns_response[index_end_rname +10],dns_response[index_end_rname +11],dns_response[index_end_rname +12]]); 
    let expire =u32::from_be_bytes([dns_response[index_end_rname +10],dns_response[index_end_rname +14],dns_response[index_end_rname +15],dns_response[index_end_rname +16]]); 
    let  minimun =  u32::from_be_bytes([dns_response[index_end_rname +5],dns_response[index_end_rname +18],dns_response[index_end_rname +19],dns_response[index_end_rname +20]]); 
    

    assert_eq!(name, "example.com.");
    assert_eq!(type_answer,6);
    assert_eq!(class_answer,1);
    assert!(ttl_answer <= 1209600);
    assert_eq!(rdlength_answer,53);
    assert_eq!(serial,2259293440);
    // assert_eq!(refresh,7200);
    // assert_eq!(retry,3600);
    // assert_eq!(expire,1209600);
    // assert_eq!(minimun,3600);
    assert_eq!(mname,"ns.icann.org.");
    assert_eq!(rname ,"noc.dns.icann.org.");

}

#[allow(dead_code)]
pub fn qtype_ptr_bytes(dns_response: Vec<u8>){

    //HEADER SECTION
    let id = u16::from_be_bytes([dns_response[0], dns_response[1]]); 
    // let flags = u16::from_be_bytes([dns_response[2], dns_response[3]]); 
    let qdcount = u16::from_be_bytes([dns_response[4], dns_response[5]]);
    let ancount = u16::from_be_bytes([dns_response[6], dns_response[7]]);
    let nscount = u16::from_be_bytes([dns_response[8], dns_response[9]]);
    let arcount = u16::from_be_bytes([dns_response[10], dns_response[11]]);
    
    assert_eq!(id ,888);
    assert_eq!(qdcount ,1);
    assert_eq!(ancount ,1);
    assert_eq!(nscount ,0);
    assert_eq!(arcount ,1);
    println!("ASSERTS HEADER OK\n");

    //QUESTION SECTION
    let (qname, index_end_qname) = get_domain_name(dns_response.clone(), 12);   
    let type_question = u16::from_be_bytes([dns_response[index_end_qname],
                                                 dns_response[index_end_qname+1]]);
    let class_question = u16::from_be_bytes([dns_response[index_end_qname+2], 
                                                 dns_response[index_end_qname+3]]);
  
    assert_eq!(qname,"8.8.8.8.in-addr.arpa.");
    assert_eq!(type_question,12);
    assert_eq!(class_question,1);
    println!("ASSERTS QUESTION OK\n");


    //ANSWER SECTION
    let (name,index_end_name) = get_domain_name(dns_response.clone(), index_end_qname+4 );
    
    let type_answer = u16::from_be_bytes([dns_response[index_end_name+1], dns_response[index_end_name+2]]);
    let class_answer = u16::from_be_bytes([dns_response[index_end_name+3], dns_response[index_end_name+4]]);
    let ttl_answer = u32::from_be_bytes([dns_response[index_end_name+5],dns_response[index_end_name+6],dns_response[index_end_name+7],dns_response[index_end_name+8]]); 
    let rdlength_answer = u16::from_be_bytes([dns_response[index_end_name+9],dns_response[index_end_name+10]]); 
    let (ptrdname,_) = get_domain_name(dns_response.clone(),index_end_name+11); 

    assert_eq!(name, "8.8.8.8.in-addr.arpa.");
    assert_eq!(type_answer,12);
    assert_eq!(class_answer,1);
    assert!(ttl_answer <= 1209600);
    assert_eq!(rdlength_answer,12);
    assert_eq!(ptrdname,"dns.google.");


}
#[allow(dead_code)]
pub fn qtype_hinfo_example_bytes(dns_response: Vec<u8>){

    //HEADER SECTION
    let id = u16::from_be_bytes([dns_response[0], dns_response[1]]); 
    // let flags = u16::from_be_bytes([dns_response[2], dns_response[3]]); 
    let qdcount = u16::from_be_bytes([dns_response[4], dns_response[5]]);
    let ancount = u16::from_be_bytes([dns_response[6], dns_response[7]]);
    let nscount = u16::from_be_bytes([dns_response[8], dns_response[9]]);
    let arcount = u16::from_be_bytes([dns_response[10], dns_response[11]]);
    
    assert_eq!(id ,46523);
    assert_eq!(qdcount ,1);
    assert_eq!(ancount ,0);
    assert_eq!(nscount ,1);
    assert_eq!(arcount ,1);
    println!("ASSERTS HEADER OK\n");

    //QUESTION SECTION
    let (qname, index_end_qname) = get_domain_name(dns_response.clone(), 12);   
    let type_question = u16::from_be_bytes([dns_response[index_end_qname],
                                                 dns_response[index_end_qname+1]]);
    let class_question = u16::from_be_bytes([dns_response[index_end_qname+2], 
                                                 dns_response[index_end_qname+3]]);
  
    assert_eq!(qname,"example.com.");
    assert_eq!(type_question,13);
    assert_eq!(class_question,1);
    println!("ASSERTS QUESTION OK\n");


    //AUTHORITY SECTION
    let (name,index_end_name) = get_domain_name(dns_response.clone(), 29 );
    
    let type_answer = u16::from_be_bytes([dns_response[index_end_name+1], dns_response[index_end_name+2]]);
    let class_answer = u16::from_be_bytes([dns_response[index_end_name+3], dns_response[index_end_name+4]]);
    let ttl_answer = u32::from_be_bytes([dns_response[index_end_name+5],dns_response[index_end_name+6],dns_response[index_end_name+7],dns_response[index_end_name+8]]); 
    let rdlength_answer = u16::from_be_bytes([dns_response[index_end_name+9],dns_response[index_end_name+10]]); 
    let (mname,end_mname) = get_domain_name(dns_response.clone(),index_end_name+11); 
    let (rname,index_end_rname ) = get_domain_name(dns_response.clone(),end_mname );

    let serial = u32::from_be_bytes([dns_response[index_end_rname +1],dns_response[index_end_rname +2],dns_response[index_end_rname +3],dns_response[index_end_rname +4]]); 
    let refresh = u32::from_be_bytes([dns_response[index_end_rname +5],dns_response[index_end_rname +6],dns_response[index_end_rname +7],dns_response[index_end_rname +8]]); 
    let retry =u32::from_be_bytes([dns_response[index_end_rname +9],dns_response[index_end_rname +10],dns_response[index_end_rname +11],dns_response[index_end_rname +12]]); 
    let expire =u32::from_be_bytes([dns_response[index_end_rname +10],dns_response[index_end_rname +14],dns_response[index_end_rname +15],dns_response[index_end_rname +16]]); 
    let  minimun =  u32::from_be_bytes([dns_response[index_end_rname +5],dns_response[index_end_rname +18],dns_response[index_end_rname +19],dns_response[index_end_rname +20]]); 
    

    assert_eq!(name, "example.com.");
    assert_eq!(type_answer,6);
    assert_eq!(class_answer,1);
    assert!(ttl_answer <= 1209600);
    assert_eq!(rdlength_answer,44);
    assert_eq!(serial,2022091285);
    assert_eq!(refresh,7200);
    assert_eq!(retry,3600);
    assert_eq!(expire,1209600);
    assert_eq!(minimun,3600);
    assert_eq!(mname,"ns.icann.org.");
    assert_eq!(rname ,"noc.dns.icann.org.");

}


#[allow(dead_code)]
pub fn qtype_mx_example_bytes(dns_response: Vec<u8>){
    // println!("{:?}",dns_response);

    //HEADER SECTION
    let id = u16::from_be_bytes([dns_response[0], dns_response[1]]); 
    // let flags = u16::from_be_bytes([dns_response[2], dns_response[3]]); 
    let qdcount = u16::from_be_bytes([dns_response[4], dns_response[5]]);
    let ancount = u16::from_be_bytes([dns_response[6], dns_response[7]]);
    // let nscount = u16::from_be_bytes([dns_response[8], dns_response[9]]);
    // let arcount = u16::from_be_bytes([dns_response[10], dns_response[11]]);
    
    assert_eq!(id ,65390);
    assert_eq!(qdcount ,1);
    assert_eq!(ancount ,1);
    // assert_eq!(nscount ,0);
    // assert_eq!(arcount ,1);
    println!("ASSERTS HEADER OK\n");

    //QUESTION SECTION
    let (qname, index_end_qname) = get_domain_name(dns_response.clone(), 12);   
    let type_question = u16::from_be_bytes([dns_response[index_end_qname],
                                                 dns_response[index_end_qname+1]]);
    let class_question = u16::from_be_bytes([dns_response[index_end_qname+2], 
                                                 dns_response[index_end_qname+3]]);
  
    assert_eq!(qname,"example.com.");
    assert_eq!(type_question,15);
    assert_eq!(class_question,1);
    println!("ASSERTS QUESTION OK\n");

    //ANSWER SECTION 
    let (name,index_end_name) = get_domain_name(dns_response.clone(), 29 );
    let type_answer = u16::from_be_bytes([dns_response[index_end_name], dns_response[index_end_name+1]]);
    let class_answer = u16::from_be_bytes([dns_response[index_end_name+2], dns_response[index_end_name+3]]);
    let ttl_answer = u32::from_be_bytes([dns_response[index_end_name+4],dns_response[index_end_name+5],dns_response[index_end_name+6],dns_response[index_end_name+7]]); 
    let rdlength_answer = u16::from_be_bytes([dns_response[index_end_name+8],dns_response[index_end_name+9]]); 
    let preference = u16::from_be_bytes([dns_response[index_end_name+10],dns_response[index_end_name+11]]);
    let (exchange,_) = get_domain_name(dns_response.clone(),index_end_name+12);

    assert_eq!(name, "example.com.");
    assert_eq!(type_answer,15);
    assert_eq!(class_answer,1);
    assert!(ttl_answer <= 1209600);
    assert_eq!(rdlength_answer,4);
    assert_eq!(preference,0);
    assert_eq!(exchange,"");

}

#[allow(dead_code)]
pub fn nonexistentdomain_bytes(dns_response: Vec<u8>){

    //HEADER SECTION
    let id = u16::from_be_bytes([dns_response[0], dns_response[1]]); 
    // let flags = u16::from_be_bytes([dns_response[2], dns_response[3]]); 
    let qdcount = u16::from_be_bytes([dns_response[4], dns_response[5]]);
    let ancount = u16::from_be_bytes([dns_response[6], dns_response[7]]);
    // let nscount = u16::from_be_bytes([dns_response[8], dns_response[9]]);
    // let arcount = u16::from_be_bytes([dns_response[10], dns_response[11]]);
    
    assert_eq!(id ,60280);
    assert_eq!(qdcount ,1);
    assert_eq!(ancount ,0);
    // assert_eq!(nscount ,1);
    // assert_eq!(arcount ,1);
    println!("ASSERTS HEADER OK\n");
}

#[allow(dead_code)]
pub fn cache_answer(dns_response: Vec<u8>){
    let flag = u16::from_be_bytes([dns_response[2], dns_response[3]]);
    let flag_binary = format!("{:016b}", flag);
    println!("Flag en binario: {}", flag_binary);
    let mask = 0b0000010000000000;
    let aa = mask & flag;

    assert_eq!(aa , 1);
}

///Returns a full domain and index where it ends
pub fn get_domain_name(dns_response: Vec<u8>, index_init:usize )-> (String, usize){
    let mut current_index: usize = index_init;
    let mut full_domain_name = String::new();

    while dns_response[current_index] != 0{

        let first_bytes_name_section = u8::from_be_bytes([dns_response[current_index]]);
        let mask_is_shortcut:u8 = 0b11000000;
        let is_shortcut = first_bytes_name_section & mask_is_shortcut;
        
        if is_shortcut == 192{
            // 192 = 11000000

            let off_set:usize = u8::from_be_bytes([dns_response[current_index+1]]) as usize;
            let (next_domain ,_) = get_domain_name(dns_response, off_set );
            full_domain_name.push_str(&next_domain);

            break;
    
        }else{

            let len_subdomain: usize = u8::from_be_bytes([dns_response[current_index]]) as usize;
            let subdomain = String::from_utf8_lossy(&dns_response[current_index + 1..current_index + 1 + len_subdomain]).to_string();
            
            full_domain_name.push_str(&subdomain); // concatenar el subdominio actual
            full_domain_name.push_str(".");
            current_index += len_subdomain + 1;
        

        }
    }

    return (full_domain_name, current_index+1);
}


#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn qtype_hinfo_example_no_answer(dns_response: DnsMessage){

    //DNS MESSAGE
    let header = dns_response.get_header();
    let question = dns_response.get_question();
    let answers = dns_response.get_answer();
    let authority  = dns_response.get_authority();

    //HEADER SECTION
    let qr = header.get_qr();
    let op_code = header.get_op_code();
    let rcode = header.get_rcode();
    let ancount = header.get_ancount();
    let nscount = header.get_nscount();

    assert_eq!(rcode, 2); 
    assert_eq!(ancount,0);
    assert_eq!(qr,true);
    assert_eq!(op_code,0);
    assert_eq!(nscount,0);

    //QUESTION SECTION
    let qname = question.get_qname().get_name();
    let qtype = question.get_qtype();
    let qclass = question.get_qclass();

    assert_eq!(qname, "example.com");
    assert_eq!(qclass, 1);
    assert_eq!(qtype, 13);

    //ANSWER SECTION
    let answer_len = answers.len();
    assert_eq!(answer_len, 0);
    
    //AUTHORITY SECTION  
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

#[allow(dead_code)]
pub fn qtype_any_example(dns_response: DnsMessage){

    //DNS MESSAGE
    let header = dns_response.get_header();
    let question = dns_response.get_question();
    let answers = dns_response.get_answer();

    //HEADER SECTION
    let qr = header.get_qr();
    let op_code = header.get_op_code();
    let rcode = header.get_rcode();
    let ancount = header.get_ancount();
    let nscount = header.get_nscount();

    assert_eq!(rcode, 0);
    assert_eq!(qr,true);
    assert_eq!(op_code,0);
    assert_eq!(nscount,0);
    assert_eq!(ancount,2); 

    //QUESTION SECTION
    let qname = question.get_qname().get_name();
    let qtype = question.get_qtype();
    let qclass = question.get_qclass();


    assert_eq!(qname, "example.com");
    assert_eq!(qclass, 1);
    assert_eq!(qtype, 255);

    //ANSWER SECTION
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

#[allow(dead_code)]
pub fn qtype_any_test(dns_response: DnsMessage){

    //DNA MESSAGE
    let header = dns_response.get_header();
    let question = dns_response.get_question();
    let answers = dns_response.get_answer();
    let authority  = dns_response.get_authority();

    //HEADER SECTION
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

    //QUESTION SECTION
    let qname = question.get_qname().get_name();
    let qtype = question.get_qtype();
    let qclass = question.get_qclass();


    assert_eq!(qname, "test");
    assert_eq!(qclass, 1);
    assert_eq!(qtype, 255);

    //ANSWER SECTION
    let answer_len = answers.len();
    assert_eq!(answer_len, 0);
    
    //AUTHORITY SECTION 
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
    
    //DNS MESSAGE
    let header = dns_response.get_header();
    let question = dns_response.get_question();

    //HEADER SECTION
    let op_code = header.get_op_code();
    let rd = header.get_rd();
    let ancount = header.get_ancount();
    // let nscount = header.get_nscount();

    assert_eq!(rd, false);    
    assert_eq!(op_code, 0);
    assert_eq!(ancount,1);
    // assert_eq!(nscount,1);

    //QUESTION SECTION
    let qname = question.get_qname().get_name();
    let qtype = question.get_qtype();
    let qclass = question.get_qclass();

    assert_eq!(qname, "example.com");
    assert_eq!(qclass, 1);
    assert_eq!(qtype, 15);

    //AUTHOORITY SECTION
    // let authority_count = header.get_nscount();
    // assert_eq!(authority_count, 0);


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

#[allow(dead_code)]
pub fn qtype_ns_example(dns_response: DnsMessage){

    //DNS MESSAGE
    let header = dns_response.get_header();
    let question = dns_response.get_question();

    //HEADER SECTION
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

    //QUESTION SECTION
    let qname = question.get_qname().get_name();
    let qtype = question.get_qtype();
    let qclass = question.get_qclass();

    assert_eq!(qname, "example.com");
    assert_eq!(qclass, 1);
    assert_eq!(qtype, 2);

    //AUTHORITY SECTION
    let authority_count = header.get_nscount();
    assert_eq!(authority_count, 0);

    //ANSWERS SECTION
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

#[allow(dead_code)]
pub fn qtype_soa_example(dns_response: DnsMessage){

    //DNS MESSAGE
    let header = dns_response.get_header();
    let question = dns_response.get_question();
    let answers = dns_response.get_answer();
    // let authority  = dns_response.get_authority();

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
    

    //QUESTION SECTION
    let qname = question.get_qname().get_name();
    let qtype = question.get_qtype();
    let qclass = question.get_qclass();

    assert_eq!(qname, "example.com");
    assert_eq!(qclass, 1);
    assert_eq!(qtype, 6);

    //ANSWER SECTION
    let answer_len = answers.len();
    assert_eq!(answer_len, 1);
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

#[allow(dead_code)]
pub fn qtype_hinfo_example(dns_response: DnsMessage){
   //FIXME:aveces falla , con el error rcode = 2 server failure ???


    //DNS MESSAGE
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

        assert!(text == "" || text== "1");
    } else {
        println!("no answers")
    }
}  

#[allow(dead_code)]
pub fn qtype_txt_example(dns_response: DnsMessage){
    //FIXME:aveces falla , con el error rcode = 2 server failure ???


    //DNS MESSAGE
    let header = dns_response.get_header();
    let question = dns_response.get_question();
    let answers = dns_response.get_answer();
    // let authority  = dns_response.get_authority();

    //HEADER SECTION
    let qr = header.get_qr();
    let op_code = header.get_op_code();
    let rcode = header.get_rcode();
    let ancount = header.get_ancount();
    let nscount = header.get_nscount();
    let qdcount = header.get_qdcount();
    // let arcount = header.get_arcount();

    
    assert_eq!(rcode, 0); 
    assert_eq!(ancount,2);
    assert_eq!(qr,true);
    assert_eq!(op_code,0);
    assert_eq!(nscount,1);
    assert_eq!(qdcount,1);
    

    //QUESTION SECTION
    let qname = question.get_qname().get_name();
    let qtype = question.get_qtype();
    let qclass = question.get_qclass();

    assert_eq!(qname, "example.com");
    assert_eq!(qclass, 1);
    assert_eq!(qtype, 16);

    //ANSWER SECTION
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

//#[allow(dead_code)]
// pub fn qtype_hinfo_example(dns_response: DnsMessage){
//     //TODO: No está hecha


//     //DNS MESSAGE
//     let header = dns_response.get_header();
//     let question = dns_response.get_question();
//     let answers = dns_response.get_answer();
//     let authority  = dns_response.get_authority();

//     //Header Section
//     let qr = header.get_qr();
//     let op_code = header.get_op_code();
//     let rcode = header.get_rcode();
//     let ancount = header.get_ancount();
//     let nscount = header.get_nscount();
//     let qdcount = header.get_qdcount();
//     let arcount = header.get_arcount();

//     //aveces falla con 2
//     assert_eq!(rcode, 0); //FIXME: 2 -> name server failure ??? a veces retorna ????
//     assert_eq!(ancount,2);
//     assert_eq!(qr,true);
//     assert_eq!(op_code,0);
//     assert_eq!(nscount,1);
//     assert_eq!(qdcount,1);
    

//     //Question Section
//     let qname = question.get_qname().get_name();
//     let qtype = question.get_qtype();
//     let qclass = question.get_qclass();

//     assert_eq!(qname, "???");
//     assert_eq!(qclass, 1);
//     assert_eq!(qtype, ??);

//     //Answer Section
//     let answer_len = answers.len();
//     assert_eq!(answer_len, ??);
// }  

// #[allow(dead_code)]
//  pub fn qtype_ptr_example(dns_response: DnsMessage){

    //TODO: No esta hecho 

    // //DNS MESSAGE
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
//  }  


// #[allow(dead_code)]
// pub fn qtype_wks_example(dns_response: DnsMessage){
    //TODO: No esta hecho 

    // //DNS MESSAGE
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
// }    


#[allow(dead_code)]
pub fn qtype_cname(dns_response: DnsMessage){
    //FIXME:

    //DNS MESSAGE
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

    
    assert_eq!(rcode, 0); 
    assert_eq!(ancount,2);
    assert_eq!(qr,true);
    assert_eq!(op_code,0);
    assert_eq!(nscount,1);
    assert_eq!(qdcount,1);
    

    //Question Section
    let qname = question.get_qname().get_name();
    let qtype = question.get_qtype();
    let qclass = question.get_qclass();

    assert_eq!(qname, "mail.yahoo.com");
    assert_eq!(qclass, 1);
    assert_eq!(qtype, 5);

    //Answer Section
    let answer_len = answers.len();
    assert_eq!(answer_len, 1);


    if answer_len > 0 {
        println!("si recibio answer");
        let answer = &answers[0];
        let rdata = match answer.get_rdata() {
            Rdata::SomeCnameRdata(val) => val.get_cname().get_name(),
            _ => "".to_string(),
        };

        assert!(rdata == "edge.gycpi.b.yahoodns.net.");
    } else {
        println!("no answers")
    }
}
