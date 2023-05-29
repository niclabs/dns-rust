mod common;

use dns_rust::{
    client::{create_client_query,
            send_client_query},
    message::{DnsMessage, },
};

#[test]
#[ignore]
fn udp_query() {
    //FIXME: UDP is not working

    //values query
    let transport_protocol = "UDP";

    //test with google resolver
    qtype_a_example_google_resolver(transport_protocol);
}

#[test]
fn tcp_query() {

    //values query
    let transport_protocol = "TCP";

    //test with google resolver
    qtype_a_example_google_resolver(transport_protocol);
}

#[test]
#[ignore]
fn non_existent_type(){
    //FIXME: it does not work as dig

    //values query
    let google_resolver = "8.8.8.8:53"; 
    let transport_protocol = "TCP";
    let domain_name = "example.com";

    // create client query
    let client_query: DnsMessage = create_client_query(domain_name,13,1);

    // send query and get response TCP and UDP
    // let dns_response_tcp = send_client_query("TCP",google_resolver,client_query.clone());
    let dns_response_udp = send_client_query(transport_protocol,google_resolver,client_query);

    // common::qtype_hinfo_example_no_answer(dns_response_tcp);
    common::qtype_hinfo_example_no_answer(dns_response_udp);
    
}

#[test]
fn invalid_domain(){

    //values query
    let domain_name = "exam¿ple.com";
    let google_resolver = "8.8.8.8:53";

    // create client query
    let client_query: DnsMessage = create_client_query(domain_name,
                                                           13,
                                                           1);

    // send query and get response
    let dns_response_tcp = send_client_query("TCP",google_resolver,client_query.clone());
    let dns_response_udp = send_client_query("UDP",google_resolver,client_query);

    //Header TCP
    let header = dns_response_tcp.get_header();
    let rcode_tcp = header.get_rcode(); 
    assert_eq!(rcode_tcp, 1); //Format Error

    //Header UDP
    let header = dns_response_udp.get_header();
    let rcode_udp = header.get_rcode(); 
    assert_eq!(rcode_udp, 1); //Format Error    
}

#[test]
#[should_panic]
#[ignore]
fn qtype_any_example(){
    //FIXME:

    //values query
    let domain_name_example = "example.com";
    let google_resolver = "8.8.8.8:53"; 
    let transport_protocol = "TCP";

    // create client query
    let client_query_example: DnsMessage = create_client_query(domain_name_example,
                                                           255,
                                                           1);


    // send query and get response
    let dns_response_example = send_client_query(transport_protocol,
                                            google_resolver,
                                                client_query_example);

    common::qtype_any_example(dns_response_example); 
}

#[test]
#[ignore]
fn qtype_any_test(){
    //Not implemented type RRSIG and is in answer

    //values query
    let domain_name_test = "test";
    let google_resolver = "8.8.8.8:53"; 
    let transport_protocol = "TCP";

    // create client query
    let client_query_test: DnsMessage = create_client_query(domain_name_test,
        255,
        1);

    // send query and get response
    let dns_response_test = send_client_query(transport_protocol,
                                                google_resolver,
                                                client_query_test);

    common::qtype_any_test(dns_response_test);   
}

#[test]
fn qtype_mx_example(){
    
    //values query
    let domain_name = "example.com";
    let google_resolver = "8.8.8.8:53"; 

    // create client query
    let client_query: DnsMessage = create_client_query(domain_name,15,1);

    //send query and get response
    let dns_response_tcp = send_client_query("TCP",google_resolver, client_query.clone());
    let dns_response_udp = send_client_query("UDP",google_resolver, client_query);

    common::qtype_mx_example(dns_response_tcp);   
    common::qtype_mx_example(dns_response_udp);   
}

#[test]
fn qtype_ns_example(){
    
    //values query
    let domain_name = "example.com";
    let cloud_fare_resolver = "1.1.1.1:53";

    // create client query
    let client_query: DnsMessage = create_client_query(domain_name,
                                                2,
                                                1);

    //send query and get response 
    let dns_response_tcp = send_client_query("TCP",cloud_fare_resolver,client_query.clone());
    let dns_response_udp = send_client_query("UDP",cloud_fare_resolver,client_query);

    common::qtype_ns_example(dns_response_tcp);  
    common::qtype_ns_example(dns_response_udp); 
}
#[test]
#[ignore]
fn qtype_cname(){
    //FIXME: 
    
    //values query
    let domain_name = "mail.yahoo.com";
    let google_resolver = "8.8.8.8:53";
    let transport_protocol = "TCP";

    // create client query
    let client_query: DnsMessage = create_client_query(domain_name,
                                                2,
                                                1);

    //send query and get response FIXME: se cae aca
    let dns_response_test = send_client_query(transport_protocol,
                                                google_resolver,
                                                client_query);

    common::qtype_cname(dns_response_test);   
}

#[test]
#[ignore]
fn qtype_txt_example(){
    //FIXME: falla rcode = 2 sserver failure
    
    //values query
    let domain_name = "example.com";
    let google_resolver = "8.8.8.8:53"; 
    let transport_protocol = "TCP";

    // create client query
    let client_query: DnsMessage = create_client_query(domain_name,
                                                16,
                                                1);

    //send query and get response FIXME: se cae aca
    let dns_response_test = send_client_query(transport_protocol,
                                                google_resolver,
                                                client_query);
    //TODO: revisar wireshark porque en nuestro se cae el resolver pero no el mensaje 

    common::qtype_txt_example(dns_response_test); 

}

#[test]
#[ignore]
fn qtype_soa_example(){
    //FIXME: falla rcode == 2 server failure, no esta retornando nada????
        
    //values query
    let domain_name = "example.com";
    let google_resolver = "8.8.8.8:53"; 
    let transport_protocol = "TCP";

    // create client query
    let client_query: DnsMessage = create_client_query(domain_name,
                                                6,
                                                1);

    //send query and get response 
    let  dns_response = send_client_query(transport_protocol,
                                                google_resolver,
                                                client_query);

    common::qtype_soa_example(dns_response.clone()); 

}


fn qtype_a_example_google_resolver(transport_protocol:&str) {
    //TODO: put by default UDP

    let google_resolver = "8.8.8.8:53"; 

    // create client query
    let client_query: DnsMessage = create_client_query("example.com",
                                                           1,
                                                           1);

    //send query and get response
    let dns_response = send_client_query(transport_protocol,
                                        google_resolver,
                                                client_query);
    //testing response 
    common::qtype_a_example(dns_response);
}

