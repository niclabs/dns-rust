mod common;

use dns_rust::{
    client::{create_client_query,
            send_client_query},
    message::{DnsMessage,
            rdata::Rdata},
};

//Thist client is tested with the google resolver -> 8.8.8.8:53


#[test]
#[ignore]
fn udp_query() {
    //FIXME: UDP is not working

    //values query
    let transport_protocol = "UDP";

    //test
    qtype_a_example(transport_protocol);
}

#[test]
fn tcp_query() {

    //values query
    let transport_protocol = "TCP";

    //test
    qtype_a_example(transport_protocol);
}

// fn nonet_query()) {
// }

// fn nonet_timeout_query()) {
// }

// fn test_timeout_query_udp() {
// }

// fn test_timeout_query_tcp() {
// }


fn qtype_a_example(transport_protocol:&str) {

    let google_resolver = "8.8.8.8:53"; 

    // create client query
    let client_query: DnsMessage = create_client_query("example.com",
                                                           1,
                                                           1);

    //send query and get response
    let mut dns_response = send_client_query(transport_protocol,
                                        google_resolver,
                                                client_query);

    dns_response.print_dns_message();
    
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