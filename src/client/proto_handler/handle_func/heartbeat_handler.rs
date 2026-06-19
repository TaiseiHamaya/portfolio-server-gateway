use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    client::client_main::{BackendServerType, Client},
    network::proto::proto::{self, to_server_message::MessageCase},
};

pub async fn handle_heartbeat(packet: proto::ToServerMessage, client: Arc<RwLock<Client>>) {
    let message_case = packet.message_case();
    tokio::spawn(async move {
        match message_case {
            MessageCase::HeartbeatRequest => {
                let mut response = proto::ToClientMessage::new();
                response.clear_heartbeat_response();
                {
                    let mut heartbeat_response = response.heartbeat_response_mut();
                    heartbeat_response.set_timestamp(chrono::Utc::now().timestamp_micros());
                }
                match client
                    .write()
                    .await
                    .tx
                    .send((BackendServerType::ECHO, response))
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("Failed to send heartbeat response: {}", e);
                    }
                }
            }
            _ => {
                log::warn!(
                    "Received unexpected message type in heartbeat handler: {:?}",
                    message_case
                );
            }
        }
    });
}
