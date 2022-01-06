mod global_tests {
    use crate::message::rdata::a_rdata::ARdata;
    use crate::message::rdata::Rdata;
    use crate::message::resource_record::ResourceRecord;
    use crate::message::DnsMessage;
    use crate::name_server::NameServer;
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
    use std::sync::mpsc::Receiver;
    use std::sync::mpsc::Sender;
    use std::thread;
    use std::time::Duration;

    use crate::config::CLIENT_IP_PORT;
    use crate::config::NAME_SERVER_IP;
    use crate::config::RESOLVER_IP_PORT;

    /*
    #[test]
    fn resolver_answer_udp_test() {
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

        resolver.set_ip_address(RESOLVER_IP_PORT.to_string());

        let mut sbelt = Slist::new();
        sbelt.insert(".".to_string(), "8.8.8.8".to_string(), 5.0);

        resolver.set_sbelt(sbelt);
        //

        // Query settings
        let qname = "dcc.uchile.cl".to_string();
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
            .send_to(&query_msg.to_bytes(), RESOLVER_IP_PORT.to_string())
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

        assert_eq!(ip_from_response, [192, 80, 24, 11]);
        //
    }
    */

    /*
    #[test]
    fn resolver_answer_tcp_test() {
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

        resolver.set_ip_address(RESOLVER_IP_PORT.to_string());

        let mut sbelt = Slist::new();
        sbelt.insert(".".to_string(), "8.8.8.8".to_string(), 5.0);

        resolver.set_sbelt(sbelt);
        //

        // TCP
        let qname = "dcc.uchile.cl".to_string();
        let qtype = 1;
        let qclass = 1;
        let op_code = 0;
        let rd = false;

        let mut rng = thread_rng();
        let id: u16 = rng.gen();

        let query_msg =
            DnsMessage::new_query_message(qname, qtype, qclass, op_code, rd, id.clone());

        let bytes = query_msg.to_bytes();

        let msg_length: u16 = bytes.len() as u16;

        let tcp_bytes_length = [(msg_length >> 8) as u8, msg_length as u8];

        let full_msg = [&tcp_bytes_length, bytes.as_slice()].concat();

        // Runs resolver
        thread::spawn(move || {
            resolver.run_resolver(add_recv_udp, delete_recv_udp, add_recv_tcp, delete_recv_tcp);
        });

        thread::sleep(Duration::new(1, 0));

        let mut client_stream = TcpStream::connect(RESOLVER_IP_PORT).expect("Failed to connect");

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

        assert_eq!(ip_from_response, [192, 80, 24, 11]);
        //
    }
    */

    /*
    #[test]
    fn resolver_delegation_udp_test() {
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

        resolver.set_ip_address(RESOLVER_IP_PORT.to_string());

        let mut sbelt = Slist::new();
        sbelt.insert(".".to_string(), "192.33.4.12".to_string(), 5.0);

        resolver.set_sbelt(sbelt);
        //

        // Query settings
        let qname = "dcc.uchile.cl".to_string();
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
            .send_to(&query_msg.to_bytes(), RESOLVER_IP_PORT.to_string())
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
        assert_eq!(dns_response.get_header().get_aa(), true);

        let answer = answers[0].clone();
        let ip_from_response = match answer.get_rdata() {
            Rdata::SomeARdata(val) => val.get_address(),
            _ => unreachable!(),
        };

        assert_eq!(ip_from_response, [192, 80, 24, 11]);
        //
    }
    */

    /*
    #[test]
    fn resolver_delegation_tcp_test() {
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

        resolver.set_ip_address(RESOLVER_IP_PORT.to_string());

        let mut sbelt = Slist::new();
        sbelt.insert(".".to_string(), "192.33.4.12".to_string(), 5.0);

        resolver.set_sbelt(sbelt);
        //

        // TCP
        let qname = "dcc.uchile.cl".to_string();
        let qtype = 1;
        let qclass = 1;
        let op_code = 0;
        let rd = false;

        let mut rng = thread_rng();
        let id: u16 = rng.gen();

        let query_msg =
            DnsMessage::new_query_message(qname, qtype, qclass, op_code, rd, id.clone());

        let bytes = query_msg.to_bytes();

        let msg_length: u16 = bytes.len() as u16;

        let tcp_bytes_length = [(msg_length >> 8) as u8, msg_length as u8];

        let full_msg = [&tcp_bytes_length, bytes.as_slice()].concat();

        // Runs resolver
        thread::spawn(move || {
            resolver.run_resolver(add_recv_udp, delete_recv_udp, add_recv_tcp, delete_recv_tcp);
        });

        thread::sleep(Duration::new(1, 0));

        // Send msg TCP
        let mut client_stream = TcpStream::connect(RESOLVER_IP_PORT).expect("Failed to connect");

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

        assert_eq!(ip_from_response, [192, 80, 24, 11]);
        //
    }
    */

    /*
    #[test]
    fn resolver_cname_delegation_udp_test() {
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

        resolver.set_ip_address(RESOLVER_IP_PORT.to_string());

        let mut sbelt = Slist::new();
        sbelt.insert(".".to_string(), "192.33.4.12".to_string(), 5.0);

        resolver.set_sbelt(sbelt);
        //

        // Query settings
        let qname = "mail.google.com".to_string();
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
            .send_to(&query_msg.to_bytes(), RESOLVER_IP_PORT.to_string())
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
        assert_eq!(answers.len(), 4);

        let expected_ips = vec![
            [64, 233, 186, 19],
            [64, 233, 186, 18],
            [64, 233, 186, 17],
            [64, 233, 186, 83],
        ];

        for answer in answers {
            let ip_from_response = match answer.get_rdata() {
                Rdata::SomeARdata(val) => val.get_address(),
                _ => unreachable!(),
            };

            assert!(expected_ips.contains(&ip_from_response));
        }
        //
    }
    */

    /*
    #[test]
    fn resolver_delegation_tcp_test() {
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

        resolver.set_ip_address(RESOLVER_IP_PORT.to_string());

        let mut sbelt = Slist::new();
        sbelt.insert(".".to_string(), "192.33.4.12".to_string(), 5.0);

        resolver.set_sbelt(sbelt);
        //

        // TCP
        let qname = "mail.google.com".to_string();
        let qtype = 1;
        let qclass = 1;
        let op_code = 0;
        let rd = false;

        let mut rng = thread_rng();
        let id: u16 = rng.gen();

        let query_msg =
            DnsMessage::new_query_message(qname, qtype, qclass, op_code, rd, id.clone());

        let bytes = query_msg.to_bytes();

        let msg_length: u16 = bytes.len() as u16;

        let tcp_bytes_length = [(msg_length >> 8) as u8, msg_length as u8];

        let full_msg = [&tcp_bytes_length, bytes.as_slice()].concat();

        // Runs resolver
        thread::spawn(move || {
            resolver.run_resolver(add_recv_udp, delete_recv_udp, add_recv_tcp, delete_recv_tcp);
        });

        thread::sleep(Duration::new(1, 0));

        // Send msg TCP
        let mut client_stream = TcpStream::connect(RESOLVER_IP_PORT).expect("Failed to connect");

        client_stream.write(&full_msg);

        // Receive response TCP
        let mut response = [0; 512];

        let bytes_read = client_stream.read(&mut response).expect("No receive msg");

        let dns_response = DnsMessage::from_bytes(&response[2..]);
        let answers = dns_response.get_answer();

        assert_eq!(dns_response.get_query_id(), id);
        assert_eq!(answers.len(), 4);

        let answer = answers[0].clone();
        let ip_from_response = match answer.get_rdata() {
            Rdata::SomeARdata(val) => val.get_address(),
            _ => unreachable!(),
        };

        let expected_ips = vec![
            [64, 233, 186, 19],
            [64, 233, 186, 18],
            [64, 233, 186, 17],
            [64, 233, 186, 83],
        ];

        for answer in answers {
            let ip_from_response = match answer.get_rdata() {
                Rdata::SomeARdata(val) => val.get_address(),
                _ => unreachable!(),
            };

            assert!(expected_ips.contains(&ip_from_response));
        }
        //
    }
    */

    /*
    #[test]
    fn name_server_zone_answer_udp_test() {
        /// Channels
        let (add_sender_udp, add_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_udp, delete_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_tcp, add_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_tcp, delete_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_udp, add_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_udp, delete_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_tcp, add_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_tcp, delete_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();

        // Name Server initialization
        let mut name_server = NameServer::new(
            true,
            delete_sender_udp.clone(),
            delete_sender_tcp.clone(),
            add_sender_ns_udp.clone(),
            delete_sender_ns_udp.clone(),
            add_sender_ns_tcp.clone(),
            delete_sender_ns_tcp.clone(),
        );

        name_server.add_zone_from_master_file("test.txt".to_string(), "".to_string());

        //

        // Query settings
        let qname = "dcc.uchile.cl".to_string();
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

        // Run NameServer
        let local_resolver_ip = "".to_string();

        thread::spawn(move || {
            name_server.run_name_server(
                NAME_SERVER_IP.to_string(),
                local_resolver_ip,
                add_recv_ns_udp,
                delete_recv_ns_udp,
                add_recv_ns_tcp,
                delete_recv_ns_tcp,
            );
        });

        thread::sleep(Duration::new(1, 0));

        let mut ip_and_port_name_server = NAME_SERVER_IP.to_string();
        ip_and_port_name_server.push_str(":53");

        // UDP
        client_socket
            .send_to(&query_msg.to_bytes(), ip_and_port_name_server)
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
        assert_eq!(dns_response.get_header().get_aa(), true);

        let answer = answers[0].clone();
        let ip_from_response = match answer.get_rdata() {
            Rdata::SomeARdata(val) => val.get_address(),
            _ => unreachable!(),
        };

        assert_eq!(ip_from_response, [192, 80, 24, 11]);
        //
    }
    */

    /*
    #[test]
    fn name_server_zone_answer_tcp_test() {
        /// Channels
        let (add_sender_udp, add_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_udp, delete_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_tcp, add_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_tcp, delete_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_udp, add_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_udp, delete_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_tcp, add_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_tcp, delete_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();

        // Name Server initialization
        let mut name_server = NameServer::new(
            true,
            delete_sender_udp.clone(),
            delete_sender_tcp.clone(),
            add_sender_ns_udp.clone(),
            delete_sender_ns_udp.clone(),
            add_sender_ns_tcp.clone(),
            delete_sender_ns_tcp.clone(),
        );

        name_server.add_zone_from_master_file("test.txt".to_string(), "".to_string());

        //

        // Query settings
        let qname = "dcc.uchile.cl".to_string();
        let qtype = 1;
        let qclass = 1;
        let op_code = 0;
        let rd = false;

        let mut rng = thread_rng();
        let id: u16 = rng.gen();

        let query_msg =
            DnsMessage::new_query_message(qname, qtype, qclass, op_code, rd, id.clone());

        let bytes = query_msg.to_bytes();

        let msg_length: u16 = bytes.len() as u16;

        let tcp_bytes_length = [(msg_length >> 8) as u8, msg_length as u8];

        let full_msg = [&tcp_bytes_length, bytes.as_slice()].concat();

        println!("{}", "Config ready");

        // Run NameServer
        let local_resolver_ip = "".to_string();

        thread::spawn(move || {
            name_server.run_name_server(
                NAME_SERVER_IP.to_string(),
                local_resolver_ip,
                add_recv_ns_udp,
                delete_recv_ns_udp,
                add_recv_ns_tcp,
                delete_recv_ns_tcp,
            );
        });

        thread::sleep(Duration::new(1, 0));

        let mut ip_and_port_name_server = NAME_SERVER_IP.to_string();
        ip_and_port_name_server.push_str(":53");

        // Send msg TCP
        let mut client_stream =
            TcpStream::connect(ip_and_port_name_server).expect("Failed to connect");

        client_stream.write(&full_msg);
        //

        println!("{}", "Query sent");

        // Receive response TCP
        let mut response = [0; 512];

        let bytes_read = client_stream.read(&mut response).expect("No receive msg");

        let dns_response = DnsMessage::from_bytes(&response[2..]);
        let answers = dns_response.get_answer();

        assert_eq!(dns_response.get_query_id(), id);
        assert_eq!(answers.len(), 1);
        assert_eq!(dns_response.get_header().get_aa(), true);

        let answer = answers[0].clone();
        let ip_from_response = match answer.get_rdata() {
            Rdata::SomeARdata(val) => val.get_address(),
            _ => unreachable!(),
        };

        assert_eq!(ip_from_response, [192, 80, 24, 11]);
        //
    }
    */

    /*
    #[test]
    fn name_server_zone_cname_udp_test() {
        /// Channels
        let (add_sender_udp, add_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_udp, delete_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_tcp, add_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_tcp, delete_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_udp, add_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_udp, delete_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_tcp, add_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_tcp, delete_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();

        // Name Server initialization
        let mut name_server = NameServer::new(
            true,
            delete_sender_udp.clone(),
            delete_sender_tcp.clone(),
            add_sender_ns_udp.clone(),
            delete_sender_ns_udp.clone(),
            add_sender_ns_tcp.clone(),
            delete_sender_ns_tcp.clone(),
        );

        name_server.add_zone_from_master_file("test.txt".to_string(), "".to_string());

        //

        // Query settings
        let qname = "test.uchile.cl".to_string();
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

        // Run NameServer
        let local_resolver_ip = "".to_string();

        thread::spawn(move || {
            name_server.run_name_server(
                NAME_SERVER_IP.to_string(),
                local_resolver_ip,
                add_recv_ns_udp,
                delete_recv_ns_udp,
                add_recv_ns_tcp,
                delete_recv_ns_tcp,
            );
        });

        thread::sleep(Duration::new(1, 0));

        let mut ip_and_port_name_server = NAME_SERVER_IP.to_string();
        ip_and_port_name_server.push_str(":53");

        // UDP
        client_socket
            .send_to(&query_msg.to_bytes(), ip_and_port_name_server)
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
        assert_eq!(dns_response.get_header().get_aa(), true);

        let answer = answers[0].clone();
        assert_eq!(answer.get_type_code(), 5);

        let cname = match answer.get_rdata() {
            Rdata::SomeCnameRdata(val) => val.get_cname().get_name(),
            _ => unreachable!(),
        };

        assert_eq!(cname, "test.com".to_string());
        //
    }
    */

    /*
    #[test]
    fn name_server_zone_cname_tcp_test() {
        /// Channels
        let (add_sender_udp, add_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_udp, delete_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_tcp, add_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_tcp, delete_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_udp, add_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_udp, delete_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_tcp, add_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_tcp, delete_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();

        // Name Server initialization
        let mut name_server = NameServer::new(
            true,
            delete_sender_udp.clone(),
            delete_sender_tcp.clone(),
            add_sender_ns_udp.clone(),
            delete_sender_ns_udp.clone(),
            add_sender_ns_tcp.clone(),
            delete_sender_ns_tcp.clone(),
        );

        name_server.add_zone_from_master_file("test.txt".to_string(), "".to_string());

        //

        // Query settings
        let qname = "test.uchile.cl".to_string();
        let qtype = 1;
        let qclass = 1;
        let op_code = 0;
        let rd = false;

        let mut rng = thread_rng();
        let id: u16 = rng.gen();

        let query_msg =
            DnsMessage::new_query_message(qname, qtype, qclass, op_code, rd, id.clone());

        let bytes = query_msg.to_bytes();

        let msg_length: u16 = bytes.len() as u16;

        let tcp_bytes_length = [(msg_length >> 8) as u8, msg_length as u8];

        let full_msg = [&tcp_bytes_length, bytes.as_slice()].concat();

        println!("{}", "Config ready");

        // Run NameServer
        let local_resolver_ip = "".to_string();

        thread::spawn(move || {
            name_server.run_name_server(
                NAME_SERVER_IP.to_string(),
                local_resolver_ip,
                add_recv_ns_udp,
                delete_recv_ns_udp,
                add_recv_ns_tcp,
                delete_recv_ns_tcp,
            );
        });

        thread::sleep(Duration::new(1, 0));

        let mut ip_and_port_name_server = NAME_SERVER_IP.to_string();
        ip_and_port_name_server.push_str(":53");

        // Send msg TCP
        let mut client_stream =
            TcpStream::connect(ip_and_port_name_server).expect("Failed to connect");

        client_stream.write(&full_msg);
        //

        println!("{}", "Query sent");

        // Receive response TCP
        let mut response = [0; 512];

        let bytes_read = client_stream.read(&mut response).expect("No receive msg");

        let dns_response = DnsMessage::from_bytes(&response[2..]);
        let answers = dns_response.get_answer();

        assert_eq!(dns_response.get_query_id(), id);
        assert_eq!(answers.len(), 1);
        assert_eq!(dns_response.get_header().get_aa(), true);

        let answer = answers[0].clone();
        assert_eq!(answer.get_type_code(), 5);

        let cname = match answer.get_rdata() {
            Rdata::SomeCnameRdata(val) => val.get_cname().get_name(),
            _ => unreachable!(),
        };

        assert_eq!(cname, "test.com".to_string());
        //
    }
    */

    /*
    #[test]
    fn name_server_zone_delegation_udp_test() {
        /// Channels
        let (add_sender_udp, add_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_udp, delete_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_tcp, add_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_tcp, delete_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_udp, add_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_udp, delete_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_tcp, add_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_tcp, delete_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();

        // Name Server initialization
        let mut name_server = NameServer::new(
            true,
            delete_sender_udp.clone(),
            delete_sender_tcp.clone(),
            add_sender_ns_udp.clone(),
            delete_sender_ns_udp.clone(),
            add_sender_ns_tcp.clone(),
            delete_sender_ns_tcp.clone(),
        );

        name_server.add_zone_from_master_file("test.txt".to_string(), "".to_string());

        //

        // Query settings
        let qname = "delegation.uchile.cl".to_string();
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

        // Run NameServer
        let local_resolver_ip = "".to_string();

        thread::spawn(move || {
            name_server.run_name_server(
                NAME_SERVER_IP.to_string(),
                local_resolver_ip,
                add_recv_ns_udp,
                delete_recv_ns_udp,
                add_recv_ns_tcp,
                delete_recv_ns_tcp,
            );
        });

        thread::sleep(Duration::new(1, 0));

        let mut ip_and_port_name_server = NAME_SERVER_IP.to_string();
        ip_and_port_name_server.push_str(":53");

        // UDP
        client_socket
            .send_to(&query_msg.to_bytes(), ip_and_port_name_server)
            .expect("failed to send message");

        println!("{}", "Query sent");

        // Receive response UDP
        let mut response = [0; 512];

        let (bytes_read, address) = client_socket
            .recv_from(&mut response)
            .expect("No receive msg");

        let dns_response = DnsMessage::from_bytes(&response);
        let answers = dns_response.get_answer();
        let authority = dns_response.get_authority();
        let additional = dns_response.get_additional();

        assert_eq!(dns_response.get_query_id(), id);
        assert_eq!(answers.len(), 0);
        assert_eq!(authority.len(), 2);
        assert_eq!(additional.len(), 1);
        assert_eq!(dns_response.get_header().get_aa(), false);

        let expected_names = ["ns2.test.com", "ns.delegation.uchile.cl"];

        for auth in authority {
            let name = match auth.get_rdata() {
                Rdata::SomeNsRdata(val) => val.get_nsdname().get_name(),
                _ => unreachable!(),
            };

            assert_eq!(auth.get_type_code(), 2);
            assert!(expected_names.contains(&name.as_str()));
        }

        let add_rr = additional[0].clone();

        let ip_address = match add_rr.get_rdata() {
            Rdata::SomeARdata(val) => val.get_address(),
            _ => unreachable!(),
        };

        assert_eq!(ip_address, [127, 0, 0, 1]);
        //
    }
    */

    /*
    #[test]
    fn name_server_zone_delegation_tcp_test() {
        /// Channels
        let (add_sender_udp, add_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_udp, delete_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_tcp, add_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_tcp, delete_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_udp, add_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_udp, delete_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_tcp, add_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_tcp, delete_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();

        // Name Server initialization
        let mut name_server = NameServer::new(
            true,
            delete_sender_udp.clone(),
            delete_sender_tcp.clone(),
            add_sender_ns_udp.clone(),
            delete_sender_ns_udp.clone(),
            add_sender_ns_tcp.clone(),
            delete_sender_ns_tcp.clone(),
        );

        name_server.add_zone_from_master_file("test.txt".to_string(), "".to_string());

        //

        // Query settings
        let qname = "delegation.uchile.cl".to_string();
        let qtype = 1;
        let qclass = 1;
        let op_code = 0;
        let rd = false;

        let mut rng = thread_rng();
        let id: u16 = rng.gen();

        let query_msg =
            DnsMessage::new_query_message(qname, qtype, qclass, op_code, rd, id.clone());

        let bytes = query_msg.to_bytes();

        let msg_length: u16 = bytes.len() as u16;

        let tcp_bytes_length = [(msg_length >> 8) as u8, msg_length as u8];

        let full_msg = [&tcp_bytes_length, bytes.as_slice()].concat();

        println!("{}", "Config ready");

        // Run NameServer
        let local_resolver_ip = "".to_string();

        thread::spawn(move || {
            name_server.run_name_server(
                NAME_SERVER_IP.to_string(),
                local_resolver_ip,
                add_recv_ns_udp,
                delete_recv_ns_udp,
                add_recv_ns_tcp,
                delete_recv_ns_tcp,
            );
        });

        thread::sleep(Duration::new(1, 0));

        let mut ip_and_port_name_server = NAME_SERVER_IP.to_string();
        ip_and_port_name_server.push_str(":53");

        // Send msg TCP
        let mut client_stream =
            TcpStream::connect(ip_and_port_name_server).expect("Failed to connect");

        client_stream.write(&full_msg);
        //

        println!("{}", "Query sent");

        // Receive response TCP
        let mut response = [0; 512];

        let bytes_read = client_stream.read(&mut response).expect("No receive msg");

        let dns_response = DnsMessage::from_bytes(&response[2..]);
        let answers = dns_response.get_answer();
        let authority = dns_response.get_authority();
        let additional = dns_response.get_additional();

        assert_eq!(dns_response.get_query_id(), id);
        assert_eq!(answers.len(), 0);
        assert_eq!(authority.len(), 2);
        assert_eq!(additional.len(), 1);
        assert_eq!(dns_response.get_header().get_aa(), false);

        let expected_names = ["ns2.test.com", "ns.delegation.uchile.cl"];

        for auth in authority {
            let name = match auth.get_rdata() {
                Rdata::SomeNsRdata(val) => val.get_nsdname().get_name(),
                _ => unreachable!(),
            };

            assert_eq!(auth.get_type_code(), 2);
            assert!(expected_names.contains(&name.as_str()));
        }

        let add_rr = additional[0].clone();

        let ip_address = match add_rr.get_rdata() {
            Rdata::SomeARdata(val) => val.get_address(),
            _ => unreachable!(),
        };

        assert_eq!(ip_address, [127, 0, 0, 1]);
        //
    }
    */

    /*
    #[test]
    fn name_server_authority_name_error_udp_test() {
        /// Channels
        let (add_sender_udp, add_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_udp, delete_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_tcp, add_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_tcp, delete_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_udp, add_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_udp, delete_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_tcp, add_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_tcp, delete_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();

        // Name Server initialization
        let mut name_server = NameServer::new(
            true,
            delete_sender_udp.clone(),
            delete_sender_tcp.clone(),
            add_sender_ns_udp.clone(),
            delete_sender_ns_udp.clone(),
            add_sender_ns_tcp.clone(),
            delete_sender_ns_tcp.clone(),
        );

        name_server.add_zone_from_master_file("test.txt".to_string(), "".to_string());

        //

        // Query settings
        let qname = "auth.uchile.cl".to_string();
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

        // Run NameServer
        let local_resolver_ip = "".to_string();

        thread::spawn(move || {
            name_server.run_name_server(
                NAME_SERVER_IP.to_string(),
                local_resolver_ip,
                add_recv_ns_udp,
                delete_recv_ns_udp,
                add_recv_ns_tcp,
                delete_recv_ns_tcp,
            );
        });

        thread::sleep(Duration::new(1, 0));

        let mut ip_and_port_name_server = NAME_SERVER_IP.to_string();
        ip_and_port_name_server.push_str(":53");

        // UDP
        client_socket
            .send_to(&query_msg.to_bytes(), ip_and_port_name_server)
            .expect("failed to send message");

        println!("{}", "Query sent");

        // Receive response UDP
        let mut response = [0; 512];

        let (bytes_read, address) = client_socket
            .recv_from(&mut response)
            .expect("No receive msg");

        let dns_response = DnsMessage::from_bytes(&response);
        let answers = dns_response.get_answer();
        let authority = dns_response.get_authority();
        let additional = dns_response.get_additional();

        assert_eq!(dns_response.get_query_id(), id);
        assert_eq!(answers.len(), 0);
        assert_eq!(authority.len(), 0);
        assert_eq!(additional.len(), 0);
        assert_eq!(dns_response.get_header().get_aa(), true);
        assert_eq!(dns_response.get_header().get_rcode(), 3);
        //
    }
    */

    /*
    #[test]
    fn name_server_authority_name_error_tcp_test() {
        /// Channels
        let (add_sender_udp, add_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_udp, delete_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_tcp, add_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_tcp, delete_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_udp, add_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_udp, delete_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_tcp, add_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_tcp, delete_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();

        // Name Server initialization
        let mut name_server = NameServer::new(
            true,
            delete_sender_udp.clone(),
            delete_sender_tcp.clone(),
            add_sender_ns_udp.clone(),
            delete_sender_ns_udp.clone(),
            add_sender_ns_tcp.clone(),
            delete_sender_ns_tcp.clone(),
        );

        name_server.add_zone_from_master_file("test.txt".to_string(), "".to_string());

        //

        // Query settings
        let qname = "auth.uchile.cl".to_string();
        let qtype = 1;
        let qclass = 1;
        let op_code = 0;
        let rd = false;

        let mut rng = thread_rng();
        let id: u16 = rng.gen();

        let query_msg =
            DnsMessage::new_query_message(qname, qtype, qclass, op_code, rd, id.clone());

        let bytes = query_msg.to_bytes();

        let msg_length: u16 = bytes.len() as u16;

        let tcp_bytes_length = [(msg_length >> 8) as u8, msg_length as u8];

        let full_msg = [&tcp_bytes_length, bytes.as_slice()].concat();

        println!("{}", "Config ready");

        // Run NameServer
        let local_resolver_ip = "".to_string();

        thread::spawn(move || {
            name_server.run_name_server(
                NAME_SERVER_IP.to_string(),
                local_resolver_ip,
                add_recv_ns_udp,
                delete_recv_ns_udp,
                add_recv_ns_tcp,
                delete_recv_ns_tcp,
            );
        });

        thread::sleep(Duration::new(1, 0));

        let mut ip_and_port_name_server = NAME_SERVER_IP.to_string();
        ip_and_port_name_server.push_str(":53");

        // Send msg TCP
        let mut client_stream =
            TcpStream::connect(ip_and_port_name_server).expect("Failed to connect");

        client_stream.write(&full_msg);
        //

        println!("{}", "Query sent");

        // Receive response TCP
        let mut response = [0; 512];

        let bytes_read = client_stream.read(&mut response).expect("No receive msg");
        //

        let dns_response = DnsMessage::from_bytes(&response[2..]);
        let answers = dns_response.get_answer();
        let authority = dns_response.get_authority();
        let additional = dns_response.get_additional();

        assert_eq!(dns_response.get_query_id(), id);
        assert_eq!(answers.len(), 0);
        assert_eq!(authority.len(), 0);
        assert_eq!(additional.len(), 0);
        assert_eq!(dns_response.get_header().get_aa(), true);
        assert_eq!(dns_response.get_header().get_rcode(), 3);
    }
    */

    /*
    #[test]
    fn name_server_asterisk_case_udp_test() {
        /// Channels
        let (add_sender_udp, add_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_udp, delete_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_tcp, add_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_tcp, delete_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_udp, add_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_udp, delete_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_tcp, add_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_tcp, delete_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();

        // Name Server initialization
        let mut name_server = NameServer::new(
            true,
            delete_sender_udp.clone(),
            delete_sender_tcp.clone(),
            add_sender_ns_udp.clone(),
            delete_sender_ns_udp.clone(),
            add_sender_ns_tcp.clone(),
            delete_sender_ns_tcp.clone(),
        );

        name_server.add_zone_from_master_file("test.txt".to_string(), "".to_string());

        //

        // Query settings
        let qname = "delegation.dcc.uchile.cl".to_string();
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

        // Run NameServer
        let local_resolver_ip = "".to_string();

        thread::spawn(move || {
            name_server.run_name_server(
                NAME_SERVER_IP.to_string(),
                local_resolver_ip,
                add_recv_ns_udp,
                delete_recv_ns_udp,
                add_recv_ns_tcp,
                delete_recv_ns_tcp,
            );
        });

        thread::sleep(Duration::new(1, 0));

        let mut ip_and_port_name_server = NAME_SERVER_IP.to_string();
        ip_and_port_name_server.push_str(":53");

        // UDP
        client_socket
            .send_to(&query_msg.to_bytes(), ip_and_port_name_server)
            .expect("failed to send message");

        println!("{}", "Query sent");

        // Receive response UDP
        let mut response = [0; 512];

        let (bytes_read, address) = client_socket
            .recv_from(&mut response)
            .expect("No receive msg");

        let dns_response = DnsMessage::from_bytes(&response);
        let answers = dns_response.get_answer();
        let authority = dns_response.get_authority();
        let additional = dns_response.get_additional();

        assert_eq!(dns_response.get_query_id(), id);
        assert_eq!(answers.len(), 1);
        assert_eq!(authority.len(), 0);
        assert_eq!(additional.len(), 0);
        assert_eq!(dns_response.get_header().get_aa(), true);

        let answer = answers[0].clone();

        let ip_address = match answer.get_rdata() {
            Rdata::SomeARdata(val) => val.get_address(),
            _ => unreachable!(),
        };

        assert_eq!(ip_address, [192, 80, 24, 10]);
        //
    }
    */

    /*
    #[test]
    fn name_server_asterisk_case_tcp_test() {
        /// Channels
        let (add_sender_udp, add_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_udp, delete_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_tcp, add_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_tcp, delete_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_udp, add_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_udp, delete_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_tcp, add_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_tcp, delete_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();

        // Name Server initialization
        let mut name_server = NameServer::new(
            true,
            delete_sender_udp.clone(),
            delete_sender_tcp.clone(),
            add_sender_ns_udp.clone(),
            delete_sender_ns_udp.clone(),
            add_sender_ns_tcp.clone(),
            delete_sender_ns_tcp.clone(),
        );

        name_server.add_zone_from_master_file("test.txt".to_string(), "".to_string());

        //

        // Query settings
        let qname = "delegation.dcc.uchile.cl".to_string();
        let qtype = 1;
        let qclass = 1;
        let op_code = 0;
        let rd = false;

        let mut rng = thread_rng();
        let id: u16 = rng.gen();

        let query_msg =
            DnsMessage::new_query_message(qname, qtype, qclass, op_code, rd, id.clone());

        let bytes = query_msg.to_bytes();

        let msg_length: u16 = bytes.len() as u16;

        let tcp_bytes_length = [(msg_length >> 8) as u8, msg_length as u8];

        let full_msg = [&tcp_bytes_length, bytes.as_slice()].concat();

        println!("{}", "Config ready");

        // Run NameServer
        let local_resolver_ip = "".to_string();

        thread::spawn(move || {
            name_server.run_name_server(
                NAME_SERVER_IP.to_string(),
                local_resolver_ip,
                add_recv_ns_udp,
                delete_recv_ns_udp,
                add_recv_ns_tcp,
                delete_recv_ns_tcp,
            );
        });

        thread::sleep(Duration::new(1, 0));

        let mut ip_and_port_name_server = NAME_SERVER_IP.to_string();
        ip_and_port_name_server.push_str(":53");

        // Send msg TCP
        let mut client_stream =
            TcpStream::connect(ip_and_port_name_server).expect("Failed to connect");

        client_stream.write(&full_msg);
        //

        println!("{}", "Query sent");

        // Receive response TCP
        let mut response = [0; 512];

        let bytes_read = client_stream.read(&mut response).expect("No receive msg");

        let dns_response = DnsMessage::from_bytes(&response[2..]);
        let answers = dns_response.get_answer();
        let authority = dns_response.get_authority();
        let additional = dns_response.get_additional();

        assert_eq!(dns_response.get_query_id(), id);
        assert_eq!(answers.len(), 1);
        assert_eq!(authority.len(), 0);
        assert_eq!(additional.len(), 0);
        assert_eq!(dns_response.get_header().get_aa(), true);

        let answer = answers[0].clone();

        let ip_address = match answer.get_rdata() {
            Rdata::SomeARdata(val) => val.get_address(),
            _ => unreachable!(),
        };

        assert_eq!(ip_address, [192, 80, 24, 10]);
        //
    }
    */

    /*
    #[test]
    fn name_server_recursive_case_udp_test() {
        /// Channels
        let (add_sender_udp, add_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_udp, delete_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_tcp, add_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_tcp, delete_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_udp, add_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_udp, delete_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_tcp, add_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_tcp, delete_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();

        // Name Server initialization
        let mut name_server = NameServer::new(
            true,
            delete_sender_udp.clone(),
            delete_sender_tcp.clone(),
            add_sender_ns_udp.clone(),
            delete_sender_ns_udp.clone(),
            add_sender_ns_tcp.clone(),
            delete_sender_ns_tcp.clone(),
        );

        name_server.add_zone_from_master_file("test.txt".to_string(), "".to_string());

        //

        // Resolver Initialization

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

        resolver.set_ip_address(RESOLVER_IP_PORT.to_string());

        let mut sbelt = Slist::new();
        sbelt.insert(".".to_string(), "192.33.4.12".to_string(), 5.0);

        resolver.set_sbelt(sbelt);
        resolver.set_ns_data(name_server.get_zones());
        //

        // Query settings
        let qname = "google.com".to_string();
        let qtype = 1;
        let qclass = 1;
        let op_code = 0;
        let rd = true;

        let mut rng = thread_rng();
        let id: u16 = rng.gen();

        let query_msg =
            DnsMessage::new_query_message(qname, qtype, qclass, op_code, rd, id.clone());

        let client_socket =
            UdpSocket::bind(CLIENT_IP_PORT.to_string()).expect("Failed to bind host socket");
        //

        println!("{}", "Config ready");

        // Run NameServer
        let local_resolver_ip = RESOLVER_IP_PORT.to_string();

        thread::spawn(move || {
            name_server.run_name_server(
                NAME_SERVER_IP.to_string(),
                local_resolver_ip,
                add_recv_ns_udp,
                delete_recv_ns_udp,
                add_recv_ns_tcp,
                delete_recv_ns_tcp,
            );
        });

        thread::spawn(move || {
            resolver.run_resolver(add_recv_udp, delete_recv_udp, add_recv_tcp, delete_recv_tcp);
        });

        thread::sleep(Duration::new(1, 0));

        let mut ip_and_port_name_server = NAME_SERVER_IP.to_string();
        ip_and_port_name_server.push_str(":53");

        // UDP
        client_socket
            .send_to(&query_msg.to_bytes(), ip_and_port_name_server)
            .expect("failed to send message");

        println!("{}", "Query sent");

        println!("Client socket: {:#?}", client_socket.local_addr());

        // Receive response UDP
        let mut response = [0; 512];

        let (bytes_read, address) = client_socket
            .recv_from(&mut response)
            .expect("No receive msg");

        let dns_response = DnsMessage::from_bytes(&response);
        let answers = dns_response.get_answer();
        let authority = dns_response.get_authority();
        let additional = dns_response.get_additional();

        assert_eq!(dns_response.get_query_id(), id);
        assert_eq!(answers.len(), 6);
        assert_eq!(authority.len(), 0);
        assert_eq!(additional.len(), 0);
        assert_eq!(dns_response.get_header().get_aa(), true);

        let expected_ips = [
            [64, 233, 186, 100],
            [64, 233, 186, 101],
            [64, 233, 186, 102],
            [64, 233, 186, 113],
            [64, 233, 186, 138],
            [64, 233, 186, 139],
        ];

        for answer in answers {
            let ip_address = match answer.get_rdata() {
                Rdata::SomeARdata(val) => val.get_address(),
                _ => unreachable!(),
            };

            println!("{:#?}", ip_address.clone());

            assert!(expected_ips.contains(&ip_address));
            //
        }
    }
    */

    /*
    #[test]
    fn name_server_recursive_case_tcp_test() {
        /// Channels
        let (add_sender_udp, add_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_udp, delete_recv_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_tcp, add_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_tcp, delete_recv_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_udp, add_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_udp, delete_recv_ns_udp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (add_sender_ns_tcp, add_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();
        let (delete_sender_ns_tcp, delete_recv_ns_tcp): (
            Sender<(String, ResourceRecord)>,
            Receiver<(String, ResourceRecord)>,
        ) = mpsc::channel();

        // Name Server initialization
        let mut name_server = NameServer::new(
            true,
            delete_sender_udp.clone(),
            delete_sender_tcp.clone(),
            add_sender_ns_udp.clone(),
            delete_sender_ns_udp.clone(),
            add_sender_ns_tcp.clone(),
            delete_sender_ns_tcp.clone(),
        );

        name_server.add_zone_from_master_file("test.txt".to_string(), "".to_string());

        //

        // Resolver Initialization

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

        resolver.set_ip_address(RESOLVER_IP_PORT.to_string());

        let mut sbelt = Slist::new();
        sbelt.insert(".".to_string(), "192.33.4.12".to_string(), 5.0);

        resolver.set_sbelt(sbelt);
        resolver.set_ns_data(name_server.get_zones());
        //

        // Query settings
        let qname = "google.com".to_string();
        let qtype = 1;
        let qclass = 1;
        let op_code = 0;
        let rd = true;

        let mut rng = thread_rng();
        let id: u16 = rng.gen();

        let query_msg =
            DnsMessage::new_query_message(qname, qtype, qclass, op_code, rd, id.clone());

        let bytes = query_msg.to_bytes();

        let msg_length: u16 = bytes.len() as u16;

        let tcp_bytes_length = [(msg_length >> 8) as u8, msg_length as u8];

        let full_msg = [&tcp_bytes_length, bytes.as_slice()].concat();

        println!("{}", "Config ready");

        // Run NameServer
        let local_resolver_ip = RESOLVER_IP_PORT.to_string();

        thread::spawn(move || {
            name_server.run_name_server(
                NAME_SERVER_IP.to_string(),
                local_resolver_ip,
                add_recv_ns_udp,
                delete_recv_ns_udp,
                add_recv_ns_tcp,
                delete_recv_ns_tcp,
            );
        });

        thread::spawn(move || {
            resolver.run_resolver(add_recv_udp, delete_recv_udp, add_recv_tcp, delete_recv_tcp);
        });

        thread::sleep(Duration::new(1, 0));

        let mut ip_and_port_name_server = NAME_SERVER_IP.to_string();
        ip_and_port_name_server.push_str(":53");

        // Send msg TCP
        let mut client_stream =
            TcpStream::connect(ip_and_port_name_server).expect("Failed to connect");

        client_stream.write(&full_msg);
        //

        println!("{}", "Query sent");

        // Receive response TCP
        let mut response = [0; 512];

        let bytes_read = client_stream.read(&mut response).expect("No receive msg");

        let dns_response = DnsMessage::from_bytes(&response[2..]);
        let answers = dns_response.get_answer();
        let authority = dns_response.get_authority();
        let additional = dns_response.get_additional();

        assert_eq!(dns_response.get_query_id(), id);
        assert_eq!(answers.len(), 6);
        assert_eq!(authority.len(), 0);
        assert_eq!(additional.len(), 0);
        assert_eq!(dns_response.get_header().get_aa(), true);

        let expected_ips = [
            [64, 233, 186, 100],
            [64, 233, 186, 101],
            [64, 233, 186, 102],
            [64, 233, 186, 113],
            [64, 233, 186, 138],
            [64, 233, 186, 139],
        ];

        for answer in answers {
            let ip_address = match answer.get_rdata() {
                Rdata::SomeARdata(val) => val.get_address(),
                _ => unreachable!(),
            };

            assert!(expected_ips.contains(&ip_address));
            //
        }
    }
    */
}
