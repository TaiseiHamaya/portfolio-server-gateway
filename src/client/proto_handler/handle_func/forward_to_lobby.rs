use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    backend::backend_client,
    client::client_main::{BackendServerType, Client, ClientStatus},
    generated::proto_client::PayloadLobbyStartGameRequest,
    network::proto::proto::{self, to_server_message::MessageCase},
};

pub async fn forward_to_lobby(message: proto::ToServerMessage, client: Arc<RwLock<Client>>) {
    let message_case = message.message_case();
    if client.read().await.status != ClientStatus::Lobby {
        log::warn!("Received message is not match to client status.");
        return;
    }
    let Some(user_id) = client.read().await.user_id else {
        log::warn!("User ID is not set for the client.");
        return;
    };
    match message_case {
        MessageCase::StartGame => {
            let mut lobby_client = backend_client::BACKEND_CLIENT_INSTANCE
                .get()
                .expect("Uninitialized BackendClient instance")
                .lobby
                .clone();
            let client_clone = client.clone();

            tokio::spawn(async move {
                client_clone.write().await.status = ClientStatus::Routing;
                let response = match lobby_client
                    .start_game(PayloadLobbyStartGameRequest { user_id })
                    .await
                {
                    Ok(response) => {
                        log::info!(
                            "Successfully started game for user_id {}: zone_id: {}, player_entity_id: {}",
                            user_id,
                            response.get_ref().zone_id,
                            response.get_ref().player_entity_id
                        );
                        response
                    }
                    Err(e) => {
                        log::error!("Failed to start game for user_id {}: {:?}", user_id, e);
                        return;
                    }
                };

                let (zone_id, player_entity_id) = (
                    response.get_ref().zone_id,
                    response.get_ref().player_entity_id,
                );

                let mut message = proto::ToClientMessage::new();
                message
                    .start_game_response_mut()
                    .set_player_entity_id(player_entity_id);
                message.start_game_response_mut().set_zone_id(zone_id);
                client_clone
                    .write()
                    .await
                    .tx
                    .send((BackendServerType::Lobby, message))
                    .await
                    .unwrap_or_else(|e| {
                        log::error!("Failed to send StartGameResponse to client: {}", e);
                    });
            });
        }
        MessageCase::EndGame => {}
        _ => {
            log::warn!("Received message is not match to client status.");
        }
    }
}
