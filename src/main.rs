use std::net::Ipv4Addr;
use tokio::net::TcpListener;

mod client;
mod logger;
mod network;

use logger::init_logger;

use tonic::{Request, Response, Status};

pub mod zone {
    //tonic::include_proto!("");
}

#[tokio::main]
async fn main() {
    // log
    init_logger::init_logger();

    // bind
    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, 3215))
        .await
        .unwrap();

    // Tonic
    //let zone = tonic::transport::Server::builder().add_service().build().;

    // initialize senders
    let mut senders = Vec::new();

    loop {
        // wait for an incoming connection
        let (socket, addr) = listener.accept().await.unwrap();

        let packetSender = client::client_main::client_main(socket, addr);
        senders.push(packetSender);
    }
}
