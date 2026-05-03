use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    backend::backend_client,
    client::client_main::{Client, ClientStatus},
    generated::proto_client::{PayloadBeginRouteCommand, PayloadLobbyEndGameRequest},
    network::proto::proto::{self},
};

pub async fn endgame_handler(_message: proto::ToServerMessage, client: Arc<RwLock<Client>>) {
    let status = client.read().await.status;
    let user_id = match client.read().await.user_id {
        Some(id) => id,
        None => {
            log::warn!("User ID is not set for the client.");
            return;
        }
    };
    if let ClientStatus::Zone(zone_id) = status {
        client.write().await.status = ClientStatus::Routing(None);
        log::info!(
            "Client status updated to Routing(None) for user_id {}",
            user_id
        );

        let mut world_route_client = backend_client::BACKEND_CLIENT_INSTANCE
            .get()
            .expect("Uninitialized BackendClient instance")
            .world_command
            .clone();

        let response = match world_route_client
            .begin_zone_route(PayloadBeginRouteCommand {
                user_id,
                source_zone_id: Some(zone_id),
                target_zone_id: None,
            })
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                log::error!("Error occurred while beginning zone route: {}", e);
                return;
            }
        };

        backend_client::BACKEND_CLIENT_INSTANCE
            .get()
            .expect("Uninitialized BackendClient instance")
            .zone_clients
            .unregister_user_from_zone(zone_id, user_id);

        let player_data = response.get_ref().player_data;
        log::info!(
            "Zone route ended for user_id: {}, player_data: {:?}",
            user_id,
            player_data
        );
        client.write().await.entity_id = None;
        client.write().await.status = ClientStatus::Disconnected;
        log::info!(
            "Client status updated to Disconnected for user_id {}",
            user_id
        );
    } else if status == ClientStatus::Lobby {
        let mut lobby_client = backend_client::BACKEND_CLIENT_INSTANCE
            .get()
            .expect("Uninitialized BackendClient instance")
            .lobby
            .clone();
        let client = client.clone();

        tokio::spawn(async move {
            let response = match lobby_client
                .end_game(PayloadLobbyEndGameRequest { user_id })
                .await
            {
                Ok(resp) => resp,
                Err(e) => {
                    log::error!("Error occurred while ending game: {}", e);
                    return;
                }
            };

            if response.get_ref().success {
                log::info!("Game ended successfully for user_id: {}", user_id);
            } else {
                log::warn!("Failed to end game for user_id: {}", user_id);
            }
            client.write().await.status = ClientStatus::Disconnected;
            log::info!(
                "Client status updated to Disconnected for user_id {}",
                user_id
            );
        });
    }
    // TODO: ルーティング中に切断された場合の処理
}
