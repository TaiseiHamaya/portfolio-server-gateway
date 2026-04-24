use std::net::SocketAddr;
use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::mpsc;

use crate::client::client_main::BackendServerType;
use crate::generated::proto_server::{
    PayloadEnemyDespawn, PayloadEnemySpawn, PayloadEntityDamaged, PayloadPlayAction,
    PayloadTextMessage, PayloadTransformSync, PayloadZoneEnterNotification,
    PayloadZoneExitNotification, zone_broadcast_service_server::ZoneBroadcastService,
};
use crate::network::proto::proto;

pub struct ZoneBroadcastServiceImpl {
    senders: Arc<DashMap<SocketAddr, mpsc::Sender<(BackendServerType, proto::ToClientMessage)>>>,
}

impl ZoneBroadcastServiceImpl {
    pub fn new(
        senders: Arc<
            DashMap<SocketAddr, mpsc::Sender<(BackendServerType, proto::ToClientMessage)>>,
        >,
    ) -> Self {
        Self { senders }
    }
}

#[tonic::async_trait]
impl ZoneBroadcastService for ZoneBroadcastServiceImpl {
    /// Zone → Client: player transform sync, play action, and chat message.
    async fn sync_transform(
        &self,
        request: tonic::Request<PayloadTransformSync>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let zone_id = 0;
        let inner = request.into_inner();
        let mut message = proto::ToClientMessage::default();
        let mut payload = message.transform_sync_mut();
        payload.set_id(inner.id);
        payload.set_position(proto::Vector3::from(inner.position.unwrap_or_default()));
        payload.set_timestamp(inner.timestamp);

        self.senders.iter().for_each(|entry| {
            let sender = entry.value();
            let _ = sender.try_send((BackendServerType::Zone(zone_id), message.clone()));
        });
        Ok(tonic::Response::new(()))
    }

    /// Zone → Client: player play action and chat message.
    async fn play_action(
        &self,
        request: tonic::Request<PayloadPlayAction>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let zone_id = 0;
        let inner = request.into_inner();
        let mut message = proto::ToClientMessage::default();
        let mut payload = message.play_action_mut();
        payload.set_id(inner.id);
        payload.set_action_id(inner.action_id);
        payload.set_timestamp(inner.timestamp);
        self.senders.iter().for_each(|entry| {
            let sender = entry.value();
            let _ = sender.try_send((BackendServerType::Zone(zone_id), message.clone()));
        });

        Ok(tonic::Response::new(()))
    }

    /// Zone → Client: player chat message.
    async fn send_chat(
        &self,
        request: tonic::Request<PayloadTextMessage>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let zone_id = 0;
        let inner = request.into_inner();
        let mut message = proto::ToClientMessage::default();
        let mut payload = message.text_message_mut();
        payload.set_id(inner.id);
        payload.set_message(inner.message);

        self.senders.iter().for_each(|entry| {
            let sender = entry.value();
            let _ = sender.try_send((BackendServerType::Zone(zone_id), message.clone()));
        });

        Ok(tonic::Response::new(()))
    }

    /// Zone → Client: enemy spawn and despawn.
    async fn enemy_spawn(
        &self,
        request: tonic::Request<PayloadEnemySpawn>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let zone_id = 0;
        let inner = request.into_inner();
        let mut message = proto::ToClientMessage::default();
        let mut payload = message.enemy_spawn_mut();
        payload.set_id(inner.id);
        payload.set_name(inner.name);
        payload.set_position(proto::Vector3::from(inner.position.unwrap_or_default()));

        self.senders.iter().for_each(|entry| {
            let sender = entry.value();
            let _ = sender.try_send((BackendServerType::Zone(zone_id), message.clone()));
        });

        Ok(tonic::Response::new(()))
    }

    /// Zone → Client: enemy spawn and despawn.
    async fn enemy_despawn(
        &self,
        request: tonic::Request<PayloadEnemyDespawn>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let zone_id = 0;
        let inner = request.into_inner();
        let mut message = proto::ToClientMessage::default();
        let mut payload = message.enemy_despawn_mut();
        payload.set_id(inner.id);

        self.senders.iter().for_each(|entry| {
            let sender = entry.value();
            let _ = sender.try_send((BackendServerType::Zone(zone_id), message.clone()));
        });

        Ok(tonic::Response::new(()))
    }

    /// Zone → Client: entity damaged.
    async fn entity_damaged(
        &self,
        request: tonic::Request<PayloadEntityDamaged>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let zone_id = 0;
        let inner = request.into_inner();
        let mut message = proto::ToClientMessage::default();
        let mut payload = message.entity_damaged_mut();
        payload.set_entity_id(inner.entity_id);
        payload.set_damage(inner.damage);

        self.senders.iter().for_each(|entry| {
            let sender = entry.value();
            let _ = sender.try_send((BackendServerType::Zone(zone_id), message.clone()));
        });

        Ok(tonic::Response::new(()))
    }
    /// Zone → Client: player zone enter notification.
    async fn player_enter(
        &self,
        request: tonic::Request<PayloadZoneEnterNotification>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let zone_id = 0;
        let inner = request.into_inner();
        let mut message = proto::ToClientMessage::default();
        let mut payload = message.zone_enter_notification_mut();
        payload.set_id(inner.id);
        payload.set_position(proto::Vector3::from(inner.position.unwrap_or_default()));
        payload.set_username(inner.username);

        self.senders.iter().for_each(|entry| {
            let sender = entry.value();
            let _ = sender.try_send((BackendServerType::Zone(zone_id), message.clone()));
        });

        Ok(tonic::Response::new(()))
    }
    /// Zone → Client: player zone exit notification.
    async fn player_exit(
        &self,
        request: tonic::Request<PayloadZoneExitNotification>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let zone_id = 0;
        let inner = request.into_inner();
        let mut message = proto::ToClientMessage::default();
        let mut payload = message.zone_exit_notification_mut();
        payload.set_id(inner.id);

        self.senders.iter().for_each(|entry| {
            let sender = entry.value();
            let _ = sender.try_send((BackendServerType::Zone(zone_id), message.clone()));
        });

        Ok(tonic::Response::new(()))
    }
}
