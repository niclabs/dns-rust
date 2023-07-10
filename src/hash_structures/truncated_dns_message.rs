use std::collections::HashMap;
use crate::dns_message::DnsMessage;

type ID = u16;

#[derive(Clone, PartialEq, Debug)]
/// Struct to save the truncated messages.
/// 
/// When a new message is received and it is truncated, 
/// it is saved in this struct. When a new message with 
/// the same ID is received, it is added to the previous 
/// message.
pub struct TruncatedDnsMessage {
    /// HashMap to save the truncated messages according to their ID.
    truncated_messages_hash: HashMap<ID, DnsMessage>,
}

impl FragmentedDnsMessage {
    fn new() -> Self {
        FragmentedDnsMessage {
            messages: HashMap::new(),
        }
    }

    fn add_message(&mut self, msg_id: id, dns_message: DnsMessage) {
        self.messages.insert(msg_id, dns_message);
    }

    fn remove_message(&mut self, msg_id: &id) -> Option<DnsMessage> {
        self.messages.remove(msg_id)
    }

    fn get_dns_message(&self, msg_id: &id) -> Option<&DnsMessage> {
        self.messages.get(key)
    }

    fn get_messages(&self) -> HashMap<id, DnsMessage> {
        self.messages.clone()
    }
}

impl FragmentedDnsMessage {
    fn set_messages(&mut self, messages: HashMap<id, DnsMessage>) {
        self.messages = messages;
    }

    fn set_dns_message(&mut self, msg_id: id, dns_message: DnsMessage) {
        self.messages.insert(msg_id, dns_message);
    }
}