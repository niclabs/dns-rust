use std::collections::HashMap;
use crate::dns_message::DnsMessage;

type ID = u16;

#[derive(Clone, PartialEq, Debug)]
pub struct FragmentedDnsMessage {
    message: HashMap<ID, DnsMessage>,

}

impl FragmentedDnsMessage {}