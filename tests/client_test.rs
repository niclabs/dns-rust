use std::process::Command;

mod common;

use dns_rust::{
    client::{create_client_query,
            send_client_query},
    message::{DnsMessage},
};


//Thist client is tested with the google resolver -> 8.8.8.8:53


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
fn non_existent_type(){

    //values query
    let google_resolver = "8.8.8.8:53"; 
    let transport_protocol = "TCP";
    let domain_name = "example.com";

    // create client query
    let client_query: DnsMessage = create_client_query(domain_name,
                                                           13,
                                                           1);

    // send query and get response
    let dns_response = send_client_query(transport_protocol,
                                        google_resolver,
                                                client_query);

    common::qtype_hinfo_example_no_answer(dns_response);
    
}

#[test]
#[ignore]
fn invalid_domain(){

    //values query
    let domain_name = "exam¿ple.com";
    let google_resolver = "8.8.8.8:53"; 
    let transport_protocol = "TCP";

    // create client query
    let client_query: DnsMessage = create_client_query(domain_name,
                                                           13,
                                                           1);

    // send query and get response
    let dns_response = send_client_query(transport_protocol,
                                        google_resolver,
                                                client_query);

    //Header
    let header = dns_response.get_header();
    let rcode = header.get_rcode(); 
    
    //Format Error
    assert_eq!(rcode, 1);
}

#[test]
#[should_panic]
#[ignore]
fn qtype_asterisk_example(){
    //Not implemented type RRSIG and is in answer 
    //revisar whireshark

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

    common::qtype_asterisk_example(dns_response_example); 
}

#[test]
#[ignore]
fn qtype_asterisk_test(){
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

    common::qtype_asterisk_test(dns_response_test);   
}

#[test]
#[ignore]
fn qtype_mx_example(){
    
    //values query
    let domain_name = "example.com";
    let google_resolver = "8.8.8.8:53"; 
    let transport_protocol = "TCP";

    // create client query
    let client_query: DnsMessage = create_client_query(domain_name,
                                                15,
                                                1);

    // // send query and get response FIXME: se cae aca
    let dns_response_test = send_client_query(transport_protocol,
                                                google_resolver,
                                                client_query);

    common::qtype_mx_example(dns_response_test);   
}

#[test]
fn qtype_ns_example(){
    //falla ves mor medio a veces 
    
    //values query
    let domain_name = "example.com";
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

    common::qtype_ns_example(dns_response_test);   
}


#[test]
#[ignore]
fn nonet_query() {
    //TODO: to run,the terminal must be with super user priviliges
    //to do that ->  sudo -i 

    let interface_name = common::get_interface()
                                        .unwrap().name;


    Command::new("tc")
                            .arg("qdisc")
                            .arg("add")
                            .arg("dev")
                            .arg(interface_name.clone())
                            .arg("root")
                            .arg("netem")
                            .arg("loss")
                            .arg("100%")
                            .spawn().expect("error");

    let show_tc = Command::new("tc")
                            .arg("qdisc")
                            .arg("show")
                            .arg("dev")
                            .arg(interface_name.clone())
                            .spawn().expect("error");


    //tc qdisc add dev wlp0s20f3 root netem loss 100%

    // println!("loss*******{:?}",add_loss);
    println!("trafic status: {:?}",show_tc);


    //test 
    //FIXME: is not working, keeps waiting
    //qtype_a_example("TCP");


    Command::new("tc")
            .arg("qdisc")
            .arg("del")
            .arg("dev")
            .arg(interface_name)
            .arg("root")
            .arg("netem")
            .arg("loss")
            .arg("100%")
            .spawn().expect("error");


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
