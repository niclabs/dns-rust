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

impl TruncatedDnsMessage {
    /// Function to create a new TruncatedDnsMessage.
    fn new() -> Self {
        TruncatedDnsMessage {
            messages: HashMap::new(),
        }
    }

    /// Function to add a new message to the TruncatedDnsMessage.
    /// todo: check if the message is already in the TruncatedDnsMessage.
    fn add_message(&mut self, msg_id: id, dns_message: DnsMessage) {
        self.messages.insert(msg_id, dns_message);
    }

    /// Function to remove a message from the TruncatedDnsMessage.
    fn remove_message(&mut self, msg_id: &id) -> Option<DnsMessage> {
        self.messages.remove(msg_id);
    }

    /// Function to get a message from the TruncatedDnsMessage.
    fn get_dns_message(&self, msg_id: &id) -> Option<&DnsMessage> {
        self.messages.get(key);
    }
}

impl TruncatedDnsMessage {
    /// Function to create a new TruncatedDnsMessage.
    fn set_truncated_messages_hash(&mut self, truncated_messages_hash: HashMap<id, DnsMessage>) {
        self.truncated_messages_hash = truncated_messages_hash;
    }

    /// Function to get all the messages from the TruncatedDnsMessage.
    fn get_truncated_messages_hash(&self) -> HashMap<id, DnsMessage> {
        self.truncated_messages_hash.clone();
    }
}