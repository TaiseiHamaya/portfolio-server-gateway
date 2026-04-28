use dashmap::DashMap;
use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::net::TcpListener;
use tonic::transport::{Endpoint, Server, channel};

mod backend;
mod client;
mod ec2_helper;
mod etcd_client_helper;
mod generated;
mod logger;
mod network;
mod proto_server;

use backend::backend_client::BackendClient;

use crate::{
    backend::backend_client::BACKEND_CLIENT_INSTANCE,
    generated::proto_server::zone_broadcast_service_server::ZoneBroadcastServiceServer,
    proto_server::zone_sync_service::ZoneBroadcastServiceImpl,
};

#[tokio::main]
async fn main() {
    // log
    logger::init().expect("Failed to initialize logger.");

    dotenvy::dotenv().expect("Failed to load .env file");

    let server_address = ec2_helper::get_local_ip().await;
    let port = env::var("PORT")
        .unwrap_or_else(|_| "50054".into())
        .parse()
        .unwrap_or(50054u16);
    let server_endpoint = SocketAddr::new(IpAddr::V4(server_address), port);

    // bind
    let client_listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, 3215))
        .await
        .expect("Failed to bind client listener.");

    // initialize senders
    let senders = Arc::new(DashMap::new());

    // gRPCサーバーの起動
    let zone_broadcast_server =
        ZoneBroadcastServiceServer::new(ZoneBroadcastServiceImpl::new(senders.clone()));
    tokio::spawn(
        Server::builder()
            .add_service(zone_broadcast_server)
            .serve(server_endpoint),
    );
    log::info!(
        "Starting gRPC server for zone broadcast on port {}...",
        port
    );

    // etcd client
    let etcd_client = etcd_client_helper::create_etcd_client().await;

    // 各種エンドポイントをetcdから取得
    let db_endpoint = // DB
        etcd_client_helper::get_existing_service_endpoint(etcd_client.clone(), "db".to_string())
            .await
            .map(|kv| {
                ["http://", &String::from_utf8_lossy(kv.value())].concat()
            })
            .and_then(|s| Endpoint::from_shared(s).ok())
            .unwrap_or_else(|| Endpoint::from_shared("http://127.0.0.1:50050").expect("Invalid DB fallback URI"));
    log::info!("Record DB Server endpoint: {:?}", db_endpoint);
    let world_endpoint = // World
        etcd_client_helper::get_existing_service_endpoint(etcd_client.clone(), "world".to_string())
            .await
            .map(|kv| {
                ["http://", &String::from_utf8_lossy(kv.value())].concat()
            })
            .and_then(|s| Endpoint::from_shared(s).ok())
            .unwrap_or_else(|| Endpoint::from_shared("http://127.0.0.1:50051").expect("Invalid World fallback URI"));
    log::info!("World Server endpoint: {:?}", world_endpoint);
    let lobby_endpoint = // Lobby
        etcd_client_helper::get_existing_service_endpoint(etcd_client.clone(), "lobby".to_string())
            .await
            .map(|kv| {
                ["http://", &String::from_utf8_lossy(kv.value())].concat()
            })
            .and_then(|s| Endpoint::from_shared(s).ok())
            .unwrap_or_else(|| Endpoint::from_shared("http://127.0.0.1:50052").expect("Invalid Lobby fallback URI"));
    log::info!("Lobby Server endpoint: {:?}", lobby_endpoint);

    // connect
    let backend_client = BackendClient::new(db_endpoint, world_endpoint, lobby_endpoint).await;
    BACKEND_CLIENT_INSTANCE
        .set(backend_client)
        .expect("Failed to set backend client instance");

    // etcdにgatewayの情報を登録し、定期的に更新するタスク
    etcd_client_helper::register_service_endpoint(
        etcd_client.clone(),
        server_endpoint,
        "gateways/00".to_string(),
    )
    .await;

    // etcdからゾーンの情報を取得し、変更を監視するタスク
    etcd_client_helper::watch_changes(etcd_client.clone(), "zones/".to_string(), |event| {
        let Some(kv) = event.kv() else {
            log::error!("Failed to get key-value from etcd event");
            return;
        };
        let zone_id = String::from_utf8_lossy(kv.key())
            .strip_prefix("zones/")
            .unwrap_or_default()
            .parse::<u64>()
            .unwrap_or_default();
        let url = String::from_utf8_lossy(kv.value())
            .parse::<String>()
            .unwrap_or_default();
        match event.event_type() {
            etcd_client::EventType::Put => {
                let client = match channel::Endpoint::from_shared(format!("http://{}", url))
                    .map(|endpoint| endpoint.connect_lazy())
                {
                    Ok(client) => client,
                    Err(e) => {
                        log::error!("Failed to create gRPC client for zone at {}: {}", url, e);
                        return;
                    }
                };

                BACKEND_CLIENT_INSTANCE
                    .get()
                    .expect("Failed to get backend client")
                    .add_zone(zone_id, client);
                log::info!("Added zone client: {} -> {}", zone_id, url);
            }
            etcd_client::EventType::Delete => {
                BACKEND_CLIENT_INSTANCE
                    .get()
                    .expect("Failed to get backend client")
                    .remove_zone(zone_id);
                log::info!("Removed zone client: {}", zone_id);
            }
        }
    })
    .await;

    etcd_client_helper::get_existing_service_endpoint_prefix(
        etcd_client.clone(),
        Some("zones/".to_string()),
        |key, value| {
            let zone_id = key
                .strip_prefix("zones/")
                .unwrap_or_default()
                .parse::<u64>()
                .unwrap_or_default();
            let url = value;

            log::info!("Found existing zone in etcd: {} -> {}", zone_id, url);

            let client = match channel::Endpoint::from_shared(format!("http://{}", url))
                .map(|endpoint| endpoint.connect_lazy())
            {
                Ok(client) => client,
                Err(e) => {
                    log::error!("Failed to create gRPC client for zone at {}: {}", url, e);
                    return;
                }
            };

            BACKEND_CLIENT_INSTANCE
                .get()
                .expect("Failed to get backend client")
                .add_zone(zone_id, client);

            log::info!("Added zone client: {} -> {}", zone_id, url);
        },
    )
    .await;

    loop {
        // wait for an incoming connection
        log::info!("Waiting for client connections...");
        match client_listener.accept().await {
            Ok((socket, addr)) => {
                let packet_sender = client::client_main::client_main(socket, addr, senders.clone());
                senders.insert(addr, packet_sender);
            }
            Err(e) => {
                eprintln!("Failed to accept client connection: {}", e);
            }
        }
    }
}
