use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use dns_rust::client::Client;
use dns_rust::client::client_connection::ClientConnection;
use dns_rust::client::udp_connection::ClientUDPConnection;
use dns_rust::domain_name::DomainName;
use dns_rust::edns::opt_option::option_code::OptionCode;
use dns_rust::edns::opt_option::option_data::OptionData;
use dns_rust::edns::opt_option::OptOption;
use dns_rust::edns::options::ede::ede_code::EdeCode;
use dns_rust::edns::options::ede::ede_optdata::EdeOptData;
use dns_rust::message::DnsMessage;
use dns_rust::message::rclass::Rclass;
use dns_rust::message::rcode::Rcode;
use dns_rust::message::rdata::Rdata;
use dns_rust::message::rrtype::Rrtype;

#[tokio::test]
async fn client_edns_ede_code() {
    // client
    let addr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
    let conn = ClientUDPConnection::new_default(addr, Duration::from_secs(10));
    let mut client = Client::new(conn);

    // message
    let mut dns_query_message =
        DnsMessage::new_query_message(
            DomainName::new_from_string("nonexistent.com".to_string()),
            Rrtype::A,
            Rclass::IN,
            0,
            true,
            1);

    dns_query_message.add_edns0(None, Rcode::NOERROR, 0, true,Some(vec![OptionCode::EDE]));

    client.set_dns_query(dns_query_message);
    let res = client.send_query().await;

    if let Err(response) = res {panic!("couldnt send the message")}

    if let Ok(response) = res {
        let additional = response.get_additional();

        assert_eq!(additional.len(), 1);

        let rr = &additional[0];

        assert_eq!(rr.get_name().get_name(), String::from(""));

        assert_eq!(rr.get_rtype(), Rrtype::OPT);

        assert_eq!(rr.get_rclass(), Rclass::UNKNOWN(512));

        assert_eq!(rr.get_ttl(), 32768);

        assert_eq!(rr.get_rdlength(), 57);

        let rdata = rr.get_rdata();

        match rdata {
            Rdata::OPT(opt) => {
                let options = opt.get_option();

                let mut expected = OptOption::new(OptionCode::EDE);
                let mut expected_opt_data = EdeOptData::new();
                expected_opt_data.set_info_code(EdeCode::from(22));
                expected_opt_data.set_extra_text("At delegation nonexistent.com for nonexistent.com/a".to_string());
                expected.set_opt_data(OptionData::EDE(expected_opt_data));
                expected.set_option_len(53);
                assert_eq!(options[0], expected);

            },
            _ =>{}
        }
    }
}



#[tokio::test]
async fn client_edns_two_options() {
    // client
    let addr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
    let conn = ClientUDPConnection::new_default(addr, Duration::from_secs(10));
    let mut client = Client::new(conn);

    // message
    let mut dns_query_message =
        DnsMessage::new_query_message(
            DomainName::new_from_string("nonexistent.com".to_string()),
            Rrtype::A,
            Rclass::IN,
            0,
            true,
            1);

    dns_query_message.add_edns0(Option::from(4096), Rcode::NOERROR, 0, true,Some(vec![OptionCode::NSID, OptionCode::EDE]));

    client.set_dns_query(dns_query_message);
    let res = client.send_query().await;

    if let Err(error) = res {panic!("couldnt send the message")}

    if let Ok(response) = res {
        let additional = response.get_additional();

        assert_eq!(additional.len(), 1);

        let rr = &additional[0];

        assert_eq!(rr.get_name().get_name(), String::from(""));

        assert_eq!(rr.get_rtype(), Rrtype::OPT);

        assert_eq!(rr.get_rclass(), Rclass::UNKNOWN(512));

        assert_eq!(rr.get_ttl(), 32768);

        assert_eq!(rr.get_rdlength(), 70);

        let rdata = rr.get_rdata();

        match rdata {
            Rdata::OPT(opt) => {
                let options = opt.get_option();

                let mut first_expected = OptOption::new(OptionCode::NSID);
                first_expected.set_option_len(9);
                first_expected.set_opt_data(OptionData::from_bytes_with_opt_type("gpdns-scl".to_string().into_bytes(), OptionCode::NSID).unwrap());
                assert_eq!(options[0], first_expected);

                let mut second_expected = OptOption::new(OptionCode::EDE);
                let mut expected_opt_data = EdeOptData::new();
                expected_opt_data.set_info_code(EdeCode::from(22));
                expected_opt_data.set_extra_text("At delegation nonexistent.com for nonexistent.com/a".to_string());
                second_expected.set_opt_data(OptionData::EDE(expected_opt_data));
                second_expected.set_option_len(53);

            },
            _ =>{}
        }
    }
}

#[tokio::test]
async fn client_edns_nsid() {
    // client
    let addr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
    let conn = ClientUDPConnection::new_default(addr, Duration::from_secs(10));
    let mut client = Client::new(conn);

    // message
    let mut dns_query_message =
        DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::A,
            Rclass::IN,
            0,
            false,
            1);

    dns_query_message.add_edns0(None, Rcode::NOERROR, 0, true,Some(vec![OptionCode::NSID]));

    client.set_dns_query(dns_query_message);
    let res = client.send_query().await;

    if let Err(error) = res {panic!("couldnt send the message")}

    if let Ok(response) = res {
        let additional = response.get_additional();

        assert_eq!(additional.len(), 1);

        let rr = &additional[0];

        assert_eq!(rr.get_name().get_name(), String::from(""));

        assert_eq!(rr.get_rtype(), Rrtype::OPT);

        assert_eq!(rr.get_rclass(), Rclass::UNKNOWN(512));

        assert_eq!(rr.get_ttl(), 32768);

        assert_eq!(rr.get_rdlength(), 13);

        let rdata = rr.get_rdata();

        match rdata {
            Rdata::OPT(opt) => {
                let options = opt.get_option();

                let mut expected = OptOption::new(OptionCode::NSID);
                expected.set_opt_data(OptionData::from_bytes_with_opt_type("gpdns-scl".to_string().into_bytes(), OptionCode::NSID).unwrap());
                expected.set_option_len(9);
                assert_eq!(options[0], expected);
            },
            _ =>{}
        }
    }
}

#[tokio::test]
async fn client_edns_padding() {
    // client
    let addr = IpAddr::V4(Ipv4Addr::new(74, 82, 42, 42)); // Hurricane Electric
    let conn = ClientUDPConnection::new_default(addr, Duration::from_secs(10));
    let mut client = Client::new(conn);

    // message
    let mut dns_query_message =
        DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::A,
            Rclass::IN,
            0,
            false,
            1);

    dns_query_message.add_edns0(None, Rcode::NOERROR, 0, true,Some(vec![OptionCode::PADDING]));

    client.set_dns_query(dns_query_message);
    let res = client.send_query().await;

    if let Err(error) = res {panic!("couldnt send the message")}

    if let Ok(response) = res {
        let additional = response.get_additional();

        assert_eq!(additional.len(), 1);

        let rr = &additional[0];

        assert_eq!(rr.get_name().get_name(), String::from(""));

        assert_eq!(rr.get_rtype(), Rrtype::OPT);

        assert_eq!(rr.get_rclass(), Rclass::UNKNOWN(512));

        assert_eq!(rr.get_ttl(), 32768);

        assert_eq!(rr.get_rdlength(), 428);

        let rdata = rr.get_rdata();

        match rdata {
            Rdata::OPT(opt) => {
                let options = opt.get_option();

                let mut expected = OptOption::new(OptionCode::PADDING);
                expected.set_opt_data(OptionData::from_bytes_with_opt_type(vec![0x00; 424], OptionCode::PADDING).unwrap());
                expected.set_option_len(424);
                assert_eq!(options[0], expected);
            },
            _ =>{}
        }
    }
}


// This test serves as a template for verifying EDNS support with the ZONEVERSION option.
// The current values are placeholders and should be adjusted based on the actual DNS server configuration
// and test expectations.
#[ignore]
#[tokio::test]
async fn client_edns_zoneversion() {
    // check all test parameters

    // client
    let addr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
    let conn = ClientUDPConnection::new_default(addr, Duration::from_secs(10));
    let mut client = Client::new(conn);
    // message
    let mut dns_query_message =
        DnsMessage::new_query_message(
            DomainName::new_from_string("example.com".to_string()),
            Rrtype::A,
            Rclass::IN,
            0,
            false,
            1);
    dns_query_message.add_edns0(None, Rcode::NOERROR, 0, true, Some(vec![OptionCode::ZONEVERSION]));
    client.set_dns_query(dns_query_message);
    let res = client.send_query().await;
    if let Err(_) = res { panic!("couldnt send the message") }

    if let Ok(response) = res {
        let additional = response.get_additional();

        assert_eq!(additional.len(), 1);

        let rr = &additional[0];

        assert_eq!(rr.get_name().get_name(), String::from(""));

        assert_eq!(rr.get_rtype(), Rrtype::OPT);

        assert_eq!(rr.get_rclass(), Rclass::UNKNOWN(512));

        assert_eq!(rr.get_ttl(), 32768);

        assert_eq!(rr.get_rdlength(), 428);

        let rdata = rr.get_rdata();

        match rdata {
            Rdata::OPT(opt) => {
                let options = opt.get_option();

                let mut expected = OptOption::new(OptionCode::ZONEVERSION);
                expected.set_opt_data(OptionData::from_bytes_with_opt_type(vec![0x00; 10], OptionCode::ZONEVERSION).unwrap());
                expected.set_option_len(10);
                assert_eq!(options[0], expected);
            },
            _ =>{}
        }
    }
}