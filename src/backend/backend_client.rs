use dashmap::DashMap;
use tokio::sync::OnceCell;
use tonic::transport::Endpoint;

use crate::generated::proto_client::{
    lobby_service_client::LobbyServiceClient,
    record_player_db_service_client::RecordPlayerDbServiceClient,
    user_db_service_client::UserDbServiceClient, world_command_client::WorldCommandClient,
    zone_sync_service_client::ZoneSyncServiceClient,
};

#[derive(Debug)]
pub struct BackendClient {
    pub user_db: UserDbServiceClient<tonic::transport::Channel>,
    pub record_player_db: RecordPlayerDbServiceClient<tonic::transport::Channel>,
    pub lobby: LobbyServiceClient<tonic::transport::Channel>,
    pub world_command: WorldCommandClient<tonic::transport::Channel>,
    pub zones: DashMap<u64, tonic::transport::Channel>,
}

impl BackendClient {
    pub async fn new(
        db_endpoint: Endpoint,
        world_endpoint: Endpoint,
        lobby_endpoint: Endpoint,
    ) -> Self {
        let user_db = UserDbServiceClient::connect(db_endpoint.clone())
            .await
            .expect("Failed to connect to UserDBService.");
        let record_player_db = RecordPlayerDbServiceClient::connect(db_endpoint.clone())
            .await
            .expect("Failed to connect to RecordPlayerDBService.");
        let world_command = WorldCommandClient::connect(world_endpoint)
            .await
            .expect("Failed to connect to WorldRouteService.");
        let lobby = LobbyServiceClient::connect(lobby_endpoint)
            .await
            .expect("Failed to connect to LobbyService.");

        BackendClient {
            user_db,
            record_player_db,
            lobby,
            world_command,
            zones: DashMap::new(),
        }
    }

    /// Zoneクライアントを取得
    pub async fn zone_mut(
        &self,
        zone_id: u64,
    ) -> Option<ZoneSyncServiceClient<tonic::transport::Channel>> {
        self.zones
            .get(&zone_id)
            .map(|entry| entry.value().clone())
            .map(|channel| ZoneSyncServiceClient::new(channel))
    }

    pub fn add_zone(&self, zone_id: u64, client: tonic::transport::Channel) {
        if self.zones.contains_key(&zone_id) {
            log::warn!(
                "Zone client for zone_id {} already exists. Skipping add.",
                zone_id
            );
            return;
        }

        log::info!("Adding zone client for zone_id {}", zone_id);
        self.zones.entry(zone_id).insert(client);
    }

    /// 不要になったZoneクライアントを削除
    pub fn remove_zone(&self, zone_id: u64) {
        self.zones.remove(&zone_id);
    }
}

pub static BACKEND_CLIENT_INSTANCE: OnceCell<BackendClient> = OnceCell::const_new();
