use std::sync::Arc;

use dashmap::{DashMap, Entry};
use tokio::sync::mpsc::Sender;

use crate::{
    client::client_main::BackendServerType,
    generated::proto_client::{
        BeginConnectGateway, zone_broadcast_service_client::ZoneBroadcastServiceClient,
        zone_sync_service_client::ZoneSyncServiceClient,
    },
    network::proto::proto::{self, ToClientMessage},
};

#[derive(Debug)]
struct ZoneClient {
    #[allow(dead_code)]
    zone_id: u64,

    users: Arc<DashMap<u64, Sender<(BackendServerType, proto::ToClientMessage)>>>,
}

#[derive(Debug)]
pub struct ZoneClients {
    channels: DashMap<u64, tonic::transport::Channel>,

    clients: DashMap<u64, ZoneClient>,
}

impl ZoneClients {
    pub fn new() -> Self {
        Self {
            channels: DashMap::new(),
            clients: DashMap::new(),
        }
    }

    /// Zoneクライアントを取得
    pub async fn sync_zone_mut(
        &self,
        zone_id: u64,
    ) -> Option<ZoneSyncServiceClient<tonic::transport::Channel>> {
        self.channels
            .get(&zone_id)
            .map(|entry| entry.value().clone())
            .map(|channel| ZoneSyncServiceClient::new(channel))
    }

    pub async fn register_user_to_zone(
        &self,
        zone_id: u64,
        user_id: u64,
        user_sender: Sender<(BackendServerType, proto::ToClientMessage)>,
    ) -> Option<()> {
        if !self.channels.contains_key(&zone_id) {
            log::warn!(
                "Zone client for zone_id {} does not exist. Cannot broadcast.",
                zone_id
            );
            return None;
        }

        match self.clients.entry(zone_id) {
            Entry::Occupied(entry) => {
                log::info!(
                    "Zone client for zone_id {} already exists. Returning existing client.",
                    zone_id
                );
                entry.get().users.insert(user_id, user_sender);
                Some(())
            }
            Entry::Vacant(entry) => {
                log::info!(
                    "Creating new ZoneClient for zone_id {} and broadcasting.",
                    zone_id
                );
                let channel = self.channels.get(&zone_id).unwrap().clone();
                let mut client = ZoneBroadcastServiceClient::new(channel);
                let mut streaming = match client
                    .zone_sync_stream(BeginConnectGateway { gateway_id: 0 }) // TODO
                    .await
                {
                    Ok(stream) => stream.into_inner(),
                    Err(e) => {
                        log::error!(
                            "Failed to connect to ZoneBroadcastService for zone_id {}: {:?}",
                            zone_id,
                            e
                        );
                        return None;
                    }
                };

                let users = Arc::new(DashMap::new());

                entry.insert(ZoneClient {
                    zone_id,
                    users: users.clone(),
                });

                users.insert(user_id, user_sender);
                log::info!("Started broadcasting for zone_id {}.", zone_id);

                tokio::spawn(async move {
                    while let Ok(Some(message)) = streaming.message().await {
                        if users.is_empty() {
                            log::info!(
                                "No users to broadcast to for zone_id {}. Stopping broadcast.",
                                zone_id
                            );
                            break;
                        }
                        log::trace!(
                            "Received message from ZoneBroadcastService for zone_id {}: {:?}",
                            zone_id,
                            message
                        );
                        let Some(payload) = message
                            .payload
                            .and_then(|opt| ToClientMessage::try_from(opt).ok())
                        else {
                            log::warn!(
                                "Received message with empty payload from ZoneBroadcastService for zone_id {}.",
                                zone_id
                            );
                            continue;
                        };
                        users.iter().for_each(|entry| {
                            let _sender = entry
                                .value()
                                .try_send((BackendServerType::Zone(zone_id), payload.clone()));
                        });
                    }
                    drop(streaming);
                    log::info!("Stopped broadcasting for zone_id {}.", zone_id);
                });

                Some(())
            }
        }
    }

    pub fn unregister_user_from_zone(&self, zone_id: u64, user_id: u64) {
        let should_remove_client = match self.clients.get(&zone_id) {
            Some(client) => {
                client.users.remove(&user_id);
                client.users.is_empty()
            }
            None => {
                log::warn!(
                    "Zone client for zone_id {} does not exist. Cannot unregister user.",
                    zone_id
                );
                false
            }
        };

        if should_remove_client {
            log::info!(
                "No more users in zone_id {}. Removing ZoneClient and channel.",
                zone_id
            );
            self.clients.remove(&zone_id);
        }
    }

    pub fn add_channel(&self, zone_id: u64, client: tonic::transport::Channel) {
        if self.channels.contains_key(&zone_id) {
            log::warn!(
                "Zone client for zone_id {} already exists. Skipping add.",
                zone_id
            );
            return;
        }

        log::info!("Adding zone client for zone_id {}", zone_id);
        match self.channels.entry(zone_id) {
            Entry::Occupied(_) => {
                log::warn!(
                    "Zone client for zone_id {} already exists. Skipping add.",
                    zone_id
                );
            }
            Entry::Vacant(entry) => {
                entry.insert(client);
                log::info!("Zone client for zone_id {} added successfully.", zone_id);
            }
        }
    }

    /// 不要になったZoneクライアントを削除
    pub fn remove_channel(&self, zone_id: u64) {
        self.channels.remove(&zone_id);
    }
}
