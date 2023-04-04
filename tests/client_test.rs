use std::process::Command;

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

#[test]
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

// fn nonet_timeout_query()) {
// }

// fn test_timeout_query_udp() {
// }

// fn test_timeout_query_tcp() {
// }


fn qtype_a_example(transport_protocol:&str) {
    //TODO: put by default UDP

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