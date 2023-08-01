use std::collections::HashMap;
use crate::message::DnsMessage;

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
    truncated_messages: HashMap<ID, Vec<DnsMessage>>,
}

impl TruncatedDnsMessage {
    /// Function to create a new TruncatedDnsMessage.
    fn new() -> Self {
        TruncatedDnsMessage {
            truncated_messages: HashMap::new(),
        }
    }

    /// Function to add a new message to the TruncatedDnsMessage.
    /// #Example
    /// ```
    /// let mut truncated_dns_message = TruncatedDnsMessage::new();
    /// let dns_message = DnsMessage::new();
    /// truncated_dns_message.add_message(1, dns_message);
    /// ```
    fn add_message(&mut self, msg_id: ID, dns_message: DnsMessage) {
        let mut truncated_messages = self.get_truncated_messages_hash();
        if let Some(y) = truncated_messages.get_mut(&msg_id) {
            let mut dns_message_vec = y.clone();
            dns_message_vec.push(dns_message);
            truncated_messages.insert(msg_id, dns_message_vec);
        }
        else {
            let mut dns_message_vec = Vec::new();
            dns_message_vec.push(dns_message);
            truncated_messages.insert(msg_id, dns_message_vec);
        }
        self.set_truncated_messages_hash(truncated_messages);
    }

    /// Function to remove a message from the TruncatedDnsMessage.
    fn remove_message(&mut self, msg_id: &ID){
        let mut truncated_messages = self.get_truncated_messages_hash();
        if let Some(y) = truncated_messages.remove(&msg_id) {
            self.set_truncated_messages_hash(truncated_messages)
        }
    }

    /// Function to get a message from the TruncatedDnsMessage.
    fn get_dns_message(&self, msg_id: &ID) -> Option<Vec<DnsMessage>> {
        let truncated_messages = self.get_truncated_messages_hash();
        if let Some(y) = truncated_messages.get(&msg_id) {
            return Some(y.clone());
        }
        else {
            return None;
        }
    }
}

impl TruncatedDnsMessage {
    /// Function to create a new TruncatedDnsMessage.
    fn set_truncated_messages_hash(&mut self, truncated_messages_hash: HashMap<ID, Vec<DnsMessage>>) {
        self.truncated_messages = truncated_messages_hash;
    }

    /// Function to get all the messages from the TruncatedDnsMessage.
    fn get_truncated_messages_hash(&self) -> HashMap<ID, Vec<DnsMessage>> {
        return self.truncated_messages.clone();
    }
}