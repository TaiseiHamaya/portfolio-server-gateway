use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    backend::backend_client,
    client::client_main::{BackendServerType, Client, ClientStatus},
    generated::proto_client::{PayloadExecuteRoute, PayloadLobbyStartGameRequest},
    network::proto::proto::{self, Vector3, to_server_message::MessageCase},
};

pub async fn start_game_handler(message: proto::ToServerMessage, client: Arc<RwLock<Client>>) {
    let message_case = message.message_case();
    let Some(user_id) = client.read().await.user_id.clone() else {
        log::warn!("User ID is not set for the client.");
        return;
    };
    match message_case {
        MessageCase::StartGame => {
            if client.read().await.status != ClientStatus::Lobby {
                log::warn!("Received message is not match to client status.");
                return;
            }
            let mut lobby_client = backend_client::BACKEND_CLIENT_INSTANCE
                .get()
                .expect("Uninitialized BackendClient instance")
                .lobby
                .clone();
            let client_clone = client.clone();

            tokio::spawn(async move {
                // 移動先が未確定なのでNone
                client_clone.write().await.status = ClientStatus::Routing(None);
                log::info!(
                    "Client status updated to Routing(None) for user_id {}",
                    user_id
                );

                // ロビーサーバーにStartGameリクエストを送信
                let (zone_id, player_entity_id, position) = match lobby_client
                    .start_game(PayloadLobbyStartGameRequest { user_id })
                    .await
                {
                    Ok(response) => {
                        let (zone_id, player_entity_id, position) = (
                            response.get_ref().zone_id,
                            response.get_ref().player_entity_id,
                            response.get_ref().position.clone(),
                        );
                        log::info!(
                            "Successfully started game for user_id {}: zone_id: {}, player_entity_id: {}, position: {:?}",
                            user_id,
                            zone_id,
                            player_entity_id,
                            position
                        );
                        (zone_id, player_entity_id, position)
                    }
                    Err(e) => {
                        log::error!("Failed to start game for user_id {}: {:?}", user_id, e);
                        return;
                    }
                };

                let tx = client_clone.read().await.tx.clone();

                // zoneサーバー接続してメッセージを受け取るようにする
                backend_client::BACKEND_CLIENT_INSTANCE
                    .get()
                    .expect("Uninitialized BackendClient instance")
                    .zone_clients
                    .register_user_to_zone(zone_id, user_id, tx)
                    .await;

                // クライアントの状態を移動先が確定した状態にする
                client_clone.write().await.status = ClientStatus::Routing(Some(zone_id));
                log::info!(
                    "Client status updated to Routing(Some({})) for user_id {}",
                    zone_id,
                    user_id
                );

                // クライアントにStartGameResponseを転送
                let mut message = proto::ToClientMessage::new();
                message
                    .start_game_response_mut()
                    .set_player_entity_id(player_entity_id);
                message.start_game_response_mut().set_zone_id(zone_id);
                if let Some(position) = position {
                    message
                        .start_game_response_mut()
                        .set_position(Vector3::from(position));
                }
                client_clone
                    .read()
                    .await
                    .tx
                    .send((BackendServerType::Lobby, message))
                    .await
                    .unwrap_or_else(|e| {
                        log::error!("Failed to send StartGameResponse to client: {}", e);
                    });
                log::info!(
                    "Sent StartGameResponse to client for user_id {}: zone_id: {}, player_entity_id: {}, position: {:?}",
                    user_id,
                    zone_id,
                    player_entity_id,
                    position
                );
            });
        }
        MessageCase::PlayerZoneEnterComplete => {
            let current_status = client.read().await.status;
            if let ClientStatus::WaitEnter(zone_id) = current_status {
                log::info!(
                    "Received PlayerZoneEnterComplete from user_id {} for zone_id {}",
                    user_id,
                    zone_id
                );
                let mut world_client = backend_client::BACKEND_CLIENT_INSTANCE
                    .get()
                    .expect("Uninitialized BackendClient instance")
                    .world_command
                    .clone();

                match world_client
                    .execute_zone_route(PayloadExecuteRoute { user_id })
                    .await
                {
                    Ok(_) => {
                        log::info!(
                            "Successfully completed zone route for user_id {}: entered zone_id {}",
                            user_id,
                            zone_id
                        );
                    }
                    Err(e) => {
                        log::error!(
                            "Failed to complete zone route for user_id {}: {:?}",
                            user_id,
                            e
                        );
                    }
                }
            } else {
                log::warn!(
                    "Received message is not match to client status. message:{:?}, status:{:?}",
                    message_case,
                    current_status,
                );
                return;
            }
        }
        _ => {
            log::warn!("Received message is not match to client status.");
        }
    }
}
