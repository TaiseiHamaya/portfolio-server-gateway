use crate::generated::proto_client;
use crate::network::proto::proto;

macro_rules! impl_proto_vector3_from {
    ($type_a:path) => {
        impl From<$type_a> for proto::Vector3 {
            fn from(value: $type_a) -> Self {
                let mut result = proto::Vector3::new();
                result.set_x(value.x);
                result.set_y(value.y);
                result.set_z(value.z);
                result
            }
        }
        impl From<proto::Vector3> for $type_a {
            fn from(value: proto::Vector3) -> Self {
                $type_a {
                    x: value.x(),
                    y: value.y(),
                    z: value.z(),
                }
            }
        }
    };
}

impl_proto_vector3_from!(proto_client::Vector3);

pub mod broadcast_stream_message_converter {
    use crate::generated::proto_client::{self, broadcast_stream};
    use crate::network::proto::proto::{self, ToClientMessage};

    impl TryFrom<proto_client::EntityData> for proto::EntityData {
        type Error = ();

        fn try_from(value: proto_client::EntityData) -> Result<Self, Self::Error> {
            let mut result = proto::EntityData::new();
            result.set_entity_id(value.entity_id);
            result.set_hp(value.hp);
            let position = value.position.ok_or(())?;
            result.set_position(proto::Vector3::from(position));
            Ok(result)
        }
    }

    impl TryFrom<broadcast_stream::Payload> for ToClientMessage {
        type Error = ();

        fn try_from(value: broadcast_stream::Payload) -> Result<Self, Self::Error> {
            let mut result = ToClientMessage::new();

            match value {
                broadcast_stream::Payload::ClientInitializerData(payload) => {
                    let mut initializer_data = result.client_initializer_data_mut();
                    initializer_data.set_players(payload.players.into_iter().filter_map(|data| {
                        let entity = data
                            .entity_data
                            .as_ref()
                            .and_then(|v| proto::EntityData::try_from(*v).ok())?;
                        let mut player_data = proto::PlayerData::new();
                        player_data.set_name(data.name);
                        player_data.set_entity_data(entity);
                        Some(player_data)
                    }));
                    initializer_data.set_enemies(payload.enemies.into_iter().filter_map(|data| {
                        let entity = data
                            .entity_data
                            .as_ref()
                            .and_then(|v| proto::EntityData::try_from(*v).ok())?;
                        let mut enemy_data = proto::EnemyData::new();
                        enemy_data.set_entity_data(entity);
                        enemy_data.set_enemy_type_id(data.enemy_type_id);
                        Some(enemy_data)
                    }));
                }
                broadcast_stream::Payload::PlayerEnter(payload) => {
                    let mut notification = result.zone_enter_notification_mut();
                    notification.set_id(payload.id);
                    notification.set_username(payload.username);
                    notification
                        .set_position(proto::Vector3::from(payload.position.unwrap_or_default()));
                }
                broadcast_stream::Payload::PlayerExit(payload) => {
                    let mut notification = result.zone_exit_notification_mut();
                    notification.set_id(payload.id);
                }
                broadcast_stream::Payload::EnemySpawn(payload) => {
                    let mut spawn = result.enemy_spawn_mut();
                    spawn.set_enemy_type_id(payload.enemy_type_id);
                    spawn.set_entity_id(payload.entity_id);
                    spawn.set_position(proto::Vector3::from(payload.position.unwrap_or_default()));
                }
                broadcast_stream::Payload::EnemyDespawn(payload) => {
                    let mut despawn = result.enemy_despawn_mut();
                    despawn.set_entity_id(payload.entity_id);
                }
                broadcast_stream::Payload::TransformSync(payload) => {
                    let mut sync = result.transform_sync_mut();
                    sync.set_id(payload.id);
                    sync.set_position(proto::Vector3::from(payload.position.unwrap_or_default()));
                    sync.set_timestamp(payload.timestamp);
                }
                broadcast_stream::Payload::PlayAction(payload) => {
                    let mut action = result.play_action_mut();
                    action.set_id(payload.id);
                    action.set_target_id(payload.target_id);
                    action.set_action_id(payload.action_id);
                    action.set_timestamp(payload.timestamp);
                }
                broadcast_stream::Payload::EntityDamaged(payload) => {
                    let mut damage = result.entity_damaged_mut();
                    damage.set_entity_id(payload.entity_id);
                    damage.set_damage(payload.damage);
                    damage.set_current_hp(payload.current_hp);
                }
                broadcast_stream::Payload::ChatMessage(payload) => {
                    let mut chat = result.text_message_mut();
                    chat.set_id(payload.id);
                    chat.set_message(payload.message);
                }
            }

            Ok(result)
        }
    }
}
