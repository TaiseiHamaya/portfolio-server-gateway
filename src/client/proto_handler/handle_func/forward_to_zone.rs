use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    backend::backend_client,
    client::client_main::{Client, ClientStatus},
    generated::proto_client::{self, PayloadTransformSync},
    network::proto::proto::{self, to_server_message::MessageCase},
};

pub async fn forward_to_zone(packet: proto::ToServerMessage, client: Arc<RwLock<Client>>) {
    let status = client.read().await.status;
    let zone_id = match status {
        ClientStatus::Zone(zone_id) => zone_id,
        _ => {
            log::error!("Client is not in a zone");
            return;
        }
    };
    let message_case = packet.message_case();
    let mut zone_sync_client = match backend_client::BACKEND_CLIENT_INSTANCE
        .get()
        .expect("Uninitialized BackendClient instance")
        .zone_clients
        .sync_zone_mut(zone_id)
        .await
    {
        Some(client) => client,
        None => {
            log::error!("Failed to get zone client for zone_id {}.", zone_id);
            return;
        }
    };

    tokio::spawn(async move {
        match message_case {
            MessageCase::PlayAction => {
                let play_action_view = packet.play_action();
                match zone_sync_client
                    .play_action(proto_client::PayloadPlayAction {
                        id: play_action_view.id(),
                        target_id: play_action_view.target_id(),
                        action_id: play_action_view.action_id(),
                        timestamp: play_action_view.timestamp(),
                    })
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("Failed to forward PlayAction to zone: {}", e);
                    }
                };
            }
            MessageCase::TransformSync => {
                let transform_sync_view = packet.transform_sync();
                log::info!(
                    "Forwarding TransformSync to zone. id: {}, timestamp: {}, position: {:?}",
                    transform_sync_view.id(),
                    transform_sync_view.timestamp(),
                    transform_sync_view.position()
                );
                match zone_sync_client
                    .sync_transform(PayloadTransformSync {
                        id: transform_sync_view.id(),
                        timestamp: transform_sync_view.timestamp(),
                        position: Some(transform_sync_view.position().to_owned().into()),
                    })
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("Failed to forward TransformSync to zone: {}", e);
                    }
                };
            }
            _ => {
                log::warn!(
                    "Received unsupported message type for forwarding to zone: {:?}",
                    message_case
                );
            }
        }
    });
}
