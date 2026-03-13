use std::collections::HashMap;

use crate::network::*;

pub struct ClientMessageHandler {
    handlers: HashMap<proto::packet::MessageCase, Box<dyn Fn(proto::Packet) + Send + Sync>>,
}

impl ClientMessageHandler {
    pub fn new() -> Self {
        let mut handlers = HashMap::new();

        Self { handlers }
    }
}
