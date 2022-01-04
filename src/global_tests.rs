mod global_tests {
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::Rdata;
    use crate::message::DnsMessage;
    use crate::resolver::slist::Slist;
    use crate::resolver::Resolver;

    use rand::thread_rng;
    use rand::Rng;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::io::Read;
    use std::io::Write;
    use std::net::TcpStream;
    use std::net::UdpSocket;
    use std::str::from_utf8;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    use crate::config::CLIENT_IP_PORT;
    use crate::config::IP_PORT;

    //#[test]
    fn udp_response_test() {}

    #[test]
    fn resolver_answer_test() {
        // Resolver settings
        let (add_sender_udp, add_recv_udp) = mpsc::channel();
        let (delete_sender_udp, delete_recv_udp) = mpsc::channel();
        let (add_sender_tcp, add_recv_tcp) = mpsc::channel();
        let (delete_sender_tcp, delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, add_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_udp, delete_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_tcp, delete_recv_ns_tcp) = mpsc::channel();

        let mut resolver = Resolver::new(
            add_sender_udp,
            delete_sender_udp,
            add_sender_tcp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp,
            add_sender_ns_tcp,
            delete_sender_ns_tcp,
        );

        resolver.set_ip_address(IP_PORT.to_string());

        let mut sbelt = Slist::new();
        sbelt.insert(".".to_string(), "8.8.8.8".to_string(), 5.0);

        resolver.set_sbelt(sbelt);
        //

        // Query settings
        let qname = "uchile.cl".to_string();
        let qtype = 1;
        let qclass = 1;
        let op_code = 0;
        let rd = false;

        let mut rng = thread_rng();
        let id: u16 = rng.gen();

        let query_msg =
            DnsMessage::new_query_message(qname, qtype, qclass, op_code, rd, id.clone());

        let client_socket =
            UdpSocket::bind(CLIENT_IP_PORT.to_string()).expect("Failed to bind host socket");
        //

        println!("{}", "Config ready");

        // Runs resolver
        thread::spawn(move || {
            resolver.run_resolver(add_recv_udp, delete_recv_udp, add_recv_tcp, delete_recv_tcp);
        });

        thread::sleep(Duration::new(1, 0));

        // UDP
        client_socket
            .send_to(&query_msg.to_bytes(), IP_PORT.to_string())
            .expect("failed to send message");

        println!("{}", "Query sent");

        // Receive response UDP
        let mut response = [0; 512];

        let (bytes_read, address) = client_socket
            .recv_from(&mut response)
            .expect("No receive msg");

        let dns_response = DnsMessage::from_bytes(&response);
        let answers = dns_response.get_answer();

        assert_eq!(dns_response.get_query_id(), id);
        assert_eq!(answers.len(), 1);

        let answer = answers[0].clone();
        let ip_from_response = match answer.get_rdata() {
            Rdata::SomeARdata(val) => val.get_address(),
            _ => unreachable!(),
        };

        assert_eq!(ip_from_response, [200, 89, 76, 36]);
        //

        // TCP and use of cache
        let mut client_stream = TcpStream::connect(IP_PORT).expect("Failed to connect");
        thread::sleep(Duration::new(1, 0));

        let bytes = query_msg.to_bytes();

        let msg_length: u16 = bytes.len() as u16;

        let tcp_bytes_length = [(msg_length >> 8) as u8, msg_length as u8];

        let full_msg = [&tcp_bytes_length, bytes.as_slice()].concat();

        client_stream.write(&full_msg);

        // Receive response TCP
        let mut response = [0; 512];

        let bytes_read = client_stream.read(&mut response).expect("No receive msg");

        let dns_response = DnsMessage::from_bytes(&response[2..]);
        let answers = dns_response.get_answer();

        assert_eq!(dns_response.get_query_id(), id);
        assert_eq!(answers.len(), 1);

        let answer = answers[0].clone();
        let ip_from_response = match answer.get_rdata() {
            Rdata::SomeARdata(val) => val.get_address(),
            _ => unreachable!(),
        };

        assert_eq!(ip_from_response, [200, 89, 76, 36]);
        //
    }

    #[test]
    fn resolver_delegation_test() {
        unimplemented!();
    }
}
