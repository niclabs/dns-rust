use crate::config::RECURSIVE_AVAILABLE;
use crate::dns_cache::DnsCache;
use crate::message::rdata::Rdata;
use crate::message::resource_record::ResourceRecord;
use crate::message::DnsMessage;
use crate::name_server::zone::NSZone;
use crate::name_server::zone_node::NSNode;
use crate::name_server::zone_refresh::ZoneRefresh;
use crate::resolver::Resolver;

use chrono::Utc;
use rand::{thread_rng, Rng};
use std::cmp;
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;
use std::net::UdpSocket;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

pub mod master_file;
pub mod zone;
pub mod zone_node;
pub mod zone_refresh;

#[derive(Clone)]
/// Structs that represents a name server
pub struct NameServer {
    zones_by_class: HashMap<u16, HashMap<String, NSZone>>,
    cache: DnsCache,
    // For refreshing zone
    primary_server: bool,
    refresh_zones_data: HashMap<String, ZoneRefresh>,
    // Ids for Soa rrs queries to refresh zone
    queries_id_for_soa_rr: HashMap<u16, String>,
    // Ids from queries
    queries_id: HashMap<u16, Vec<(u16, String)>>,
    // Channel to share cache data between threads
    delete_sender_udp: Sender<(String, ResourceRecord)>,
    // Channel to share cache data between threads
    delete_sender_tcp: Sender<(String, ResourceRecord)>,
    // Channel to share cache data between name server and resolver
    add_sender_ns_udp: Sender<(String, ResourceRecord)>,
    // Channel to delete cache data in name server and resolver
    delete_sender_ns_udp: Sender<(String, ResourceRecord)>,
    // Channel to share cache data between name server and resolver
    add_sender_ns_tcp: Sender<(String, ResourceRecord)>,
    // Channel to delete cache data in name server and resolver
    delete_sender_ns_tcp: Sender<(String, ResourceRecord)>,
    // Channel to update the zones when AXFR msg arrive
    update_refresh_zone_udp: Sender<ZoneRefresh>,
    // Channel to update the zones when AXFR msg arrive
    update_refresh_zone_tcp: Sender<ZoneRefresh>,
    // Channel to update the zones when AXFR msg arrive
    update_zone_udp_resolver: Sender<NSZone>,
    // Channel to update the zones when AXFR msg arrive
    update_zone_tcp_resolver: Sender<NSZone>,
}

impl NameServer {
    /// Creates a new name server
    pub fn new(
        primary_server: bool,
        delete_channel_udp: Sender<(String, ResourceRecord)>,
        delete_channel_tcp: Sender<(String, ResourceRecord)>,
        add_channel_ns_udp: Sender<(String, ResourceRecord)>,
        delete_channel_ns_udp: Sender<(String, ResourceRecord)>,
        add_channel_ns_tcp: Sender<(String, ResourceRecord)>,
        delete_channel_ns_tcp: Sender<(String, ResourceRecord)>,
        update_refresh_zone_udp: Sender<ZoneRefresh>,
        update_refresh_zone_tcp: Sender<ZoneRefresh>,
        update_zone_udp_resolver: Sender<NSZone>,
        update_zone_tcp_resolver: Sender<NSZone>,
    ) -> Self {
        let name_server = NameServer {
            zones_by_class: HashMap::<u16, HashMap<String, NSZone>>::new(),
            cache: DnsCache::new(),
            queries_id: HashMap::<u16, Vec<(u16, String)>>::new(),
            queries_id_for_soa_rr: HashMap::<u16, String>::new(),
            primary_server: primary_server,
            refresh_zones_data: HashMap::<String, ZoneRefresh>::new(),
            delete_sender_udp: delete_channel_udp,
            delete_sender_tcp: delete_channel_tcp,
            add_sender_ns_udp: add_channel_ns_udp,
            delete_sender_ns_udp: delete_channel_ns_udp,
            add_sender_ns_tcp: add_channel_ns_tcp,
            delete_sender_ns_tcp: delete_channel_ns_tcp,
            update_refresh_zone_udp: update_refresh_zone_udp,
            update_refresh_zone_tcp: update_refresh_zone_tcp,
            update_zone_udp_resolver: update_zone_udp_resolver,
            update_zone_tcp_resolver: update_zone_tcp_resolver,
        };

        name_server
    }

    /// Runs an UDP and TCP Name server
    pub fn run_name_server(
        &mut self,
        name_server_ip_address: String,
        local_resolver_ip_and_port: String,
        rx_add_ns_udp: Receiver<(String, ResourceRecord)>,
        rx_delete_ns_udp: Receiver<(String, ResourceRecord)>,
        rx_add_ns_tcp: Receiver<(String, ResourceRecord)>,
        rx_delete_ns_tcp: Receiver<(String, ResourceRecord)>,
        rx_update_cache_ns_udp: Receiver<(String, String, u32)>,
        rx_update_cache_ns_tcp: Receiver<(String, String, u32)>,
        rx_update_refresh_zone_udp: Receiver<ZoneRefresh>,
        rx_update_refresh_zone_tcp: Receiver<ZoneRefresh>,
    ) {
        let mut name_server_copy = self.clone();
        let name_server_ip_address_copy = name_server_ip_address.clone();
        let local_resolver_ip_and_port_copy = local_resolver_ip_and_port.clone();

        thread::spawn(move || {
            name_server_copy.run_name_server_udp(
                name_server_ip_address_copy,
                local_resolver_ip_and_port_copy,
                rx_add_ns_udp,
                rx_delete_ns_udp,
                rx_update_cache_ns_udp,
                rx_update_refresh_zone_udp,
            );
        });

        self.run_name_server_tcp(
            name_server_ip_address,
            local_resolver_ip_and_port,
            rx_add_ns_tcp,
            rx_delete_ns_tcp,
            rx_update_cache_ns_tcp,
            rx_update_refresh_zone_tcp,
        );
    }

    fn run_name_server_udp(
        &mut self,
        mut name_server_ip_address: String,
        local_resolver_ip_and_port: String,
        rx_add_ns_udp: Receiver<(String, ResourceRecord)>,
        rx_delete_ns_udp: Receiver<(String, ResourceRecord)>,
        rx_update_cache_ns_udp: Receiver<(String, String, u32)>,
        rx_update_refresh_zone_udp: Receiver<ZoneRefresh>,
    ) {
        // Hashmap to save incomplete messages
        let messages = HashMap::<u16, DnsMessage>::new();

        // Add port 53 to ip address
        name_server_ip_address.push_str(":53");

        // Chanel to share the ids queries
        let (tx, rx) = mpsc::channel();

        // Channels to send data between threads, resolvers and name server
        let tx_delete_udp = self.get_delete_channel_udp();
        let tx_delete_tcp = self.get_delete_channel_tcp();
        let tx_delete_ns_udp = self.get_delete_channel_ns_udp();
        let tx_delete_ns_tcp = self.get_delete_channel_ns_tcp();

        // Channel to update zones
        let tx_update_refresh_zone_udp = self.get_update_refresh_zone_udp();
        let tx_update_refresh_zone_tcp = self.get_update_refresh_zone_tcp();

        // Channel to update zones in local resolvers
        let tx_update_zone_udp_resolver = self.get_update_zone_udp_resolver();
        let tx_update_zone_tcp_resolver = self.get_update_zone_tcp_resolver();

        // Creates refresh data for zones
        let primary_server = self.get_primary_server();
        let mut refresh_data = self.get_refresh_zones_data();
        let mut minimum_refresh: u32 = 2147483648;

        if primary_server == false {
            let zones_by_class = self.get_zones_by_class();

            for (_key, val) in zones_by_class.iter() {
                for (second_key, second_val) in val.iter() {
                    let zone_data = ZoneRefresh::new(second_val.clone());
                    let zone_refresh = zone_data.get_refresh();

                    if zone_refresh < minimum_refresh {
                        minimum_refresh = zone_refresh;
                    }

                    refresh_data.insert(second_key.to_string(), zone_data);
                }
            }

            self.set_refresh_zones_data(refresh_data);
        }

        // Creates an UDP socket
        let socket = UdpSocket::bind(&name_server_ip_address).expect("Failed to bind host socket");

        if primary_server == false {
            // Setting read timeout
            let read_timeout = Duration::new(minimum_refresh.into(), 0);

            socket
                .set_read_timeout(Some(read_timeout))
                .expect("set_read_timeout call failed");
            //
        }

        println!("{}", "Socket Created");

        loop {
            // Updates zone refresh and zones
            let mut received_update_refresh_zone = rx_update_refresh_zone_udp.try_iter();

            let mut next_value = received_update_refresh_zone.next();

            let mut refresh_zones = self.get_refresh_zones_data();
            let mut zones_by_class = self.get_zones_by_class();

            while next_value.is_none() == false {
                let updated_refresh_zone = next_value.unwrap();
                let zone = updated_refresh_zone.get_zone();
                let zone_name = zone.get_name();
                let zone_class = zone.get_class();

                tx_update_zone_udp_resolver
                    .send(zone.clone())
                    .expect("Couldn't update resolver zone using UDP");
                tx_update_zone_tcp_resolver
                    .send(zone.clone())
                    .expect("Couldn't update resolver zone using TCP");

                refresh_zones.insert(zone_name.clone(), updated_refresh_zone);

                let mut new_zone_hash = HashMap::new();
                new_zone_hash.insert(zone_name, zone);

                zones_by_class.insert(zone_class, new_zone_hash);

                next_value = received_update_refresh_zone.next();
            }

            self.set_zones_by_class(zones_by_class);
            self.set_refresh_zones_data(refresh_zones);
            //

            // Checking refresh queries
            let mut queries_id_for_soa_rr = self.get_queries_id_for_soa_rr();
            let mut refresh_zone_data = self.get_refresh_zones_data();

            for (key, val) in queries_id_for_soa_rr.clone().iter() {
                let mut query_zone = refresh_zone_data.get(val).unwrap().clone();

                let last_query_timestamp = query_zone.get_timestamp();
                let now = Utc::now();
                let now_timestamp = now.timestamp() as u32;

                let retry = query_zone.get_retry();

                if (now_timestamp - last_query_timestamp) > retry {
                    query_zone.set_last_fails(true);
                }

                refresh_zone_data.insert(val.to_string(), query_zone.clone());
                queries_id_for_soa_rr.remove(key);
            }

            self.set_queries_id_for_soa_rr(queries_id_for_soa_rr);
            self.set_refresh_zones_data(refresh_zone_data);

            //

            // Sending queries for Soa RR's for Zone refreshing

            if primary_server == false {
                let mut refresh_data = self.get_refresh_zones_data();

                for (key, val) in refresh_data.iter_mut() {
                    let last_serial_check = val.get_last_serial_check();
                    let now = Utc::now();
                    let now_timestamp = now.timestamp() as u32;
                    let time_between_now_and_last_serial_check = now_timestamp - last_serial_check;
                    let expire = val.get_expire();

                    // Expire time for the zone
                    if expire < time_between_now_and_last_serial_check {
                        // Inactive zone
                        if val.get_zone().get_active() == true {
                            let mut zone = val.get_zone();
                            zone.set_active(false);

                            tx_update_zone_udp_resolver
                                .send(zone.clone())
                                .expect("Couldn't update the zone using UDP to resolver");
                            tx_update_zone_tcp_resolver
                                .send(zone.clone())
                                .expect("Couldn't update the zone using TCP to resolver");

                            val.set_zone(zone);

                            tx_update_refresh_zone_udp
                                .send(val.clone())
                                .expect("Couldn't update the refresh zone using UDP to resolver");
                            tx_update_refresh_zone_tcp
                                .send(val.clone())
                                .expect("Couldn't update the refresh zone using TCP to resolver");
                        }
                    } else {
                        let last_timestamp = val.get_timestamp();
                        let last_fails = val.get_last_fails();
                        let time_between_queries = now_timestamp - last_timestamp;

                        if last_fails == true {
                            let retry = val.get_retry();

                            if time_between_queries > retry {
                                let zone = val.get_zone();
                                let msg = DnsMessage::soa_rr_query_msg(zone.clone());
                                let msg_id = msg.get_query_id();
                                let mut queries_id_for_soa_rr = self.get_queries_id_for_soa_rr();
                                queries_id_for_soa_rr.insert(msg_id, key.to_string());
                                self.set_queries_id_for_soa_rr(queries_id_for_soa_rr);

                                let msg_to_bytes = msg.to_bytes();

                                socket
                                    .send_to(&msg_to_bytes, zone.get_ip_address_for_refresh_zone())
                                    .expect(
                                        "Couldn't send the message to the resresh zone using UDP",
                                    );

                                val.set_timestamp(now_timestamp);
                            }
                        } else {
                            let refresh = val.get_refresh();

                            if time_between_queries > refresh {
                                let zone = val.get_zone();
                                let msg = DnsMessage::soa_rr_query_msg(zone.clone());
                                let msg_id = msg.get_query_id();
                                let mut queries_id_for_soa_rr = self.get_queries_id_for_soa_rr();
                                queries_id_for_soa_rr.insert(msg_id, key.to_string());
                                self.set_queries_id_for_soa_rr(queries_id_for_soa_rr);

                                let msg_to_bytes = msg.to_bytes();

                                socket
                                    .send_to(&msg_to_bytes, zone.get_ip_address_for_refresh_zone())
                                    .expect("Couldn't send the message to the zone using UDP");

                                val.set_timestamp(now_timestamp);
                            }
                        }
                    }
                }
                self.set_refresh_zones_data(refresh_data);
            }

            //

            println!("{}", "Waiting msg");

            // We receive the msg
            let dns_message_option =
                Resolver::receive_udp_msg(socket.try_clone().unwrap(), messages.clone());

            let mut dns_message;
            let src_address;

            println!("{}", "Message recv");

            match dns_message_option {
                Some(val) => {
                    dns_message = val.0;
                    src_address = val.1;
                }
                // If no msg
                None => {
                    (dns_message, src_address) = (DnsMessage::new(), "".to_string());
                }
            }
            //

            // Delete from cache

            let mut received_delete = rx_delete_ns_udp.try_iter();

            let mut next_value = received_delete.next();

            let mut cache = self.get_cache();

            while next_value.is_none() == false {
                let (name, rr) = next_value.unwrap();
                let rr_type = rr.get_string_type();
                cache.remove(name, rr_type);
                next_value = received_delete.next();
            }

            self.set_cache(cache);

            //

            // Update response time cache

            let mut received_update = rx_update_cache_ns_udp.try_iter();

            let mut next_value = received_update.next();

            let mut cache = self.get_cache();

            while next_value.is_none() == false {
                let (host_name, address, response_time) = next_value.unwrap();
                cache.update_response_time(host_name, "A".to_string(), response_time, address);
                next_value = received_update.next();
            }

            self.set_cache(cache);

            //

            // Adding to Cache

            let mut received_add = rx_add_ns_udp.try_iter();

            let mut next_value = received_add.next();

            let mut cache = self.get_cache();

            while next_value.is_none() == false {
                let (name, rr) = next_value.unwrap();
                cache.add(name, rr);
                next_value = received_add.next();
            }

            self.set_cache(cache);

            // Update queries ids

            let mut received = rx.try_iter();

            let mut next_value = received.next();

            let mut queries_id = self.get_queries_id();

            while next_value.is_none() == false {
                let (old, new) = next_value.unwrap();
                queries_id.insert(new, old);
                next_value = received.next();
            }

            self.set_queries_id(queries_id);

            //

            println!("{}", "Message recv");

            let socket_copy = socket.try_clone().unwrap();

            if dns_message.get_header().get_qr() == false {
                let op_code = dns_message.get_header().get_op_code();

                // If is an inverse query
                if op_code == 1 {
                    let not_implemented_msg = DnsMessage::not_implemented_msg();

                    NameServer::send_response_by_udp(
                        not_implemented_msg,
                        src_address.to_string(),
                        &socket_copy,
                    );

                    continue;
                }
                //

                let zones_by_class = self.get_zones_by_class();

                let cache = self.get_cache();

                let tx_clone = tx.clone();

                let resolver_ip_clone = local_resolver_ip_and_port.clone();

                let tx_delete_udp_copy = tx_delete_udp.clone();
                let tx_delete_tcp_copy = tx_delete_tcp.clone();
                let tx_delete_ns_udp_copy = tx_delete_ns_udp.clone();
                let tx_delete_ns_tcp_copy = tx_delete_ns_tcp.clone();

                thread::spawn(move || {
                    // default RA bit to 1
                    let mut ra = true;
                    let mut new_msg = NameServer::set_ra(dns_message.clone(), true);

                    // RA bit to 0
                    if RECURSIVE_AVAILABLE == false {
                        new_msg = NameServer::set_ra(dns_message, false);
                        ra = false;
                    }

                    let rd = new_msg.get_header().get_rd();

                    if rd == true && ra == true {
                        NameServer::step_5_udp(
                            resolver_ip_clone,
                            new_msg,
                            socket_copy,
                            tx_clone,
                            src_address,
                        );
                    } else {
                        let response_dns_msg = NameServer::step_2(
                            new_msg,
                            zones_by_class,
                            cache,
                            tx_delete_udp_copy,
                            tx_delete_tcp_copy,
                            tx_delete_ns_udp_copy,
                            tx_delete_ns_tcp_copy,
                        );

                        println!(
                            "Response answer len: {}",
                            response_dns_msg.get_answer().len()
                        );

                        NameServer::send_response_by_udp(
                            response_dns_msg,
                            src_address.to_string(),
                            &socket_copy,
                        );
                    }
                });
            } else {
                let mut queries_id = self.get_queries_id();
                let new_id = dns_message.get_query_id();

                println!("Pasa por respuesta");

                match queries_id.get(&new_id.clone()) {
                    Some(val) => {
                        let val_copy = val.clone();
                        println!("Encuentra la id en las queries id");
                        let mut header = dns_message.get_header();
                        header.set_id(val_copy[0].clone().0);
                        dns_message.set_header(header);
                        queries_id.remove(&new_id);

                        NameServer::send_response_by_udp(
                            dns_message,
                            val_copy[0].clone().1,
                            &socket_copy,
                        );
                    }
                    None => {
                        println!("No encuentra la id en las queries id");
                        // If the msg is a refresh soa rr query
                        let mut queries_id_for_soa_rr = self.get_queries_id_for_soa_rr();
                        let host_name_result = queries_id_for_soa_rr.get(&new_id);

                        match host_name_result {
                            Some(val) => {
                                let rcode = dns_message.get_header().get_rcode();
                                let qtype = dns_message.get_question().get_qtype();

                                let mut refresh_zone_data = self.get_refresh_zones_data();
                                let mut refresh_data_actual_zone =
                                    refresh_zone_data.get(val).unwrap().clone();

                                if rcode == 0 {
                                    if qtype == 6 {
                                        let soa_rr = dns_message.get_answer()[0].clone();
                                        let soa_rdata = match soa_rr.get_rdata() {
                                            Rdata::SomeSoaRdata(val) => val,
                                            _ => unreachable!(),
                                        };

                                        let serial = soa_rdata.get_serial();

                                        let new_serial_greater_than_old = refresh_data_actual_zone
                                            .new_serial_greater_than_old(serial);

                                        // Updates the last serial check timestamp
                                        let now = Utc::now();
                                        let now_timestamp = now.timestamp() as u32;

                                        refresh_data_actual_zone
                                            .set_last_serial_check(now_timestamp);
                                        //

                                        // Refresh zone necessary
                                        if new_serial_greater_than_old == true {
                                            // Copy variables for using in threads
                                            let mut refresh_data_actual_zone_copy =
                                                refresh_data_actual_zone.clone();

                                            let tx_update_refresh_zone_tcp_copy =
                                                tx_update_refresh_zone_tcp.clone();
                                            let tx_update_refresh_zone_udp_copy =
                                                tx_update_refresh_zone_udp.clone();
                                            //

                                            thread::spawn(move || {
                                                // Creates AXFR message
                                                let qname = refresh_data_actual_zone_copy
                                                    .get_zone()
                                                    .get_name();
                                                let axfr_message =
                                                    DnsMessage::axfr_query_message(qname);
                                                //

                                                // Send TCP query to the name server
                                                let mut stream = TcpStream::connect(
                                                    refresh_data_actual_zone_copy
                                                        .get_ip_address_for_refresh_zone(),
                                                )
                                                .expect("Connect Failed");

                                                let bytes = axfr_message.to_bytes();

                                                let msg_length: u16 = bytes.len() as u16;

                                                let tcp_bytes_length =
                                                    [(msg_length >> 8) as u8, msg_length as u8];

                                                let full_msg =
                                                    [&tcp_bytes_length, bytes.as_slice()].concat();

                                                stream
                                                    .write(&full_msg)
                                                    .expect("Couldn't write the message");
                                                //

                                                // Receive response from name server and parse the msg

                                                let received_msg = Resolver::receive_tcp_msg(
                                                    stream.try_clone().unwrap(),
                                                )
                                                .unwrap();

                                                let dns_axfr_msg_parse =
                                                    DnsMessage::from_bytes(&received_msg).unwrap();
                                                //

                                                // Check no errors and update zone
                                                if dns_axfr_msg_parse.get_header().get_rcode() == 0
                                                {
                                                    let mut update_zone =
                                                        NSZone::from_axfr_msg(dns_axfr_msg_parse);

                                                    update_zone.set_ip_address_for_refresh_zone(
                                                        refresh_data_actual_zone_copy
                                                            .get_ip_address_for_refresh_zone(),
                                                    );

                                                    // Update refresh zone with new soa values in tcp and udp name servers
                                                    refresh_data_actual_zone_copy
                                                        .update_zone_refresh(update_zone);

                                                    tx_update_refresh_zone_udp_copy
                                                        .send(
                                                            refresh_data_actual_zone_copy.clone(),
                                                        ).expect("Couldn't send the new refresh zone using UDP");

                                                    tx_update_refresh_zone_tcp_copy
                                                        .send(refresh_data_actual_zone_copy)
                                                        .expect("Couldn't send the new refresh zone using TCP");
                                                    //
                                                }
                                                //
                                            });
                                        }
                                        //
                                        else {
                                            // Updates the last query timestamp
                                            let now = Utc::now();
                                            let now_timestamp = now.timestamp() as u32;

                                            refresh_data_actual_zone.set_timestamp(now_timestamp);
                                            //
                                        }
                                    }
                                } else {
                                    refresh_data_actual_zone.set_last_fails(true);
                                }

                                refresh_zone_data
                                    .insert(val.to_string(), refresh_data_actual_zone.clone());
                                self.set_refresh_zones_data(refresh_zone_data);

                                queries_id_for_soa_rr.remove(&new_id);
                                self.set_queries_id_for_soa_rr(queries_id_for_soa_rr.clone());
                            }
                            None => {}
                        }
                    }
                }
            }
        }
    }

    fn run_name_server_tcp(
        &mut self,
        mut name_server_ip_address: String,
        local_resolver_ip_and_port: String,
        rx_add_ns_tcp: Receiver<(String, ResourceRecord)>,
        rx_delete_ns_tcp: Receiver<(String, ResourceRecord)>,
        rx_update_cache_ns_tcp: Receiver<(String, String, u32)>,
        rx_update_refresh_zone_tcp: Receiver<ZoneRefresh>,
    ) {
        name_server_ip_address.push_str(":53");

        // Channels to send data between threads, resolvers and name server
        let tx_delete_udp = self.get_delete_channel_udp();
        let tx_delete_tcp = self.get_delete_channel_tcp();
        let tx_delete_ns_udp = self.get_delete_channel_ns_udp();
        let tx_delete_ns_tcp = self.get_delete_channel_ns_tcp();

        // Channel to update zones
        let tx_update_refresh_zone_udp = self.get_update_refresh_zone_udp();
        let tx_update_refresh_zone_tcp = self.get_update_refresh_zone_tcp();

        // Channel to update zones in local resolvers
        let tx_update_zone_udp_resolver = self.get_update_zone_udp_resolver();
        let tx_update_zone_tcp_resolver = self.get_update_zone_tcp_resolver();

        // Creates refresh data for zones
        let primary_server = self.get_primary_server();
        let mut refresh_data = self.get_refresh_zones_data();
        let mut minimum_refresh: u32 = 2147483648;

        if primary_server == false {
            let zones_by_class = self.get_zones_by_class();

            for (_key, val) in zones_by_class.iter() {
                for (second_key, second_val) in val.iter() {
                    let zone_data = ZoneRefresh::new(second_val.clone());
                    let zone_refresh = zone_data.get_refresh();

                    if zone_refresh < minimum_refresh {
                        minimum_refresh = zone_refresh;
                    }

                    refresh_data.insert(second_key.to_string(), zone_data);
                }
            }

            self.set_refresh_zones_data(refresh_data);
        }

        // Creates a TCP Listener
        let listener = TcpListener::bind(&name_server_ip_address).expect("Could not bind");

        // Sets nonblocking listener
        if primary_server == false {
            listener
                .set_nonblocking(true)
                .expect("Cannot set non-blocking");
        }

        //

        println!("{}", "TcpListener Created");

        loop {
            //println!("{}", "Waiting msg");

            match listener.accept() {
                Ok((stream, src_address)) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());

                    // We receive the msg
                    let received_msg =
                        Resolver::receive_tcp_msg(stream.try_clone().unwrap()).unwrap();

                    println!("{}", "Message recv");

                    // Updates zone refresh and zones
                    let mut received_update_refresh_zone = rx_update_refresh_zone_tcp.try_iter();

                    let mut next_value = received_update_refresh_zone.next();

                    let mut refresh_zones = self.get_refresh_zones_data();
                    let mut zones_by_class = self.get_zones_by_class();

                    while next_value.is_none() == false {
                        let updated_refresh_zone = next_value.unwrap();
                        let zone = updated_refresh_zone.get_zone();
                        let zone_name = zone.get_name();
                        let zone_class = zone.get_class();

                        tx_update_zone_udp_resolver
                            .send(zone.clone())
                            .expect("Couldn't send the new zone via UDP to resolver");
                        tx_update_zone_tcp_resolver
                            .send(zone.clone())
                            .expect("Couldn't send the new zone via TCP to resolver");

                        refresh_zones.insert(zone_name.clone(), updated_refresh_zone);

                        let mut new_zone_hashmap = HashMap::new();
                        new_zone_hashmap.insert(zone_name, zone);

                        zones_by_class.insert(zone_class, new_zone_hashmap);

                        next_value = received_update_refresh_zone.next();
                    }

                    self.set_zones_by_class(zones_by_class);
                    self.set_refresh_zones_data(refresh_zones);
                    //

                    // Delete from cache

                    let mut received_delete = rx_delete_ns_tcp.try_iter();

                    let mut next_value = received_delete.next();

                    let mut cache = self.get_cache();

                    while next_value.is_none() == false {
                        let (name, rr) = next_value.unwrap();
                        let rr_type = rr.get_string_type();
                        cache.remove(name, rr_type);
                        next_value = received_delete.next();
                    }

                    self.set_cache(cache);

                    //

                    // Update response time cache

                    let mut received_update = rx_update_cache_ns_tcp.try_iter();

                    let mut next_value = received_update.next();

                    let mut cache = self.get_cache();

                    while next_value.is_none() == false {
                        let (host_name, address, response_time) = next_value.unwrap();
                        cache.update_response_time(
                            host_name,
                            "A".to_string(),
                            response_time,
                            address,
                        );
                        next_value = received_update.next();
                    }

                    self.set_cache(cache);

                    //

                    // Adding to Cache

                    let mut received_add = rx_add_ns_tcp.try_iter();

                    let mut next_value = received_add.next();

                    let mut cache = self.get_cache();

                    while next_value.is_none() == false {
                        let (name, rr) = next_value.unwrap();
                        cache.add(name, rr);
                        next_value = received_add.next();
                    }

                    self.set_cache(cache);

                    // Msg parsed
                    let dns_message_parse_result = DnsMessage::from_bytes(&received_msg);

                    match dns_message_parse_result {
                        Ok(_) => {}
                        Err(_) => {
                            let dns_msg_format_error = DnsMessage::format_error_msg();

                            NameServer::send_response_by_tcp(
                                dns_msg_format_error,
                                src_address.clone().to_string(),
                                stream,
                            );

                            continue;
                        }
                    }

                    let dns_message = dns_message_parse_result.unwrap();

                    println!("{}", "Message parsed");

                    if dns_message.get_header().get_qr() == false {
                        let op_code = dns_message.get_header().get_op_code();

                        // If is an inverse query
                        if op_code == 1 {
                            let not_implemented_msg = DnsMessage::not_implemented_msg();

                            NameServer::send_response_by_tcp(
                                not_implemented_msg,
                                src_address.to_string(),
                                stream,
                            );

                            continue;
                        }
                        //

                        let zones_by_class = self.get_zones_by_class();
                        let cache = self.get_cache();
                        let resolver_ip_clone = local_resolver_ip_and_port.clone();

                        let tx_delete_udp_copy = tx_delete_udp.clone();
                        let tx_delete_tcp_copy = tx_delete_tcp.clone();
                        let tx_delete_ns_udp_copy = tx_delete_ns_udp.clone();
                        let tx_delete_ns_tcp_copy = tx_delete_ns_tcp.clone();

                        thread::spawn(move || {
                            let query_id = dns_message.get_query_id();
                            let qtype = dns_message.get_question().get_qtype();

                            if qtype == 253 {
                                NameServer::send_axfr_response(
                                    dns_message.clone(),
                                    src_address.clone().to_string(),
                                    zones_by_class.clone(),
                                    stream,
                                );
                            } else {
                                // Set RA bit to 1
                                let new_msg = NameServer::set_ra(dns_message, true);

                                let rd = new_msg.get_header().get_rd();

                                if rd == true {
                                    println!("RD true");

                                    let mut response_dns_msg = NameServer::step_5_tcp(
                                        resolver_ip_clone,
                                        new_msg,
                                        cache.clone(),
                                        zones_by_class.clone(),
                                    );

                                    response_dns_msg.set_query_id(query_id.clone());

                                    println!("{:#?}", response_dns_msg.to_bytes());

                                    NameServer::send_response_by_tcp(
                                        response_dns_msg,
                                        src_address.to_string(),
                                        stream.try_clone().unwrap(),
                                    );
                                } else {
                                    let mut response_dns_msg = NameServer::step_2(
                                        new_msg,
                                        zones_by_class,
                                        cache,
                                        tx_delete_udp_copy,
                                        tx_delete_tcp_copy,
                                        tx_delete_ns_udp_copy,
                                        tx_delete_ns_tcp_copy,
                                    );

                                    response_dns_msg.set_query_id(query_id);

                                    NameServer::send_response_by_tcp(
                                        response_dns_msg,
                                        src_address.to_string(),
                                        stream,
                                    );
                                }
                            }
                        });
                    }
                }
                Err(_) => {
                    // Checking refresh queries
                    let mut queries_id_for_soa_rr = self.get_queries_id_for_soa_rr();
                    let mut refresh_zone_data = self.get_refresh_zones_data();

                    for (key, val) in queries_id_for_soa_rr.clone().iter() {
                        let mut query_zone = refresh_zone_data.get(val).unwrap().clone();

                        let last_query_timestamp = query_zone.get_timestamp();
                        let now = Utc::now();
                        let now_timestamp = now.timestamp() as u32;

                        let retry = query_zone.get_retry();

                        if (now_timestamp - last_query_timestamp) > retry {
                            query_zone.set_last_fails(true);
                        }

                        refresh_zone_data.insert(val.to_string(), query_zone.clone());
                        queries_id_for_soa_rr.remove(key);
                    }

                    self.set_queries_id_for_soa_rr(queries_id_for_soa_rr);
                    self.set_refresh_zones_data(refresh_zone_data);

                    //

                    // Sending queries for Soa RR's for Zone refreshing

                    if primary_server == false {
                        let mut refresh_data = self.get_refresh_zones_data();

                        for (key, val) in refresh_data.clone().iter_mut() {
                            let last_serial_check = val.get_last_serial_check();
                            let now = Utc::now();
                            let now_timestamp = now.timestamp() as u32;
                            let time_between_now_and_last_serial_check =
                                now_timestamp - last_serial_check;
                            let expire = val.get_expire();

                            // Zone expire
                            if time_between_now_and_last_serial_check > expire {
                                let mut zone = val.get_zone();

                                if zone.get_active() == true {
                                    zone.set_active(false);

                                    tx_update_zone_udp_resolver
                                        .send(zone.clone())
                                        .expect("Couldn't send the new zone using UDP to resolver");
                                    tx_update_zone_tcp_resolver
                                        .send(zone.clone())
                                        .expect("Couldn't send the new zone using TCP to resolver");

                                    val.set_zone(zone);

                                    tx_update_refresh_zone_udp.send(val.clone()).expect(
                                        "Couldn't send the new refresh zone using UDP to resolver",
                                    );
                                    tx_update_refresh_zone_tcp.send(val.clone()).expect(
                                        "Couldn't send the new refresh zone using TCP to resolver",
                                    );
                                }
                            } else {
                                let last_timestamp = val.get_timestamp();
                                let last_fails = val.get_last_fails();
                                let time_between_queries = now_timestamp - last_timestamp;

                                if last_fails == true {
                                    let retry = val.get_retry();

                                    if time_between_queries > retry {
                                        let zone = val.get_zone();
                                        let msg = DnsMessage::soa_rr_query_msg(zone.clone());
                                        let msg_id = msg.get_query_id();
                                        let mut queries_id_for_soa_rr =
                                            self.get_queries_id_for_soa_rr();
                                        queries_id_for_soa_rr.insert(msg_id, key.to_string());
                                        self.set_queries_id_for_soa_rr(queries_id_for_soa_rr);

                                        let msg_to_bytes = msg.to_bytes();

                                        // Adds the two bytes needs for tcp
                                        let msg_length: u16 = msg_to_bytes.len() as u16;
                                        let tcp_bytes_length =
                                            [(msg_length >> 8) as u8, msg_length as u8];
                                        let full_msg =
                                            [&tcp_bytes_length, msg_to_bytes.as_slice()].concat();

                                        // Send query to local resolver
                                        let mut stream = TcpStream::connect(
                                            zone.get_ip_address_for_refresh_zone(),
                                        )
                                        .unwrap();

                                        stream
                                            .set_read_timeout(Some(Duration::new(2, 0)))
                                            .expect("set_read_timeout call failed");

                                        stream
                                            .write(&full_msg)
                                            .expect("Couldn't write the message");

                                        let mut received_msg = Vec::new();
                                        let bytes_readed = stream.read(&mut received_msg).unwrap();

                                        if bytes_readed == 0 {
                                            val.set_last_fails(true);
                                            val.set_timestamp(now_timestamp);
                                        } else {
                                            let msg =
                                                DnsMessage::from_bytes(&received_msg).unwrap();
                                            let qtype = msg.get_question().get_qtype();
                                            let rcode = msg.get_header().get_rcode();

                                            if rcode == 0 {
                                                if qtype == 6 {
                                                    let soa_rr = msg.get_answer()[0].clone();
                                                    let soa_rdata = match soa_rr.get_rdata() {
                                                        Rdata::SomeSoaRdata(val) => val,
                                                        _ => unreachable!(),
                                                    };

                                                    let serial = soa_rdata.get_serial();

                                                    let new_serial_greater_than_old =
                                                        val.new_serial_greater_than_old(serial);

                                                    // Refresh zone necessary
                                                    if new_serial_greater_than_old == true {
                                                        // Clone values for using in threads
                                                        let mut val_copy = val.clone();
                                                        let tx_update_refresh_zone_tcp_copy =
                                                            tx_update_refresh_zone_tcp.clone();
                                                        let tx_update_refresh_zone_udp_copy =
                                                            tx_update_refresh_zone_udp.clone();
                                                        //

                                                        thread::spawn(move || {
                                                            // Creates AXFR message
                                                            let qname = val_copy
                                                                .clone()
                                                                .get_zone()
                                                                .get_name();
                                                            let axfr_message =
                                                                DnsMessage::axfr_query_message(
                                                                    qname,
                                                                );
                                                            //

                                                            // Send TCP query to the name server
                                                            let mut stream = TcpStream::connect(
                                                                val_copy
                                                                    .get_ip_address_for_refresh_zone(),
                                                            )
                                                            .expect("Connect Failed");

                                                            let bytes = axfr_message.to_bytes();

                                                            let msg_length: u16 =
                                                                bytes.len() as u16;

                                                            let tcp_bytes_length = [
                                                                (msg_length >> 8) as u8,
                                                                msg_length as u8,
                                                            ];

                                                            let full_msg = [
                                                                &tcp_bytes_length,
                                                                bytes.as_slice(),
                                                            ]
                                                            .concat();

                                                            stream.write(&full_msg).expect(
                                                                "Couldn't write the message",
                                                            );
                                                            //

                                                            // Receive response from name server and parse the msg

                                                            let received_msg =
                                                                Resolver::receive_tcp_msg(
                                                                    stream.try_clone().unwrap(),
                                                                )
                                                                .unwrap();

                                                            let dns_axfr_msg_parse =
                                                                DnsMessage::from_bytes(
                                                                    &received_msg,
                                                                )
                                                                .unwrap();
                                                            //

                                                            // Check no errors and update zone
                                                            if dns_axfr_msg_parse
                                                                .get_header()
                                                                .get_rcode()
                                                                == 0
                                                            {
                                                                let mut update_zone =
                                                                    NSZone::from_axfr_msg(
                                                                        dns_axfr_msg_parse,
                                                                    );

                                                                update_zone.set_ip_address_for_refresh_zone(
                                                                    val_copy
                                                                        .get_ip_address_for_refresh_zone(),
                                                                );

                                                                // Update refresh zone with new soa values in tcp and udp name servers
                                                                val_copy.update_zone_refresh(
                                                                    update_zone,
                                                                );

                                                                tx_update_refresh_zone_udp_copy
                                                                    .send(val_copy.clone())
                                                                    .expect("Couldn't send the new refresh zone using UDP");

                                                                tx_update_refresh_zone_tcp_copy
                                                                    .send(val_copy.clone())
                                                                    .expect("Couldn't send the new refresh zone using TCP");
                                                                //
                                                            }
                                                            //
                                                        });
                                                    }
                                                    //
                                                    else {
                                                        // Updates the last query timestamp
                                                        let now = Utc::now();
                                                        let now_timestamp = now.timestamp() as u32;

                                                        val.set_timestamp(now_timestamp);
                                                        //
                                                    }
                                                }
                                            } else {
                                                val.set_last_fails(true);
                                                val.set_timestamp(now_timestamp);
                                            }
                                        }
                                        refresh_data.insert(key.to_string(), val.clone());
                                    }
                                } else {
                                    let refresh = val.get_refresh();

                                    if time_between_queries > refresh {
                                        let zone = val.get_zone();
                                        let msg = DnsMessage::soa_rr_query_msg(zone.clone());
                                        let msg_id = msg.get_query_id();
                                        let mut queries_id_for_soa_rr =
                                            self.get_queries_id_for_soa_rr();
                                        queries_id_for_soa_rr.insert(msg_id, key.to_string());
                                        self.set_queries_id_for_soa_rr(queries_id_for_soa_rr);

                                        let msg_to_bytes = msg.to_bytes();

                                        // Adds the two bytes needs for tcp
                                        let msg_length: u16 = msg_to_bytes.len() as u16;
                                        let tcp_bytes_length =
                                            [(msg_length >> 8) as u8, msg_length as u8];
                                        let full_msg =
                                            [&tcp_bytes_length, msg_to_bytes.as_slice()].concat();

                                        // Send query to local resolver
                                        let mut stream = TcpStream::connect(
                                            zone.get_ip_address_for_refresh_zone(),
                                        )
                                        .unwrap();

                                        stream
                                            .set_read_timeout(Some(Duration::new(2, 0)))
                                            .expect("set_read_timeout call failed");

                                        stream
                                            .write(&full_msg)
                                            .expect("Couldn't write the message");

                                        let mut received_msg = Vec::new();
                                        let bytes_readed = stream.read(&mut received_msg).unwrap();

                                        if bytes_readed == 0 {
                                            val.set_last_fails(true);
                                            val.set_timestamp(now_timestamp);
                                        } else {
                                            let msg =
                                                DnsMessage::from_bytes(&received_msg).unwrap();
                                            let qtype = msg.get_question().get_qtype();
                                            let rcode = msg.get_header().get_rcode();

                                            if rcode == 0 {
                                                if qtype == 6 {
                                                    let soa_rr = msg.get_answer()[0].clone();
                                                    let soa_rdata = match soa_rr.get_rdata() {
                                                        Rdata::SomeSoaRdata(rdata) => rdata,
                                                        _ => unreachable!(),
                                                    };

                                                    let serial = soa_rdata.get_serial();

                                                    let new_serial_greater_than_old =
                                                        val.new_serial_greater_than_old(serial);

                                                    // Refresh zone necessary
                                                    if new_serial_greater_than_old == true {
                                                        // Clone values for using in threads
                                                        let mut val_copy = val.clone();
                                                        let tx_update_refresh_zone_tcp_copy =
                                                            tx_update_refresh_zone_tcp.clone();
                                                        let tx_update_refresh_zone_udp_copy =
                                                            tx_update_refresh_zone_udp.clone();
                                                        //

                                                        thread::spawn(move || {
                                                            // Creates AXFR message
                                                            let qname = val_copy
                                                                .clone()
                                                                .get_zone()
                                                                .get_name();
                                                            let axfr_message =
                                                                DnsMessage::axfr_query_message(
                                                                    qname,
                                                                );
                                                            //

                                                            // Send TCP query to the name server
                                                            let mut stream = TcpStream::connect(
                                                                val_copy
                                                                    .get_ip_address_for_refresh_zone(),
                                                            )
                                                            .expect("Connect Failed");

                                                            let bytes = axfr_message.to_bytes();

                                                            let msg_length: u16 =
                                                                bytes.len() as u16;

                                                            let tcp_bytes_length = [
                                                                (msg_length >> 8) as u8,
                                                                msg_length as u8,
                                                            ];

                                                            let full_msg = [
                                                                &tcp_bytes_length,
                                                                bytes.as_slice(),
                                                            ]
                                                            .concat();

                                                            stream.write(&full_msg).expect(
                                                                "Couldn't write the message",
                                                            );
                                                            //

                                                            // Receive response from name server and parse the msg

                                                            let received_msg =
                                                                Resolver::receive_tcp_msg(
                                                                    stream.try_clone().unwrap(),
                                                                )
                                                                .unwrap();

                                                            let dns_axfr_msg_parse =
                                                                DnsMessage::from_bytes(
                                                                    &received_msg,
                                                                )
                                                                .unwrap();
                                                            //

                                                            // Check no errors and update zone
                                                            if dns_axfr_msg_parse
                                                                .get_header()
                                                                .get_rcode()
                                                                == 0
                                                            {
                                                                let mut update_zone =
                                                                    NSZone::from_axfr_msg(
                                                                        dns_axfr_msg_parse,
                                                                    );

                                                                update_zone.set_ip_address_for_refresh_zone(
                                                                    val_copy
                                                                        .get_ip_address_for_refresh_zone(),
                                                                );

                                                                // Update refresh zone with new soa values in tcp and udp name servers
                                                                val_copy.update_zone_refresh(
                                                                    update_zone,
                                                                );

                                                                tx_update_refresh_zone_udp_copy
                                                                    .send(val_copy.clone())
                                                                    .expect("Couldn't send the new refresh zone using UDP");

                                                                tx_update_refresh_zone_tcp_copy
                                                                    .send(val_copy.clone())
                                                                    .expect("Couldn't send the new refresh zone using TCP");
                                                                //
                                                            }
                                                            //
                                                        });
                                                    }
                                                    //
                                                    else {
                                                        // Updates the last query timestamp
                                                        let now = Utc::now();
                                                        let now_timestamp = now.timestamp() as u32;

                                                        val.set_timestamp(now_timestamp);
                                                        //
                                                    }
                                                }
                                            } else {
                                                val.set_last_fails(true);
                                                val.set_timestamp(now_timestamp);
                                            }
                                        }
                                        refresh_data.insert(key.to_string(), val.clone());
                                    }
                                }
                            }
                        }
                        self.set_refresh_zones_data(refresh_data);
                    }

                    //
                }
            }
        }
    }
}

// Utils for TCP and UDP
impl NameServer {
    // FIXME:
    // Step 2 from RFC 1034
    // find the zone by the qname and class, if is not found goes to find the label of the parent
    pub fn search_nearest_ancestor_zone(
        zones: HashMap<u16, HashMap<String, NSZone>>,
        mut qname: String,
        qclass: u16,
    ) -> (NSZone, bool) {
        // Get the zone by class
        let zones_by_class_option = zones.get(&qclass);

        match zones_by_class_option {
            Some(_) => {}
            None => return (NSZone::new(), false),
        }
        //

        let zones_by_class = zones_by_class_option.unwrap();

        let (zone, available) = match zones_by_class.get(&qname) {
            Some(val) => (val.clone(), true),
            None => (NSZone::new(), false),
        };

        if zone.get_name() != "" && zone.get_active() == true {
            return (zone, available);
        } 
        else {
            let dot_position = qname.find(".").unwrap_or(0);
            if dot_position > 0 {
                qname.replace_range(..dot_position + 1, "");
                return NameServer::search_nearest_ancestor_zone(zones, qname, qclass);
            } else {
                qname = String::from(".");
                return NameServer::search_nearest_ancestor_zone(zones, qname, qclass);
            }
        }
    }

    // Step 2 from RFC 1034
    //Looks for the nearest ancestor
    pub fn search_nearest_ancestor(
        zones: HashMap<u16, HashMap<String, NSZone>>,
        mut qname: String,
        qclass: u16,
    ) -> (NSZone, bool) {
        // Get the zone by class
        let zones_by_class_option = zones.get(&qclass);

        match zones_by_class_option {
            Some(_) => {}
            None => return (NSZone::new(), false),
        }
        //

        let zones_by_class = zones_by_class_option.unwrap();

        let (zone, available) = match zones_by_class.get(&qname) {
            Some(val) => (val.clone(), true),
            None => (NSZone::new(), false),
        };

        if zone.get_name() != "" && zone.get_active() == true {
            return (zone, available);
        } else {
            let dot_position = qname.find(".").unwrap_or(0);
            if dot_position > 0 {
                qname.replace_range(..dot_position + 1, "");
                return NameServer::search_nearest_ancestor_zone(zones, qname, qclass);
            } else {
                return (zone, available);
            }
        }
    }

    /// Step 3 from RFC 1034
    /// Start matching down, label by label, in the zone.
    //TODO: cambiarle de nombre a algo step_3
    fn search_in_zone(
        zone: NSZone,
        qname: String,
        msg: DnsMessage,
        zones_by_class: HashMap<u16, HashMap<String, NSZone>>,
        cache: DnsCache,
        tx_delete_resolver_udp: Sender<(String, ResourceRecord)>,
        tx_delete_resolver_tcp: Sender<(String, ResourceRecord)>,
        tx_delete_ns_udp: Sender<(String, ResourceRecord)>,
        tx_delete_ns_tcp: Sender<(String, ResourceRecord)>,
    ) -> DnsMessage {
        let mut qname_without_zone_label = qname.replace(&zone.get_name(), "");
        let zone = zone.clone();

        let mut zone_nodes = zone.get_zone_nodes();

        println!("QNAME without label: {}", qname_without_zone_label.clone());

        // We were looking for the first node
        if qname_without_zone_label == "".to_string() {
            return NameServer::step_3a(
                zone_nodes,
                msg,
                zones_by_class,
                cache,
                tx_delete_resolver_udp,
                tx_delete_resolver_tcp,
                tx_delete_ns_udp,
                tx_delete_ns_tcp,
            );
        }

        // Delete last dot
        qname_without_zone_label.pop().unwrap();

        let mut labels: Vec<&str> = qname_without_zone_label.split(".").collect();

        labels.reverse();

        for label in labels {
            let exist_child = zone_nodes.exist_child(label.to_string());

            println!("Exist child: {}", exist_child.clone());

            if exist_child == true {
                zone_nodes = zone_nodes.get_child(label.to_string()).0.clone();

                // RFC 1034 - Step 3b:
                // If a match would take us out of the authoritative data
                // we have a referral.
                if zone_nodes.get_subzone() == true {
                    return NameServer::step_3b(
                        zone_nodes,
                        msg,
                        cache,
                        zones_by_class,
                        tx_delete_resolver_udp,
                        tx_delete_resolver_tcp,
                        tx_delete_ns_udp,
                        tx_delete_ns_tcp,
                    );
                } else {
                    continue;
                }
            } else {
                return NameServer::step_3c(zone_nodes, msg, cache, zones_by_class);
            }
        }

        return NameServer::step_3a(
            zone_nodes,
            msg,
            zones_by_class,
            cache,
            tx_delete_resolver_udp,
            tx_delete_resolver_tcp,
            tx_delete_ns_udp,
            tx_delete_ns_tcp,
        );
    }

    /// Step 2 RFC 1034
    /// Search the available zones for the zone which is the nearest
    /// ancestor to QNAME.  If such a zone is found, go to step 3,
    /// otherwise step 4.
    pub fn step_2(
        mut msg: DnsMessage,
        zones_by_class: HashMap<u16, HashMap<String, NSZone>>,
        cache: DnsCache,
        tx_delete_resolver_udp: Sender<(String, ResourceRecord)>,
        tx_delete_resolver_tcp: Sender<(String, ResourceRecord)>,
        tx_delete_ns_udp: Sender<(String, ResourceRecord)>,
        tx_delete_ns_tcp: Sender<(String, ResourceRecord)>,
    ) -> DnsMessage {
        let qname = msg.get_question().get_qname().get_name();
        let qclass = msg.get_question().get_qclass();

        // Class is *
        if qclass == 255 {
            let mut all_answers = Vec::new();

            // Gets all answers for all classes
            for (class, _hashzones) in zones_by_class.iter() {
                let (zone, available) = NameServer::search_nearest_ancestor_zone(
                    zones_by_class.clone(),
                    qname.clone(),
                    class.clone(),
                );

                if available == true {
                    let new_msg = NameServer::search_in_zone(
                        zone,
                        qname.clone(),
                        msg.clone(),
                        zones_by_class.clone(),
                        cache.clone(),
                        tx_delete_resolver_udp.clone(),
                        tx_delete_resolver_tcp.clone(),
                        tx_delete_ns_udp.clone(),
                        tx_delete_ns_tcp.clone(),
                    );

                    all_answers.append(&mut new_msg.get_answer());
                }
            }
            //

            // If answers were found
            if all_answers.len() > 0 {
                // Set answers
                msg.set_answer(all_answers);

                // Set AA to 0
                let mut header = msg.get_header();
                header.set_aa(false);
                msg.set_header(header);

                // Update header coutners
                msg.update_header_counters();

                return msg;
            } else {
                return NameServer::step_4(
                    msg,
                    cache,
                    zones_by_class,
                    tx_delete_resolver_udp,
                    tx_delete_resolver_tcp,
                    tx_delete_ns_udp,
                    tx_delete_ns_tcp,
                );
            }
            //
        } else {
            let (zone, available) = NameServer::search_nearest_ancestor_zone(
                zones_by_class.clone(),
                qname.clone(),
                qclass.clone(),
            );

            println!("Ancestor zone for {}: {}", qname.clone(), available.clone());

            if available == true {
                // Step 3 RFC 1034
                return NameServer::search_in_zone(
                    zone,
                    qname.clone(),
                    msg.clone(),
                    zones_by_class,
                    cache,
                    tx_delete_resolver_udp,
                    tx_delete_resolver_tcp,
                    tx_delete_ns_udp,
                    tx_delete_ns_tcp,
                );
            } else {
                // Step 4 RFC 1034
                return NameServer::step_4(
                    msg,
                    cache,
                    zones_by_class,
                    tx_delete_resolver_udp,
                    tx_delete_resolver_tcp,
                    tx_delete_ns_udp,
                    tx_delete_ns_tcp,
                );
            }
        }
    }

    /// RFC 1034 - Step 3.a:
    /// Step when the whole of QNAME is matched
    pub fn step_3a(
        matched_node: NSNode,
        mut msg: DnsMessage,
        zones_by_class: HashMap<u16, HashMap<String, NSZone>>,
        cache: DnsCache,
        tx_delete_resolver_udp: Sender<(String, ResourceRecord)>,
        tx_delete_resolver_tcp: Sender<(String, ResourceRecord)>,
        tx_delete_ns_udp: Sender<(String, ResourceRecord)>,
        tx_delete_ns_tcp: Sender<(String, ResourceRecord)>,
    ) -> DnsMessage {
        // Step 3.a
        let qtype = msg.get_question().get_qtype();
        let qclass = msg.get_question().get_qclass();
        let mut rrs_by_type = matched_node.get_rrs_by_type(qtype);

        println!("RRS len: {}", rrs_by_type.len());

        if rrs_by_type.len() > 0 {
            // Set the ttl from SOA RR
            let (main_zone, _available) = NameServer::search_nearest_ancestor_zone(
                zones_by_class.clone(),
                msg.get_question().get_qname().get_name(),
                qclass,
            );

            let soa_rr = main_zone.get_zone_nodes().get_rrs_by_type(6)[0].clone();
            let soa_rdata = match soa_rr.get_rdata() {
                Rdata::SomeSoaRdata(val) => val,
                _ => unreachable!(),
            };

            let soa_minimun_ttl = soa_rdata.get_minimum();

            for rr in rrs_by_type.iter_mut() {
                let rr_ttl = rr.get_ttl();

                rr.set_ttl(cmp::max(rr_ttl, soa_minimun_ttl));
            }
            //

            println!("rrs by type len: {}", rrs_by_type.len());

            msg.set_answer(rrs_by_type);

            let mut header = msg.get_header();

            header.set_aa(true);
            msg.set_header(header);

            return NameServer::step_6(msg, cache, zones_by_class);
        } else {
            let rr = matched_node.get_value()[0].clone();
            if rr.get_type_code() == 5 && qtype != 5 {
                println!("CNAME!!!");

                rrs_by_type.push(rr.clone());
                msg.set_answer(rrs_by_type);

                let mut header = msg.get_header();
                header.set_aa(true);

                msg.set_header(header);

                let canonical_name = match rr.get_rdata() {
                    Rdata::SomeCnameRdata(val) => val.get_cname(),
                    _ => unreachable!(),
                };

                println!("Cname name: {}", canonical_name.get_name());

                let mut question = msg.get_question();

                question.set_qname(canonical_name);
                msg.set_question(question);

                return NameServer::step_2(
                    msg,
                    zones_by_class,
                    cache,
                    tx_delete_resolver_udp,
                    tx_delete_resolver_tcp,
                    tx_delete_ns_udp,
                    tx_delete_ns_tcp,
                );
            } else {
                let mut header = msg.get_header();
                header.set_aa(true);

                msg.set_header(header);
                return NameServer::step_6(msg, cache, zones_by_class);
            }
        }
        //
    }

    /// RFC 1034 step 3b
    /// If a match would take us out of the authoritative data.
    /// Copy the NS RRs for the subzone into the authority
    /// section of the reply.  Put whatever addresses are
    /// available into the additional section, using glue RRs
    /// if the addresses are not available from authoritative
    /// data or the cache.  Go to step 4.
    pub fn step_3b(
        subzone_node: NSNode,
        mut msg: DnsMessage,
        mut cache: DnsCache,
        zones_by_class: HashMap<u16, HashMap<String, NSZone>>,
        tx_delete_resolver_udp: Sender<(String, ResourceRecord)>,
        tx_delete_resolver_tcp: Sender<(String, ResourceRecord)>,
        tx_delete_ns_udp: Sender<(String, ResourceRecord)>,
        tx_delete_ns_tcp: Sender<(String, ResourceRecord)>,
    ) -> DnsMessage {
        let ns_rrs = subzone_node.get_value();

        msg.set_authority(ns_rrs.clone());
        let mut additional = Vec::<ResourceRecord>::new();

        for ns_rr in ns_rrs {
            let name_ns = match ns_rr.get_rdata() {
                Rdata::SomeNsRdata(val) => val.get_nsdname().get_name(),
                _ => unreachable!(),
            };

            let rrs = cache.get(name_ns.clone(), "A".to_string());

            //If there are RR A in cache with the referral host name
            if rrs.len() > 0 {
                for rr in rrs {
                    additional.push(rr.get_resource_record());
                }
            } else {
                println!("Ns name: {}", name_ns.clone());

                match name_ns.find(&subzone_node.get_name()) {
                    Some(index) => {
                        let new_ns_name = name_ns[..index - 1].to_string();
                        let _labels: Vec<&str> = new_ns_name.split(".").collect();
                        let mut a_glue_rrs;

                        // Gets the rrs from the zone
                        let glue_rrs = subzone_node.clone().get_value();

                        // Gets the glue rrs for the ns rr
                        a_glue_rrs = NameServer::look_for_type_records(name_ns, glue_rrs, 1);

                        additional.append(&mut a_glue_rrs);
                    }
                    None => {}
                }
            }
        }

        msg.set_additional(additional);

        return NameServer::step_4(
            msg,
            cache,
            zones_by_class,
            tx_delete_resolver_udp,
            tx_delete_resolver_tcp,
            tx_delete_ns_udp,
            tx_delete_ns_tcp,
        );
    }

    /// RFC 1034 - Step 3c:
    /// If at some label, a match is impossible (i.e., the
    /// corresponding label does not exist), look to see if a
    /// the "*" label exists.
    pub fn step_3c(
        current_node: NSNode,
        mut msg: DnsMessage,
        cache: DnsCache,
        zones_by_class: HashMap<u16, HashMap<String, NSZone>>,
    ) -> DnsMessage {
        let exist = current_node.exist_child("*".to_string());

        if exist == true {
            let (child_node, _available) = current_node.get_child("*".to_string());
            let rrs = child_node.get_value();
            let qtype = msg.get_question().get_qtype();
            let mut answer = Vec::<ResourceRecord>::new();

            for mut rr in rrs {
                if rr.get_type_code() == qtype {
                    rr.set_name(msg.get_question().get_qname());
                    answer.push(rr);
                }
            }

            let mut header = msg.get_header();
            header.set_aa(true);

            if answer.len() == 0 {
                header.set_rcode(2);
            }

            msg.set_answer(answer);

            msg.set_header(header);
            return NameServer::step_6(msg, cache, zones_by_class);
        } else {
            // * label does not exists
            let mut header = msg.get_header();
            let rr = current_node.get_value()[0].clone();
            let qname = msg.get_question().get_qname();

            let canonical_name = match rr.get_rdata() {
                Rdata::SomeCnameRdata(val) => val.get_cname(),
                _ => unreachable!(),
            };

            if qname.get_name() != canonical_name.get_name() {
                header.set_rcode(3);
                if msg.get_answer().len() == 0 {
                    header.set_aa(true);
                }
            }

            msg.set_header(header);

            return msg;
        }
    }

    /// RFC 1034 - Step 4:
    /// Start matching down in the cache.  If QNAME is found in the
    /// cache, copy all RRs attached to it that match QTYPE into the
    /// answer section.  If there was no delegation from
    /// authoritative data, look for the best one from the cache, and
    /// put it in the authority section.  Go to step 6.
    pub fn step_4(
        mut msg: DnsMessage,
        mut cache: DnsCache,
        zones_by_class: HashMap<u16, HashMap<String, NSZone>>,
        tx_delete_resolver_udp: Sender<(String, ResourceRecord)>,
        tx_delete_resolver_tcp: Sender<(String, ResourceRecord)>,
        tx_delete_ns_udp: Sender<(String, ResourceRecord)>,
        tx_delete_ns_tcp: Sender<(String, ResourceRecord)>,
    ) -> DnsMessage {
        let qtype = msg.get_question_qtype();
        let qclass = msg.get_question().get_qclass();
        let mut domain_name = msg.get_question().get_qname().get_name();
        let mut answer = Vec::<ResourceRecord>::new();

        let rrs_by_type = cache.get(domain_name.clone(), qtype);
        let mut rrs = Vec::new();

        // Get the rrs for qname and qclass
        if qclass != 255 {
            // Get rrs for qclass
            for rr in rrs_by_type {
                let rr_class = rr.get_resource_record().get_class();

                if rr_class == qclass {
                    rrs.push(rr);
                }
            }
            //
        } else {
            rrs = rrs_by_type;
        }
        //

        let now = Utc::now();
        let timestamp = now.timestamp() as u32;

        // We check the ttls from the RR's

        for rr_cache in rrs.clone() {
            let mut rr = rr_cache.get_resource_record();
            let rr_ttl = rr.get_ttl();
            let relative_ttl = rr_ttl - timestamp;

            if relative_ttl > 0 {
                rr.set_ttl(relative_ttl);
                answer.push(rr);
            }
        }

        // If there are RR's with TTL < 0, we remove the RR's from the cache
        if rrs.len() > 0 && answer.len() < rrs.len() {
            NameServer::remove_from_cache(
                domain_name.clone(),
                rrs[0].clone().get_resource_record(),
                tx_delete_resolver_udp,
                tx_delete_resolver_tcp,
                tx_delete_ns_udp,
                tx_delete_ns_tcp,
            );
        }

        //

        let mut header = msg.get_header();

        if answer.len() > 0 {
            msg.set_answer(answer);
            header.set_aa(false);
            msg.set_header(header);
        }

        if msg.get_authority().len() > 0 {
            return NameServer::step_6(msg, cache, zones_by_class);
        } else {
            let mut authority = Vec::<ResourceRecord>::new();

            while domain_name != "".to_string() {
                let rrs = cache.get(domain_name.clone(), "NS".to_string());

                if rrs.len() > 0 {
                    for rr in rrs {
                        authority.push(rr.get_resource_record());
                    }

                    msg.set_authority(authority);

                    break;
                } else {
                    let dot_index = domain_name.find(".").unwrap_or(domain_name.len());

                    if dot_index == domain_name.len() {
                        break;
                    } else {
                        domain_name.replace_range(..dot_index + 1, "");
                    }
                }
            }
        }

        return NameServer::step_6(msg, cache, zones_by_class);
    }

    /// step 6 RFC 1034
    /// Using local data only, attempt to add other RRs which may be
    /// useful to the additional section of the query.  Exit.
    fn step_6(
        mut msg: DnsMessage,
        mut cache: DnsCache,
        zones_by_class: HashMap<u16, HashMap<String, NSZone>>,
    ) -> DnsMessage {
        let answers = msg.get_answer();
        let mut additional = msg.get_additional();
        let aa = msg.get_header().get_aa();
        let qclass = msg.get_question().get_qclass();

        for answer in answers {
            let answer_type = answer.get_type_code();

            match answer_type {
                //answer type MX
                15 => {
                    let exchange = match answer.get_rdata() {
                        Rdata::SomeMxRdata(val) => val.get_exchange().get_name(),
                        _ => unreachable!(),
                    };

                    //If answer found is authoritative
                    //add in additional RR type A of top node of the zone
                    if aa == true {
                        let (zone, _available) = NameServer::search_nearest_ancestor_zone(
                            zones_by_class.clone(),
                            exchange,
                            qclass.clone(),
                        );

                        let mut rrs = zone.get_zone_nodes().get_rrs_by_type(1);
                        additional.append(&mut rrs);
                    } else {
                        let rrs = cache.get(exchange, "A".to_string());

                        for rr in rrs {
                            additional.push(rr.get_resource_record());
                        }
                    }
                }

                //answer type NS
                2 => {
                    let name_ns = match answer.get_rdata() {
                        Rdata::SomeNsRdata(val) => val.get_nsdname().get_name(),
                        _ => unreachable!(),
                    };

                    let (zone, _available) = NameServer::search_nearest_ancestor_zone(
                        zones_by_class.clone(),
                        name_ns.clone(),
                        qclass.clone(),
                    );

                    //if zone is a redirection we add rr glue in additional
                    if zone.get_zone_nodes().get_subzone() == true {
                        let glue_rrs = zone.get_glue_rrs();

                        let mut a_glue_rrs =
                            NameServer::look_for_type_records(name_ns, glue_rrs, 1);

                        additional.append(&mut a_glue_rrs);
                    } else {
                        let rrs = cache.get(name_ns, "A".to_string());

                        for rr in rrs {
                            additional.push(rr.get_resource_record());
                        }
                    }
                }
                _ => {}
            }
        }

        msg.set_additional(additional);

        return msg;
    }
}

// Utils for UDP

/*
5. Using the local resolver or a copy of its algorithm to answer the query. Store
the results, including any intermediate CNAMEs, in the answer
section of the response.
*/
impl NameServer {
    fn step_5_udp(
        resolver_ip_and_port: String,
        mut msg: DnsMessage,
        socket: UdpSocket,
        tx: Sender<(Vec<(u16, String)>, u16)>,
        src_address: String,
    ) {
        let old_id = msg.get_query_id();
        let mut rng = thread_rng();
        let new_id: u16 = rng.gen();

        let mut header = msg.get_header();
        header.set_id(new_id);

        msg.set_header(header);

        tx.send((vec![(old_id, src_address)], new_id))
            .expect("Couldn't send the information through the channel");

        // Send request to resolver
        socket
            .send_to(&msg.to_bytes(), resolver_ip_and_port)
            .expect("Couldn't send request to resolver");
    }

    // Sends the response to the address by udp
    fn send_response_by_udp(mut response: DnsMessage, src_address: String, socket: &UdpSocket) {
        response.update_header_counters();
        let bytes = response.to_bytes();

        if bytes.len() <= 512 {
            println!("Enviando mensaje de respuesta: {}", src_address.clone());

            socket
                .send_to(&bytes, src_address)
                .expect("Couldn't send message");
        } else {
            let ancount = response.get_header().get_ancount();
            let nscount = response.get_header().get_nscount();
            let arcount = response.get_header().get_arcount();
            let total_rrs = ancount + nscount + arcount;
            let half_rrs: f32 = (total_rrs / 2).into();
            let ceil_half_rrs = half_rrs.ceil() as u32;

            let mut answer = response.get_answer();
            let mut authority = response.get_authority();
            let mut additional = response.get_additional();

            let mut first_tc_msg = DnsMessage::new();
            let mut new_header = response.get_header();
            new_header.set_tc(true);
            first_tc_msg.set_header(new_header);

            for _i in 1..ceil_half_rrs + 1 {
                if answer.len() > 0 {
                    let rr = answer.remove(0);
                    first_tc_msg.add_answers(vec![rr]);
                } else if authority.len() > 0 {
                    let rr = authority.remove(0);
                    first_tc_msg.add_authorities(vec![rr]);
                } else if additional.len() > 0 {
                    let rr = additional.remove(0);
                    first_tc_msg.add_additionals(vec![rr]);
                } else {
                    continue;
                }
            }

            first_tc_msg.update_header_counters();

            NameServer::send_response_by_udp(
                first_tc_msg,
                src_address.clone(),
                &socket.try_clone().unwrap(),
            );

            let mut second_tc_msg = DnsMessage::new();
            let new_header = response.get_header();
            second_tc_msg.set_header(new_header);

            for _i in 1..ceil_half_rrs + 1 {
                if answer.len() > 0 {
                    let rr = answer.remove(0);
                    second_tc_msg.add_answers(vec![rr]);
                } else if authority.len() > 0 {
                    let rr = authority.remove(0);
                    second_tc_msg.add_authorities(vec![rr]);
                } else if additional.len() > 0 {
                    let rr = additional.remove(0);
                    second_tc_msg.add_additionals(vec![rr]);
                } else {
                    continue;
                }
            }

            second_tc_msg.update_header_counters();

            NameServer::send_response_by_udp(second_tc_msg, src_address, socket);
        }
    }
}

//Utils for TCP
impl NameServer {
    fn step_5_tcp(
        resolver_ip_and_port: String,
        mut msg: DnsMessage,
        cache: DnsCache,
        zones: HashMap<u16, HashMap<String, NSZone>>,
    ) -> DnsMessage {
        let mut rng = thread_rng();
        let new_id: u16 = rng.gen();

        let mut header = msg.get_header();
        header.set_id(new_id);

        msg.set_header(header);

        let bytes = msg.to_bytes();

        // Adds the two bytes needs for tcp
        let msg_length: u16 = bytes.len() as u16;
        let tcp_bytes_length = [(msg_length >> 8) as u8, msg_length as u8];
        let full_msg = [&tcp_bytes_length, bytes.as_slice()].concat();

        // Send query to local resolver
        let mut stream = TcpStream::connect(resolver_ip_and_port).unwrap();
        stream.write(&full_msg).expect("Couldn't write the message");

        let received_msg = Resolver::receive_tcp_msg(stream).unwrap();

        let dns_response_result = DnsMessage::from_bytes(&received_msg);

        match dns_response_result {
            Ok(_) => {}
            Err(_) => {
                return DnsMessage::format_error_msg();
            }
        }

        let dns_response = dns_response_result.unwrap();

        return NameServer::step_6(dns_response, cache, zones);
    }

    fn send_response_by_tcp(mut msg: DnsMessage, _address: String, mut stream: TcpStream) {
        msg.update_header_counters();

        let bytes = msg.to_bytes();
        let msg_length: u16 = bytes.len() as u16;
        let tcp_bytes_length = [(msg_length >> 8) as u8, msg_length as u8];
        let full_msg = [&tcp_bytes_length, bytes.as_slice()].concat();

        stream.write(&full_msg).expect("Couldn't write the message");
    }

    //
    fn send_axfr_response(
        mut msg: DnsMessage,
        address: String,
        zones: HashMap<u16, HashMap<String, NSZone>>,
        stream: TcpStream,
    ) {
        // Get the zone for the qname and qclass
        let zone_class = msg.get_question().get_qclass();
        let zone_name = msg.get_question().get_qname().get_name();

        let zone_by_class = zones.get(&zone_class).unwrap();
        let zone = zone_by_class.get(&zone_name).unwrap();

        // Create response msg
        let mut header = msg.get_header();
        let mut answers = msg.get_answer();

        // Set headers bits
        header.set_qr(true);
        //

        // Add Soa rr first
        let soa_rr = zone.get_zone_nodes().get_rrs_by_type(6)[0].clone();

        answers.push(soa_rr.clone());
        //

        // Add others RRs from top node to the answer
        let rrs = zone.get_zone_nodes().get_value();

        for rr in rrs {
            if rr.get_type_code() != 6 {
                answers.push(rr);
            }
        }
        //

        // Add rrs from the children
        let children = zone.get_zone_nodes().get_children();

        for child in children {
            let mut rrs_from_child = child.get_all_rrs();
            answers.append(&mut rrs_from_child);
        }
        //

        // Add Soa rr last
        answers.push(soa_rr);
        //

        if answers.len() == 0 {
            header.set_rcode(5);
        }

        msg.set_answer(answers);

        msg.set_header(header);

        NameServer::send_response_by_tcp(msg, address, stream);
    }
}

// Utils
impl NameServer {
    fn look_for_type_records(
        name_ns: String,
        rrs: Vec<ResourceRecord>,
        rr_type: u16,
    ) -> Vec<ResourceRecord> {
        let mut a_rrs = Vec::<ResourceRecord>::new();

        for rr in rrs {
            let rr_type_glue = rr.get_type_code();
            let rr_name = rr.get_name().get_name();

            if rr_type_glue == rr_type && rr_name == name_ns {
                a_rrs.push(rr);
            }
        }

        return a_rrs;
    }

    fn set_ra(mut msg: DnsMessage, ra: bool) -> DnsMessage {
        let mut header = msg.get_header();
        header.set_ra(ra);

        msg.set_header(header);

        msg
    }

    pub fn add_zone_from_master_file(
        &mut self,
        file_name: String,
        origin: String,
        ip_address_for_refresh: String,
        validity_check: bool,
    ) {
        let new_zone = NSZone::from_file(file_name, origin, ip_address_for_refresh, validity_check);

        let mut zones_by_class = self.get_zones_by_class();
        let zone_class = new_zone.get_class();

        // Create the new zone hash
        let mut new_zone_hash = HashMap::<String, NSZone>::new();
        new_zone_hash.insert(new_zone.get_name(), new_zone);

        // Insert the new zone by class
        zones_by_class.insert(zone_class, new_zone_hash);

        self.set_zones_by_class(zones_by_class);
    }

    fn remove_from_cache(
        domain_name: String,
        resource_record: ResourceRecord,
        tx_resolver_udp: Sender<(String, ResourceRecord)>,
        tx_resolver_tcp: Sender<(String, ResourceRecord)>,
        tx_ns_udp: Sender<(String, ResourceRecord)>,
        tx_ns_tcp: Sender<(String, ResourceRecord)>,
    ) {
        tx_resolver_udp
            .send((domain_name.clone(), resource_record.clone()))
            .expect("Couldn't send domain name and RR while removing from cache");
        tx_resolver_tcp
            .send((domain_name.clone(), resource_record.clone()))
            .expect("Couldn't send domain name and RR while removing from cache");
        tx_ns_udp
            .send((domain_name.clone(), resource_record.clone()))
            .expect("Couldn't send domain name and RR while removing from cache");
        tx_ns_tcp
            .send((domain_name.clone(), resource_record.clone()))
            .expect("Couldn't send domain name and RR while removing from cache");
    }
}

// Getters
impl NameServer {
    /// Gets the zones data from the name server
    pub fn get_zones_by_class(&self) -> HashMap<u16, HashMap<String, NSZone>> {
        self.zones_by_class.clone()
    }

    /// Gets the cache from the name server
    pub fn get_cache(&self) -> DnsCache {
        self.cache.clone()
    }

    /// Gets if the server is primary or not
    pub fn get_primary_server(&self) -> bool {
        self.primary_server
    }

    /// Gets the ip address to ask for refresh a zone
    pub fn get_queries_id_for_soa_rr(&self) -> HashMap<u16, String> {
        self.queries_id_for_soa_rr.clone()
    }

    pub fn get_queries_id(&self) -> HashMap<u16, Vec<(u16, String)>> {
        self.queries_id.clone()
    }

    pub fn get_refresh_zones_data(&self) -> HashMap<String, ZoneRefresh> {
        self.refresh_zones_data.clone()
    }

    /// Get the owner's query address
    pub fn get_delete_channel_udp(&self) -> Sender<(String, ResourceRecord)> {
        self.delete_sender_udp.clone()
    }

    /// Get the owner's query address
    pub fn get_delete_channel_tcp(&self) -> Sender<(String, ResourceRecord)> {
        self.delete_sender_tcp.clone()
    }

    /// Get the owner's query address
    pub fn get_add_channel_ns_udp(&self) -> Sender<(String, ResourceRecord)> {
        self.add_sender_ns_udp.clone()
    }

    /// Get the owner's query address
    pub fn get_add_channel_ns_tcp(&self) -> Sender<(String, ResourceRecord)> {
        self.add_sender_ns_tcp.clone()
    }

    /// Get the owner's query address
    pub fn get_delete_channel_ns_udp(&self) -> Sender<(String, ResourceRecord)> {
        self.delete_sender_ns_udp.clone()
    }

    /// Get the owner's query address
    pub fn get_delete_channel_ns_tcp(&self) -> Sender<(String, ResourceRecord)> {
        self.delete_sender_ns_tcp.clone()
    }

    /// Gets the sender to update a refresh zone in name server udp
    pub fn get_update_refresh_zone_tcp(&self) -> Sender<ZoneRefresh> {
        self.update_refresh_zone_tcp.clone()
    }

    /// Gets the sender to update a refresh zone in name server tcp
    pub fn get_update_refresh_zone_udp(&self) -> Sender<ZoneRefresh> {
        self.update_refresh_zone_udp.clone()
    }

    /// Gets the sender to update a zone in the udp resolver
    pub fn get_update_zone_udp_resolver(&self) -> Sender<NSZone> {
        self.update_zone_udp_resolver.clone()
    }

    /// Gets the sender to update a zone in the tcp resolver
    pub fn get_update_zone_tcp_resolver(&self) -> Sender<NSZone> {
        self.update_zone_tcp_resolver.clone()
    }
}

// Setters
impl NameServer {
    /// Sets the zones with a new value
    pub fn set_zones_by_class(&mut self, zones_by_class: HashMap<u16, HashMap<String, NSZone>>) {
        self.zones_by_class = zones_by_class;
    }

    /// Sets the cache with a new cache
    pub fn set_cache(&mut self, cache: DnsCache) {
        self.cache = cache;
    }

    /// Sets the primary server with a new value
    pub fn set_primary_server(&mut self, primary_server: bool) {
        self.primary_server = primary_server;
    }

    /// Sets the queries ids with a new value
    pub fn set_queries_id(&mut self, queries_id: HashMap<u16, Vec<(u16, String)>>) {
        self.queries_id = queries_id;
    }

    /// Sets the queries ids for soa queries with a new value
    pub fn set_queries_id_for_soa_rr(&mut self, queries_id: HashMap<u16, String>) {
        self.queries_id_for_soa_rr = queries_id;
    }

    pub fn set_refresh_zones_data(&mut self, refresh_data: HashMap<String, ZoneRefresh>) {
        self.refresh_zones_data = refresh_data;
    }
}

#[cfg(test)]

mod name_server_test{
    use std::sync::mpsc;
    use crate::name_server::HashMap;
    use chrono::Utc;

    use super::NameServer;
    use crate::name_server::zone::NSZone;
    use crate::name_server::ResourceRecord;
    use crate::name_server::Rdata;
    use crate::message::rdata::a_rdata::ARdata;
    use crate::name_server::DnsCache;
    use crate::name_server::ZoneRefresh;
    use crate::domain_name::DomainName;
    use crate::message::rdata::soa_rdata::SoaRdata;
    use crate::name_server::NSNode;


    //ToDo: Revisar Práctica 1
    #[test]
    fn constructor_test(){
        let (delete_sender_udp, _delete_recv_udp) = mpsc::channel();
        let (delete_sender_tcp, _delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, _add_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, _add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_udp, _delete_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_tcp, _delete_recv_ns_tcp) = mpsc::channel();
        let (update_refresh_zone_udp, _rx_update_refresh_zone_udp) = mpsc::channel();
        let (update_refresh_zone_tcp, _rx_update_refresh_zone_tcp) = mpsc::channel();
        let (update_zone_udp_resolver, _tx_update_zone_udp_resolver) = mpsc::channel();
        let (update_zone_tcp_resolver, _tx_update_zone_tcp_resolver) = mpsc::channel();

        let name_server = NameServer::new(
            true,
            delete_sender_udp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp, 
            add_sender_ns_tcp, 
            delete_sender_ns_tcp, 
            update_refresh_zone_udp,
            update_refresh_zone_tcp,
            update_zone_udp_resolver,
            update_zone_tcp_resolver,
        );

        assert_eq!(name_server.cache.get_size(), 0);
        assert_eq!(name_server.zones_by_class.len(), 0);
        assert_eq!(name_server.queries_id.len(), 0);
        assert!(name_server.primary_server);
        assert_eq!(name_server.queries_id_for_soa_rr.len(), 0);
        assert_eq!(name_server.refresh_zones_data.len(), 0);
    }

    //ToDo:Revisar Práctica 1
    #[test]
    fn set_and_get_zones_by_class(){
        let (delete_sender_udp, _delete_recv_udp) = mpsc::channel();
        let (delete_sender_tcp, _delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, _add_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, _add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_udp, _delete_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_tcp, _delete_recv_ns_tcp) = mpsc::channel();
        let (update_refresh_zone_udp, _rx_update_refresh_zone_udp) = mpsc::channel();
        let (update_refresh_zone_tcp, _rx_update_refresh_zone_tcp) = mpsc::channel();
        let (update_zone_udp_resolver, _tx_update_zone_udp_resolver) = mpsc::channel();
        let (update_zone_tcp_resolver, _tx_update_zone_tcp_resolver) = mpsc::channel();

        let mut name_server = NameServer::new(
            true,
            delete_sender_udp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp, 
            add_sender_ns_tcp, 
            delete_sender_ns_tcp, 
            update_refresh_zone_udp,
            update_refresh_zone_tcp,
            update_zone_udp_resolver,
            update_zone_tcp_resolver,
        );

        let file_name = "test.txt".to_string();
        let origin = "example".to_string();
        let ip = "192.80.24.11".to_string();
        let nszone = NSZone::from_file(file_name, origin, ip, true);

        let mut hash_string_and_nszone = HashMap::<String, NSZone>::new();

        hash_string_and_nszone.insert("test.com".to_string(), nszone);

        let mut new_ns_data = HashMap::<u16, HashMap<String, NSZone>>::new();

        new_ns_data.insert(2, hash_string_and_nszone);

        assert_eq!(name_server.get_zones_by_class().len(), 0);

        name_server.set_zones_by_class(new_ns_data);

        let res = name_server.get_zones_by_class();

        assert_eq!(res.len(), 1);
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn set_and_get_cache(){
        let (delete_sender_udp, _delete_recv_udp) = mpsc::channel();
        let (delete_sender_tcp, _delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, _add_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, _add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_udp, _delete_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_tcp, _delete_recv_ns_tcp) = mpsc::channel();
        let (update_refresh_zone_udp, _rx_update_refresh_zone_udp) = mpsc::channel();
        let (update_refresh_zone_tcp, _rx_update_refresh_zone_tcp) = mpsc::channel();
        let (update_zone_udp_resolver, _tx_update_zone_udp_resolver) = mpsc::channel();
        let (update_zone_tcp_resolver, _tx_update_zone_tcp_resolver) = mpsc::channel();

        let mut name_server = NameServer::new(
            true,
            delete_sender_udp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp, 
            add_sender_ns_tcp, 
            delete_sender_ns_tcp, 
            update_refresh_zone_udp,
            update_refresh_zone_tcp,
            update_zone_udp_resolver,
            update_zone_tcp_resolver,
        );

        let mut cache_test = DnsCache::new();
        let ip_address: [u8; 4] = [127, 0, 0, 0];
        let mut a_rdata = ARdata::new();

        cache_test.set_max_size(1);

        a_rdata.set_address(ip_address);

        let rdata = Rdata::SomeARdata(a_rdata);
        let mut resource_record = ResourceRecord::new(rdata);
        resource_record.set_type_code(1);

        cache_test.add("127.0.0.0".to_string(), resource_record);

        name_server.set_cache(cache_test);

        assert_eq!(name_server.get_cache().get_size(), 1);
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn set_and_get_primary_server(){
        let (delete_sender_udp, _delete_recv_udp) = mpsc::channel();
        let (delete_sender_tcp, _delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, _add_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, _add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_udp, _delete_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_tcp, _delete_recv_ns_tcp) = mpsc::channel();
        let (update_refresh_zone_udp, _rx_update_refresh_zone_udp) = mpsc::channel();
        let (update_refresh_zone_tcp, _rx_update_refresh_zone_tcp) = mpsc::channel();
        let (update_zone_udp_resolver, _tx_update_zone_udp_resolver) = mpsc::channel();
        let (update_zone_tcp_resolver, _tx_update_zone_tcp_resolver) = mpsc::channel();

        let mut name_server = NameServer::new(
            false,
            delete_sender_udp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp, 
            add_sender_ns_tcp, 
            delete_sender_ns_tcp, 
            update_refresh_zone_udp,
            update_refresh_zone_tcp,
            update_zone_udp_resolver,
            update_zone_tcp_resolver,
        );

        assert!(!name_server.get_primary_server());

        name_server.set_primary_server(true);

        assert!(name_server.get_primary_server());
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn set_and_get_queries_id_for_soa_rr(){
        let (delete_sender_udp, _delete_recv_udp) = mpsc::channel();
        let (delete_sender_tcp, _delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, _add_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, _add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_udp, _delete_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_tcp, _delete_recv_ns_tcp) = mpsc::channel();
        let (update_refresh_zone_udp, _rx_update_refresh_zone_udp) = mpsc::channel();
        let (update_refresh_zone_tcp, _rx_update_refresh_zone_tcp) = mpsc::channel();
        let (update_zone_udp_resolver, _tx_update_zone_udp_resolver) = mpsc::channel();
        let (update_zone_tcp_resolver, _tx_update_zone_tcp_resolver) = mpsc::channel();

        let mut name_server = NameServer::new(
            true,
            delete_sender_udp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp, 
            add_sender_ns_tcp, 
            delete_sender_ns_tcp, 
            update_refresh_zone_udp,
            update_refresh_zone_tcp,
            update_zone_udp_resolver,
            update_zone_tcp_resolver,
        );

        let mut new_queries_id_soa_rr = HashMap::<u16, String>::new();
        new_queries_id_soa_rr.insert(1, String::from("test.com"));

        name_server.set_queries_id_for_soa_rr(new_queries_id_soa_rr);

        let res = name_server.get_queries_id_for_soa_rr();

        assert_eq!(res.len(), 1);
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn set_and_get_queries_id(){
        let (delete_sender_udp, _delete_recv_udp) = mpsc::channel();
        let (delete_sender_tcp, _delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, _add_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, _add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_udp, _delete_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_tcp, _delete_recv_ns_tcp) = mpsc::channel();
        let (update_refresh_zone_udp, _rx_update_refresh_zone_udp) = mpsc::channel();
        let (update_refresh_zone_tcp, _rx_update_refresh_zone_tcp) = mpsc::channel();
        let (update_zone_udp_resolver, _tx_update_zone_udp_resolver) = mpsc::channel();
        let (update_zone_tcp_resolver, _tx_update_zone_tcp_resolver) = mpsc::channel();

        let mut name_server = NameServer::new(
            true,
            delete_sender_udp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp, 
            add_sender_ns_tcp, 
            delete_sender_ns_tcp, 
            update_refresh_zone_udp,
            update_refresh_zone_tcp,
            update_zone_udp_resolver,
            update_zone_tcp_resolver,
        );

        let name = String::from("test.com");
        let mut new_queries_id_vec = Vec::<(u16, String)>::new();
        new_queries_id_vec.insert(0, (2, name));
        let mut new_queries_id = HashMap::<u16, Vec<(u16, String)>>::new();
        new_queries_id.insert(1, new_queries_id_vec.clone());

        assert_eq!(name_server.get_queries_id().len(), 0);

        name_server.set_queries_id(new_queries_id);

        let res = name_server.get_queries_id().clone();
        assert_eq!(res.len(), 1);
        assert!(res.contains_key(&1));
        assert!(!res.contains_key(&2));
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn set_and_get_refresh_zone_data(){
        let (delete_sender_udp, _delete_recv_udp) = mpsc::channel();
        let (delete_sender_tcp, _delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, _add_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, _add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_udp, _delete_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_tcp, _delete_recv_ns_tcp) = mpsc::channel();
        let (update_refresh_zone_udp, _rx_update_refresh_zone_udp) = mpsc::channel();
        let (update_refresh_zone_tcp, _rx_update_refresh_zone_tcp) = mpsc::channel();
        let (update_zone_udp_resolver, _tx_update_zone_udp_resolver) = mpsc::channel();
        let (update_zone_tcp_resolver, _tx_update_zone_tcp_resolver) = mpsc::channel();

        let mut name_server = NameServer::new(
            true,
            delete_sender_udp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp, 
            add_sender_ns_tcp, 
            delete_sender_ns_tcp, 
            update_refresh_zone_udp,
            update_refresh_zone_tcp,
            update_zone_udp_resolver,
            update_zone_tcp_resolver,
        );

        let mut ns_zone = NSZone::new();
        let name = String::from("mail.example.com");
        ns_zone.set_name(name);
        let ip = String::from("200.89.76.36");
        ns_zone.set_ip_address_for_refresh_zone(ip);

        let mut value = Vec::<ResourceRecord>::new();
        let mut soa_rdata = Rdata::SomeSoaRdata(SoaRdata::new());
        let mut mname_domain_name = DomainName::new();
        let domain_name_name = String::from("ns.primaryserver.com");
        mname_domain_name.set_name(domain_name_name);
        let mut rname_domain_name = DomainName::new();
        let rname_name = String::from("admin.mail.example.com");
        rname_domain_name.set_name(rname_name);
        match soa_rdata {
            Rdata::SomeSoaRdata(ref mut val) => {
                val.set_mname(mname_domain_name);
                val.set_rname(rname_domain_name);
                val.set_serial(1111111111 as u32)
            }
            _ => unreachable!(),
        }
        let resource_record = ResourceRecord::new(soa_rdata);
        value.push(resource_record);

        let mut top_node = ns_zone.get_zone_nodes();
        top_node.set_value(value);
        ns_zone.set_zone_nodes(top_node);

        let zone_refresh = ZoneRefresh::new(ns_zone);

        let mut new_refresh_zone_data = HashMap::<String, ZoneRefresh>::new();

        new_refresh_zone_data.insert(String::from("example.com"), zone_refresh);

        assert_eq!(name_server.get_refresh_zones_data().len(), 0);

        name_server.set_refresh_zones_data(new_refresh_zone_data);
        let res = name_server.get_refresh_zones_data();

        assert_eq!(res.len(), 1);
        assert!(res.contains_key(&String::from("example.com")));
    }

    //ToDo: Revisar Prácitca 1
    #[test]
    fn get_delete_channel_udp(){
        let (delete_sender_udp, _delete_recv_udp) = mpsc::channel();
        let (delete_sender_tcp, _delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, _add_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, _add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_udp, _delete_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_tcp, _delete_recv_ns_tcp) = mpsc::channel();
        let (update_refresh_zone_udp, _rx_update_refresh_zone_udp) = mpsc::channel();
        let (update_refresh_zone_tcp, _rx_update_refresh_zone_tcp) = mpsc::channel();
        let (update_zone_udp_resolver, _tx_update_zone_udp_resolver) = mpsc::channel();
        let (update_zone_tcp_resolver, _tx_update_zone_tcp_resolver) = mpsc::channel();

        let name_server = NameServer::new(
            true,
            delete_sender_udp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp, 
            add_sender_ns_tcp, 
            delete_sender_ns_tcp, 
            update_refresh_zone_udp,
            update_refresh_zone_tcp,
            update_zone_udp_resolver,
            update_zone_tcp_resolver,
        );

        let delete_channel_udp_test = name_server.get_delete_channel_udp();
        let delete_rcv_udp_ = _delete_recv_udp;
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let rr = ResourceRecord::new(a_rdata);
        let msg = (String::from("test"), rr.clone());

        delete_channel_udp_test.send(msg).unwrap();
        let (name, rr_result) = delete_rcv_udp_.recv().unwrap();

        /*if the message was correctly sent it should work with the variable
        created with the get fn used*/ 
        assert_eq!(name, String::from("test"));
        assert_eq!(rr_result.get_name(), rr.clone().get_name());
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn get_delete_channel_tcp(){
        let (delete_sender_udp, _delete_recv_udp) = mpsc::channel();
        let (delete_sender_tcp, _delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, _add_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, _add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_udp, _delete_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_tcp, _delete_recv_ns_tcp) = mpsc::channel();
        let (update_refresh_zone_udp, _rx_update_refresh_zone_udp) = mpsc::channel();
        let (update_refresh_zone_tcp, _rx_update_refresh_zone_tcp) = mpsc::channel();
        let (update_zone_udp_resolver, _tx_update_zone_udp_resolver) = mpsc::channel();
        let (update_zone_tcp_resolver, _tx_update_zone_tcp_resolver) = mpsc::channel();

        let name_server = NameServer::new(
            true,
            delete_sender_udp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp, 
            add_sender_ns_tcp, 
            delete_sender_ns_tcp, 
            update_refresh_zone_udp,
            update_refresh_zone_tcp,
            update_zone_udp_resolver,
            update_zone_tcp_resolver,
        );

        let delete_channel_tcp_test = name_server.get_delete_channel_tcp();
        let delete_rcv_tcp_ = _delete_recv_tcp;
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let rr = ResourceRecord::new(a_rdata);
        let msg = (String::from("test"), rr.clone());

        delete_channel_tcp_test.send(msg).unwrap();
        let (name, rr_result) = delete_rcv_tcp_.recv().unwrap();

        /*if the message was correctly sent it should work with the variable
        created with the get fn used*/ 
        assert_eq!(name, String::from("test"));
        assert_eq!(rr_result.get_name(), rr.clone().get_name());
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn get_delete_channel_ns_udp(){
        let (delete_sender_udp, _delete_recv_udp) = mpsc::channel();
        let (delete_sender_tcp, _delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, _add_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, _add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_udp, _delete_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_tcp, _delete_recv_ns_tcp) = mpsc::channel();
        let (update_refresh_zone_udp, _rx_update_refresh_zone_udp) = mpsc::channel();
        let (update_refresh_zone_tcp, _rx_update_refresh_zone_tcp) = mpsc::channel();
        let (update_zone_udp_resolver, _tx_update_zone_udp_resolver) = mpsc::channel();
        let (update_zone_tcp_resolver, _tx_update_zone_tcp_resolver) = mpsc::channel();

        let name_server = NameServer::new(
            true,
            delete_sender_udp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp, 
            add_sender_ns_tcp, 
            delete_sender_ns_tcp, 
            update_refresh_zone_udp,
            update_refresh_zone_tcp,
            update_zone_udp_resolver,
            update_zone_tcp_resolver,
        );

        let delete_channel_ns_udp_test = name_server.get_delete_channel_ns_udp();
        let delete_rcv_ns_udp_ = _delete_recv_ns_udp;
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let rr = ResourceRecord::new(a_rdata);
        let msg = (String::from("test"), rr.clone());

        delete_channel_ns_udp_test.send(msg).unwrap();
        let (name, rr_result) = delete_rcv_ns_udp_.recv().unwrap();

        /*if the message was correctly sent it should work with the variable
        created with the get fn used*/ 
        assert_eq!(name, String::from("test"));
        assert_eq!(rr_result.get_name(), rr.clone().get_name());
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn get_delete_channel_ns_tcp(){
        let (delete_sender_udp, _delete_recv_udp) = mpsc::channel();
        let (delete_sender_tcp, _delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, _add_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, _add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_udp, _delete_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_tcp, _delete_recv_ns_tcp) = mpsc::channel();
        let (update_refresh_zone_udp, _rx_update_refresh_zone_udp) = mpsc::channel();
        let (update_refresh_zone_tcp, _rx_update_refresh_zone_tcp) = mpsc::channel();
        let (update_zone_udp_resolver, _tx_update_zone_udp_resolver) = mpsc::channel();
        let (update_zone_tcp_resolver, _tx_update_zone_tcp_resolver) = mpsc::channel();

        let name_server = NameServer::new(
            true,
            delete_sender_udp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp, 
            add_sender_ns_tcp, 
            delete_sender_ns_tcp, 
            update_refresh_zone_udp,
            update_refresh_zone_tcp,
            update_zone_udp_resolver,
            update_zone_tcp_resolver,
        );

        let delete_channel_ns_tcp_test = name_server.get_delete_channel_ns_tcp();
        let delete_rcv_ns_tcp_ = _delete_recv_ns_tcp;
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let rr = ResourceRecord::new(a_rdata);
        let msg = (String::from("test"), rr.clone());

        delete_channel_ns_tcp_test.send(msg).unwrap();
        let (name, rr_result) = delete_rcv_ns_tcp_.recv().unwrap();

        /*if the message was correctly sent it should work with the variable
        created with the get fn used*/ 
        assert_eq!(name, String::from("test"));
        assert_eq!(rr_result.get_name(), rr.clone().get_name());
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn get_add_channel_ns_udp(){
        let (delete_sender_udp, _delete_recv_udp) = mpsc::channel();
        let (delete_sender_tcp, _delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, _add_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, _add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_udp, _delete_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_tcp, _delete_recv_ns_tcp) = mpsc::channel();
        let (update_refresh_zone_udp, _rx_update_refresh_zone_udp) = mpsc::channel();
        let (update_refresh_zone_tcp, _rx_update_refresh_zone_tcp) = mpsc::channel();
        let (update_zone_udp_resolver, _tx_update_zone_udp_resolver) = mpsc::channel();
        let (update_zone_tcp_resolver, _tx_update_zone_tcp_resolver) = mpsc::channel();

        let name_server = NameServer::new(
            true,
            delete_sender_udp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp, 
            add_sender_ns_tcp, 
            delete_sender_ns_tcp, 
            update_refresh_zone_udp,
            update_refresh_zone_tcp,
            update_zone_udp_resolver,
            update_zone_tcp_resolver,
        );

        let add_channel_ns_udp_test = name_server.get_add_channel_ns_udp();
        let add_rcv_ns_udp_ = _add_recv_ns_udp;
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let rr = ResourceRecord::new(a_rdata);
        let msg = (String::from("test"), rr.clone());

        add_channel_ns_udp_test.send(msg).unwrap();
        let (name, rr_result) = add_rcv_ns_udp_.recv().unwrap();

        /*if the message was correctly sent it should work with the variable
        created with the get fn used*/ 
        assert_eq!(name, String::from("test"));
        assert_eq!(rr_result.get_name(), rr.clone().get_name());
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn get_add_channel_ns_tcp(){
        let (delete_sender_udp, _delete_recv_udp) = mpsc::channel();
        let (delete_sender_tcp, _delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, _add_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, _add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_udp, _delete_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_tcp, _delete_recv_ns_tcp) = mpsc::channel();
        let (update_refresh_zone_udp, _rx_update_refresh_zone_udp) = mpsc::channel();
        let (update_refresh_zone_tcp, _rx_update_refresh_zone_tcp) = mpsc::channel();
        let (update_zone_udp_resolver, _tx_update_zone_udp_resolver) = mpsc::channel();
        let (update_zone_tcp_resolver, _tx_update_zone_tcp_resolver) = mpsc::channel();

        let name_server = NameServer::new(
            true,
            delete_sender_udp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp, 
            add_sender_ns_tcp, 
            delete_sender_ns_tcp, 
            update_refresh_zone_udp,
            update_refresh_zone_tcp,
            update_zone_udp_resolver,
            update_zone_tcp_resolver,
        );

        let add_channel_ns_tcp_test = name_server.get_add_channel_ns_tcp();
        let add_rcv_ns_tcp_ = _add_recv_ns_tcp;
        let a_rdata = Rdata::SomeARdata(ARdata::new());
        let rr = ResourceRecord::new(a_rdata);
        let msg = (String::from("test"), rr.clone());

        add_channel_ns_tcp_test.send(msg).unwrap();
        let (name, rr_result) = add_rcv_ns_tcp_.recv().unwrap();

        /*if the message was correctly sent it should work with the variable
        created with the get fn used*/ 
        assert_eq!(name, String::from("test"));
        assert_eq!(rr_result.get_name(), rr.clone().get_name());
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn get_update_refresh_zone_udp(){
        let (delete_sender_udp, _delete_recv_udp) = mpsc::channel();
        let (delete_sender_tcp, _delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, _add_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, _add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_udp, _delete_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_tcp, _delete_recv_ns_tcp) = mpsc::channel();
        let (update_refresh_zone_udp, _rx_update_refresh_zone_udp) = mpsc::channel();
        let (update_refresh_zone_tcp, _rx_update_refresh_zone_tcp) = mpsc::channel();
        let (update_zone_udp_resolver, _tx_update_zone_udp_resolver) = mpsc::channel();
        let (update_zone_tcp_resolver, _tx_update_zone_tcp_resolver) = mpsc::channel();

        let name_server = NameServer::new(
            true,
            delete_sender_udp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp, 
            add_sender_ns_tcp, 
            delete_sender_ns_tcp, 
            update_refresh_zone_udp,
            update_refresh_zone_tcp,
            update_zone_udp_resolver,
            update_zone_tcp_resolver,
        );

        let mut ns_zone = NSZone::new();
        let name = String::from("mail.example.com");
        ns_zone.set_name(name);
        let ip = String::from("200.89.76.36");
        ns_zone.set_ip_address_for_refresh_zone(ip);

        let mut value = Vec::<ResourceRecord>::new();
        let mut soa_rdata = Rdata::SomeSoaRdata(SoaRdata::new());
        let mut mname_domain_name = DomainName::new();
        let domain_name_name = String::from("ns.primaryserver.com");
        mname_domain_name.set_name(domain_name_name);
        let mut rname_domain_name = DomainName::new();
        let rname_name = String::from("admin.mail.example.com");
        rname_domain_name.set_name(rname_name);
        match soa_rdata {
            Rdata::SomeSoaRdata(ref mut val) => {
                val.set_mname(mname_domain_name);
                val.set_rname(rname_domain_name);
                val.set_serial(1111111111 as u32)
            }
            _ => unreachable!(),
        }
        let resource_record = ResourceRecord::new(soa_rdata);
        value.push(resource_record);

        let mut top_node = ns_zone.get_zone_nodes();
        top_node.set_value(value);
        ns_zone.set_zone_nodes(top_node);

        let zone_refresh = ZoneRefresh::new(ns_zone);

        let update_refresh_zone_udp_test = name_server.get_update_refresh_zone_udp();
        let update_refresh_zone_rcv_udp = _rx_update_refresh_zone_udp;

        update_refresh_zone_udp_test.send(zone_refresh).unwrap();
        let res_zone_refresh = update_refresh_zone_rcv_udp.recv().unwrap();

        let some_timestamp = Utc::now().timestamp() as u32;
        let expected_name = String::from("mail.example.com");
        assert_eq!(res_zone_refresh.get_zone().get_name(), expected_name);
        let expected_ip = String::from("200.89.76.36");

        assert_eq!(res_zone_refresh.get_ip_address_for_refresh_zone(), expected_ip);
        assert_eq!(res_zone_refresh.get_serial(), 1111111111 as u32);
        assert_eq!(res_zone_refresh.get_refresh(), 0 as u32);
        assert_eq!(res_zone_refresh.get_retry(), 0 as u32);
        assert_eq!(res_zone_refresh.get_expire(), 0 as u32);
        assert_eq!(res_zone_refresh.get_timestamp(), some_timestamp);
        assert_eq!(res_zone_refresh.get_last_fails(), false);
        assert_eq!(res_zone_refresh.get_last_serial_check(), some_timestamp);
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn get_update_refresh_zone_tcp(){
        let (delete_sender_udp, _delete_recv_udp) = mpsc::channel();
        let (delete_sender_tcp, _delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, _add_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, _add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_udp, _delete_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_tcp, _delete_recv_ns_tcp) = mpsc::channel();
        let (update_refresh_zone_udp, _rx_update_refresh_zone_udp) = mpsc::channel();
        let (update_refresh_zone_tcp, _rx_update_refresh_zone_tcp) = mpsc::channel();
        let (update_zone_udp_resolver, _tx_update_zone_udp_resolver) = mpsc::channel();
        let (update_zone_tcp_resolver, _tx_update_zone_tcp_resolver) = mpsc::channel();

        let name_server = NameServer::new(
            true,
            delete_sender_udp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp, 
            add_sender_ns_tcp, 
            delete_sender_ns_tcp, 
            update_refresh_zone_udp,
            update_refresh_zone_tcp,
            update_zone_udp_resolver,
            update_zone_tcp_resolver,
        );

        let mut ns_zone = NSZone::new();
        let name = String::from("mail.example.com");
        ns_zone.set_name(name);
        let ip = String::from("200.89.76.36");
        ns_zone.set_ip_address_for_refresh_zone(ip);

        let mut value = Vec::<ResourceRecord>::new();
        let mut soa_rdata = Rdata::SomeSoaRdata(SoaRdata::new());
        let mut mname_domain_name = DomainName::new();
        let domain_name_name = String::from("ns.primaryserver.com");
        mname_domain_name.set_name(domain_name_name);
        let mut rname_domain_name = DomainName::new();
        let rname_name = String::from("admin.mail.example.com");
        rname_domain_name.set_name(rname_name);
        match soa_rdata {
            Rdata::SomeSoaRdata(ref mut val) => {
                val.set_mname(mname_domain_name);
                val.set_rname(rname_domain_name);
                val.set_serial(1111111111 as u32)
            }
            _ => unreachable!(),
        }
        let resource_record = ResourceRecord::new(soa_rdata);
        value.push(resource_record);

        let mut top_node = ns_zone.get_zone_nodes();
        top_node.set_value(value);
        ns_zone.set_zone_nodes(top_node);

        let zone_refresh = ZoneRefresh::new(ns_zone);

        let update_refresh_zone_tcp_test = name_server.get_update_refresh_zone_tcp();
        let update_refresh_zone_rcv_tcp = _rx_update_refresh_zone_tcp;

        update_refresh_zone_tcp_test.send(zone_refresh).unwrap();
        let res_zone_refresh = update_refresh_zone_rcv_tcp.recv().unwrap();

        let some_timestamp = Utc::now().timestamp() as u32;
        let expected_name = String::from("mail.example.com");
        assert_eq!(res_zone_refresh.get_zone().get_name(), expected_name);
        let expected_ip = String::from("200.89.76.36");

        assert_eq!(res_zone_refresh.get_ip_address_for_refresh_zone(), expected_ip);
        assert_eq!(res_zone_refresh.get_serial(), 1111111111 as u32);
        assert_eq!(res_zone_refresh.get_refresh(), 0 as u32);
        assert_eq!(res_zone_refresh.get_retry(), 0 as u32);
        assert_eq!(res_zone_refresh.get_expire(), 0 as u32);
        assert_eq!(res_zone_refresh.get_timestamp(), some_timestamp);
        assert_eq!(res_zone_refresh.get_last_fails(), false);
        assert_eq!(res_zone_refresh.get_last_serial_check(), some_timestamp);
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn get_update_zone_udp_resolver(){
        let (delete_sender_udp, _delete_recv_udp) = mpsc::channel();
        let (delete_sender_tcp, _delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, _add_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, _add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_udp, _delete_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_tcp, _delete_recv_ns_tcp) = mpsc::channel();
        let (update_refresh_zone_udp, _rx_update_refresh_zone_udp) = mpsc::channel();
        let (update_refresh_zone_tcp, _rx_update_refresh_zone_tcp) = mpsc::channel();
        let (update_zone_udp_resolver, _tx_update_zone_udp_resolver) = mpsc::channel();
        let (update_zone_tcp_resolver, _tx_update_zone_tcp_resolver) = mpsc::channel();

        let name_server = NameServer::new(
            true,
            delete_sender_udp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp, 
            add_sender_ns_tcp, 
            delete_sender_ns_tcp, 
            update_refresh_zone_udp,
            update_refresh_zone_tcp,
            update_zone_udp_resolver,
            update_zone_tcp_resolver,
        );

        let mut nszone = NSZone::new();
        let mut nsnode = NSNode::new();

        nsnode.set_name("mail.example.com".to_string());
        nszone.set_zone_nodes(nsnode.clone());
        nszone.set_name(String::from("example.com"));
        nszone.set_ip_address_for_refresh_zone(String::from("127.0.0.0"));

        let update_zone_udp_test = name_server.get_update_zone_udp_resolver();
        let update_zone_udp_rcv = _tx_update_zone_udp_resolver;

        update_zone_udp_test.send(nszone).unwrap();
        let res_nszone = update_zone_udp_rcv.recv().unwrap();

        assert_eq!(res_nszone.get_name(), String::from("example.com"));
        assert_eq!(res_nszone.get_zone_nodes().get_name(), nsnode.clone().get_name());
        assert_eq!(res_nszone.get_ip_address_for_refresh_zone(), String::from("127.0.0.0"));
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn get_update_zone_tcp_resolver(){
        let (delete_sender_udp, _delete_recv_udp) = mpsc::channel();
        let (delete_sender_tcp, _delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, _add_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, _add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_udp, _delete_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_tcp, _delete_recv_ns_tcp) = mpsc::channel();
        let (update_refresh_zone_udp, _rx_update_refresh_zone_udp) = mpsc::channel();
        let (update_refresh_zone_tcp, _rx_update_refresh_zone_tcp) = mpsc::channel();
        let (update_zone_udp_resolver, _tx_update_zone_udp_resolver) = mpsc::channel();
        let (update_zone_tcp_resolver, _tx_update_zone_tcp_resolver) = mpsc::channel();

        let name_server = NameServer::new(
            true,
            delete_sender_udp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp, 
            add_sender_ns_tcp, 
            delete_sender_ns_tcp, 
            update_refresh_zone_udp,
            update_refresh_zone_tcp,
            update_zone_udp_resolver,
            update_zone_tcp_resolver,
        );

        let mut nszone = NSZone::new();
        let mut nsnode = NSNode::new();

        nsnode.set_name("mail.example.com".to_string());
        nszone.set_zone_nodes(nsnode.clone());
        nszone.set_name(String::from("example.com"));
        nszone.set_ip_address_for_refresh_zone(String::from("127.0.0.0"));

        let update_zone_tcp_test = name_server.get_update_zone_tcp_resolver();
        let update_zone_tcp_rcv = _tx_update_zone_tcp_resolver;

        update_zone_tcp_test.send(nszone).unwrap();
        let res_nszone = update_zone_tcp_rcv.recv().unwrap();

        assert_eq!(res_nszone.get_name(), String::from("example.com"));
        assert_eq!(res_nszone.get_zone_nodes().get_name(), nsnode.clone().get_name());
        assert_eq!(res_nszone.get_ip_address_for_refresh_zone(), String::from("127.0.0.0"));
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn add_zone_from_master_file(){
        let (delete_sender_udp, _delete_recv_udp) = mpsc::channel();
        let (delete_sender_tcp, _delete_recv_tcp) = mpsc::channel();
        let (add_sender_ns_udp, _add_recv_ns_udp) = mpsc::channel();
        let (add_sender_ns_tcp, _add_recv_ns_tcp) = mpsc::channel();
        let (delete_sender_ns_udp, _delete_recv_ns_udp) = mpsc::channel();
        let (delete_sender_ns_tcp, _delete_recv_ns_tcp) = mpsc::channel();
        let (update_refresh_zone_udp, _rx_update_refresh_zone_udp) = mpsc::channel();
        let (update_refresh_zone_tcp, _rx_update_refresh_zone_tcp) = mpsc::channel();
        let (update_zone_udp_resolver, _tx_update_zone_udp_resolver) = mpsc::channel();
        let (update_zone_tcp_resolver, _tx_update_zone_tcp_resolver) = mpsc::channel();

        let mut name_server = NameServer::new(
            true,
            delete_sender_udp,
            delete_sender_tcp,
            add_sender_ns_udp,
            delete_sender_ns_udp, 
            add_sender_ns_tcp, 
            delete_sender_ns_tcp, 
            update_refresh_zone_udp,
            update_refresh_zone_tcp,
            update_zone_udp_resolver,
            update_zone_tcp_resolver,
        );

        let file_name = "test.txt".to_string();
        let origin = "example".to_string();
        let ip = "192.80.24.11".to_string();

        assert_eq!(name_server.get_zones_by_class().len(), 0);

        name_server.add_zone_from_master_file(file_name, origin, ip, true);

        let res = name_server.get_zones_by_class();

        assert!(res.contains_key(&1));
        assert_eq!(res.len(), 1);
    }

    //ToDo: Revisar Práctica 1
    #[test]
    fn search_nearest_ancestor_zone(){

    }
}
