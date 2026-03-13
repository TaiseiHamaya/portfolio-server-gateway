use protobuf::Serialize;
use tokio::sync::mpsc;
use tokio::{
    io::AsyncWriteExt,
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
};
use tokio_stream::StreamExt;
use tokio_util::{
    bytes::Buf,
    codec::{FramedRead, LengthDelimitedCodec},
};

use super::proto_handler::handler;
use crate::network::*;

pub fn client_main(
    socket: tokio::net::TcpStream,
    addr: std::net::SocketAddr,
) -> mpsc::Sender<proto::Packet> {
    log::info!("New connection from {}", addr);
    let (read_socket, write_socket) = socket.into_split();

    let (tx, rx) = mpsc::channel::<proto::Packet>(100);

    tokio::spawn(reader(read_socket, addr, tx.clone()));
    tokio::spawn(writer(write_socket, rx));

    return tx;
}

async fn reader(
    read_socket: OwnedReadHalf,
    addr: std::net::SocketAddr,
    tx: mpsc::Sender<proto::Packet>,
) {
    // デフォルトのヘッダ
    // 32bitのメッセージ長ヘッダ
    let mut frame_reader = FramedRead::new(read_socket, LengthDelimitedCodec::new());
    let handler = handler::ClientMessageHandler::new();

    loop {
        match frame_reader.next().await {
            Some(Ok(frame)) => match proto::Packet::parse(frame.chunk()) {
                Ok(packet) => {
                    // TODO
                }
                Err(e) => log::error!("Failed to parse packet from {}: {}", addr, e),
            },
            Some(Err(e)) => log::error!("Failed to read frame from {}: {}", addr, e),
            None => break, // connection closed
        }
    }

    drop(tx); // close channel

    log::info!("Connection from {} closed", addr);
}

async fn writer(mut write_socket: OwnedWriteHalf, mut rx: mpsc::Receiver<proto::Packet>) {
    // receive packet from channel and write to socket
    while let Some(packet) = rx.recv().await {
        match packet.serialize() {
            Ok(bytes) => {
                if let Err(e) = write_socket.write_all(&bytes).await {
                    log::error!("Failed to write to socket: {}", e);
                    break;
                }
            }
            Err(e) => log::error!("Failed to serialize packet: {}", e),
        }
    }
}
