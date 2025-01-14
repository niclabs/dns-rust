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
            false,
            1);

    dns_query_message.add_edns0(None, Rcode::NOERROR, 0, true,Some(vec![OptionCode::EDE]));

    client.set_dns_query(dns_query_message);
    let res = client.send_query().await;

    if let Ok(response) = res {
        let additional = response.get_additional();

        assert_eq!(additional.len(), 1);

        let rr = &additional[0];

        assert_eq!(rr.get_name().get_name(), String::from("."));

        assert_eq!(rr.get_rtype(), Rrtype::OPT);

        assert_eq!(rr.get_rclass(), Rclass::UNKNOWN(512));

        assert_eq!(rr.get_ttl(), 32768);

        assert_eq!(rr.get_rdlength(), 4);

        let rdata = rr.get_rdata();

        match rdata {
            Rdata::OPT(opt) => {
                let options = opt.get_option();
                let mut expected = OptOption::new(OptionCode::EDE);
                expected.set_opt_data(OptionData::EDE(EdeOptData::new(EdeCode::from(22), "At delegation nonexistent.com for nonexistent.com/a".to_string())));
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
            false,
            1);

    dns_query_message.add_edns0(None, Rcode::NOERROR, 0, true,Some(vec![OptionCode::EDE, OptionCode::NSID]));

    client.set_dns_query(dns_query_message);
    let res = client.send_query().await;

    if let Ok(response) = res {
        let additional = response.get_additional();

        assert_eq!(additional.len(), 1);

        let rr = &additional[0];

        assert_eq!(rr.get_name().get_name(), String::from("."));

        assert_eq!(rr.get_rtype(), Rrtype::OPT);

        assert_eq!(rr.get_rclass(), Rclass::UNKNOWN(512));

        assert_eq!(rr.get_ttl(), 32768);

        assert_eq!(rr.get_rdlength(), 8);

        let rdata = rr.get_rdata();

        match rdata {
            Rdata::OPT(opt) => {
                let options = opt.get_option();
                let mut first_expected = OptOption::new(OptionCode::EDE);
                first_expected.set_opt_data(OptionData::EDE(EdeOptData::new(EdeCode::from(22), "At delegation nonexistent.com for nonexistent.com/a".to_string())));
                assert_eq!(options[0], first_expected);

                let mut second_expected = OptOption::new(OptionCode::NSID);
                second_expected.set_opt_data(OptionData::from_bytes_with_opt_type("iad.aservers.dns.icann.org".to_string().into_bytes(), OptionCode::NSID).unwrap());

            },
            _ =>{}
        }
    }

}