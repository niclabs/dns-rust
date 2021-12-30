mod global_tests {
    use crate::resolver::slist::Slist;
    use crate::resolver::Resolver;

    use std::net::UdpSocket;
    use std::net::{TcpStream};
    use std::io::BufRead;
    use std::io::BufReader;
    use std::io::Read;
    use std::io::Write;
    use std::str::from_utf8;
    use std::sync::mpsc;

    use crate::config::IP_PORT;

    //#[test]
    fn udp_response_test() {
    }

    #[test]
    fn tcp_response_test() {        
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
        print!("2\n");

        resolver.run_resolver(add_recv_udp, delete_recv_udp, add_recv_tcp, delete_recv_tcp);  
        print!("3\n");
    }
}
