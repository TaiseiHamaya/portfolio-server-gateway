use dashmap::DashMap;
use futures::SinkExt;
use std::sync::Arc;

use protobuf::Serialize;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::{RwLock, mpsc};
use tokio_stream::StreamExt;
use tokio_util::{
    bytes::{Buf, Bytes},
    codec::{FramedRead, FramedWrite, LengthDelimitedCodec},
};

use super::proto_handler::handler;
use crate::network::proto::proto::{ToServerMessage, to_client_message};
use crate::network::proto::*;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BackendServerType {
    Login,
    Lobby,
    World,
    Zone(u64), // zone_id
    DB,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClientStatus {
    Connected,
    Authenticated,
    Lobby,
    Routing,
    Zone(u64), // zone_id
    Disconnected,
}

#[allow(dead_code)]
pub struct Client {
    pub status: ClientStatus,
    pub user_id: Option<u64>,
    pub entity_id: Option<u64>,
    pub tx: mpsc::Sender<(BackendServerType, proto::ToClientMessage)>,
}

pub fn client_main(
    socket: tokio::net::TcpStream,
    addr: std::net::SocketAddr,
    senders: Arc<
        DashMap<std::net::SocketAddr, mpsc::Sender<(BackendServerType, proto::ToClientMessage)>>,
    >,
) -> mpsc::Sender<(BackendServerType, proto::ToClientMessage)> {
    log::info!("New connection from {}", addr);
    let (read_socket, write_socket) = socket.into_split();

    let (tx, rx) = mpsc::channel::<(BackendServerType, proto::ToClientMessage)>(128);

    let client = Arc::new(RwLock::new(Client {
        status: ClientStatus::Connected,
        user_id: None,
        entity_id: None,
        tx: tx.clone(),
    }));

    tokio::spawn(reader(
        read_socket,
        addr,
        client.clone(),
        tx.clone(),
        senders,
    ));
    tokio::spawn(writer(write_socket, rx, client.clone()));

    log::info!("Spawned reader and writer tasks for {}", addr);

    return tx;
}

async fn reader(
    read_socket: OwnedReadHalf,
    addr: std::net::SocketAddr,
    client: Arc<RwLock<Client>>,
    tx: mpsc::Sender<(BackendServerType, proto::ToClientMessage)>,
    senders: Arc<
        DashMap<std::net::SocketAddr, mpsc::Sender<(BackendServerType, proto::ToClientMessage)>>,
    >,
) {
    // デフォルトのヘッダ
    // 32bitのメッセージ長ヘッダ
    let mut frame_reader = FramedRead::new(read_socket, LengthDelimitedCodec::new());
    let handler = handler::ClientMessageHandler::new();

    loop {
        match frame_reader.next().await {
            Some(Ok(frame)) => match proto::ToServerMessage::parse(frame.chunk()) {
                Ok(message) => {
                    log::info!(
                        "Received packet from {}: {:?}",
                        addr,
                        message.message_case()
                    );
                    handler.handle_message(message, client.clone()).await;
                }
                Err(e) => log::error!("Failed to parse packet from {}: {}", addr, e),
            },
            Some(Err(e)) => log::error!("Failed to read frame from {}: {}", addr, e),
            None => break, // connection closed
        }
    }

    drop(tx); // close channel
    senders.remove(&addr);

    log::info!("Connection from {} closed", addr);

    let mut message = ToServerMessage::default();
    message.clear_end_game();
    handler.handle_message(message, client.clone()).await;
}

async fn send_message_to_client(
    framed_write: &mut FramedWrite<OwnedWriteHalf, LengthDelimitedCodec>,
    message: proto::ToClientMessage,
) {
    let Ok(buffer) = message.serialize().map(|buffer| Bytes::from(buffer)) else {
        log::error!("Failed to serialize message");
        return;
    };
    let Ok(result) = framed_write.send(buffer).await else {
        log::error!("Failed to send message");
        return;
    };

    result
}

async fn writer(
    write_socket: OwnedWriteHalf,
    mut rx: mpsc::Receiver<(BackendServerType, proto::ToClientMessage)>,
    client: Arc<RwLock<Client>>,
) {
    let mut framed_write = FramedWrite::new(write_socket, LengthDelimitedCodec::new());
    while let Some((server_type, message)) = rx.recv().await {
        log::info!(
            "Received message to send to client: {:?}",
            message.message_case()
        );

        let message_case = message.message_case();
        match message_case {
            to_client_message::MessageCase::LogoutResponse
            | to_client_message::MessageCase::SignupResponse => {
                send_message_to_client(&mut framed_write, message).await;
            }
            to_client_message::MessageCase::EnemySpawn
            | to_client_message::MessageCase::EnemyDespawn
            | to_client_message::MessageCase::TransformSync
            | to_client_message::MessageCase::PlayAction
            | to_client_message::MessageCase::EntityDamaged
            | to_client_message::MessageCase::TextMessage => {
                let zone_id = match server_type {
                    BackendServerType::Zone(id) => id,
                    _ => {
                        // skip if not zone message
                        continue;
                    }
                };
                // if the same zone, send directly
                // otherwise, discard
                if client.read().await.status == ClientStatus::Zone(zone_id) {
                    send_message_to_client(&mut framed_write, message).await;
                }
            }
            to_client_message::MessageCase::LobbyEnterResponse => {
                if client.read().await.status == ClientStatus::Lobby {
                    send_message_to_client(&mut framed_write, message).await;
                }
            }
            to_client_message::MessageCase::StartGameResponse => {
                if client.read().await.status == ClientStatus::Routing {
                    client.write().await.status =
                        ClientStatus::Zone(message.start_game_response().zone_id());
                    send_message_to_client(&mut framed_write, message).await;
                }
            }
            _ => {}
        }
    }
}
