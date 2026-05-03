use tokio::sync::OnceCell;
use tonic::transport::Endpoint;

use crate::{
    backend::zone_clients::ZoneClients,
    generated::proto_client::{
        lobby_service_client::LobbyServiceClient,
        record_player_db_service_client::RecordPlayerDbServiceClient,
        user_db_service_client::UserDbServiceClient, world_command_client::WorldCommandClient,
    },
};

#[derive(Debug)]
pub struct BackendClient {
    pub user_db: UserDbServiceClient<tonic::transport::Channel>,
    pub record_player_db: RecordPlayerDbServiceClient<tonic::transport::Channel>,

    pub lobby: LobbyServiceClient<tonic::transport::Channel>,
    pub world_command: WorldCommandClient<tonic::transport::Channel>,

    pub zone_clients: ZoneClients,
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
            zone_clients: ZoneClients::new(),
        }
    }
}

pub static BACKEND_CLIENT_INSTANCE: OnceCell<BackendClient> = OnceCell::const_new();
