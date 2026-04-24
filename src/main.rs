use dashmap::DashMap;
use etcd_client::{PutOptions, WatchOptions};
use std::{
    env,
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::net::TcpListener;
use tonic::transport::Server;

mod backend;
mod client;
mod generated;
mod logger;
mod network;
mod proto_server;

use backend::backend_client::BackendClient;

use crate::{
    backend::backend_client::BACKEND_CLIENT_INSTANCE,
    generated::{
        proto_client::zone_sync_service_client::ZoneSyncServiceClient,
        proto_server::zone_broadcast_service_server::ZoneBroadcastServiceServer,
    },
    proto_server::zone_sync_service::ZoneBroadcastServiceImpl,
};

#[tokio::main]
async fn main() {
    // log
    logger::init().expect("Failed to initialize logger.");

    dotenvy::dotenv().expect("Failed to load .env file");

    let server_ip = env::var("SERVER_IP").unwrap_or_else(|_| Ipv4Addr::LOCALHOST.to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "50054".into())
        .parse()
        .unwrap_or(50054u16);

    // bind
    let client_listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, 3215))
        .await
        .expect("Failed to bind client listener.");

    // initialize senders
    let senders = Arc::new(DashMap::new());

    // connect to backend server
    let backend_client = BackendClient::new().await;
    BACKEND_CLIENT_INSTANCE
        .set(backend_client)
        .expect("Failed to set backend client instance");

    // etcd client
    let zone_broadcast_server =
        ZoneBroadcastServiceServer::new(ZoneBroadcastServiceImpl::new(senders.clone()));

    // start gRPC server for zone broadcast
    log::info!(
        "Starting gRPC server for zone broadcast on port {}...",
        port
    );
    let grpc_server_endpoint = SocketAddr::new(server_ip.parse().unwrap(), port);
    tokio::spawn(
        Server::builder()
            .add_service(zone_broadcast_server)
            .serve(grpc_server_endpoint),
    );

    // etcdにgatewayの情報を登録
    let etcd_client = etcd_client::Client::connect(
        [format!(
            "{}:2379",
            env::var("ETCD_ADDR").unwrap_or_else(|_| "localhost".into())
        )],
        None,
    )
    .await
    .expect("Failed to connect to etcd server");
    let mut etcd_client_checker = etcd_client.clone();
    let mut etcd_client_keeper = etcd_client.clone();

    // etcdにゾーンの情報を登録し、定期的に更新するタスク
    tokio::spawn(async move {
        let lease = etcd_client_keeper
            .lease_grant(5, None)
            .await
            .expect("Failed to create etcd lease");
        let options = PutOptions::new().with_lease(lease.id());
        etcd_client_keeper
            .put(
                format!("gateways/{}", "zone"),
                format!("{}:{}", server_ip, port),
                Some(options),
            )
            .await
            .expect("Failed to put zone info into etcd");

        loop {
            etcd_client_keeper
                .lease_keep_alive(lease.id())
                .await
                .expect("Failed to keep alive etcd lease");
            log::info!("Updated zone info in etcd with lease ID {}", lease.id());

            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        }
    });

    // etcdからゾーンの情報を取得し、変更を監視するタスク
    tokio::spawn(async move {
        let prefix = "zones/";

        // get existing zones
        {
            let response = etcd_client_checker
                .get(prefix, Some(etcd_client::GetOptions::new().with_prefix()))
                .await
                .expect("Failed to get existing zones from etcd");
            for kv in response.kvs() {
                let zone_id = String::from_utf8_lossy(kv.key())
                    .strip_prefix(prefix)
                    .unwrap_or_default()
                    .parse::<u64>()
                    .unwrap_or_default();
                let url = String::from_utf8_lossy(kv.value())
                    .parse::<String>()
                    .unwrap_or_default();

                log::info!("Found existing zone in etcd: {} -> {}", zone_id, url);

                let Ok(client) = ZoneSyncServiceClient::connect(format!("http://{}", url)).await
                else {
                    log::error!("Failed to connect to zone at {}", url);
                    continue;
                };

                BACKEND_CLIENT_INSTANCE
                    .get()
                    .expect("Failed to get backend client")
                    .add_zone(zone_id, client)
                    .await;

                log::info!("Added zone client: {} -> {}", zone_id, url);
            }
        }
        let config = WatchOptions::new().with_prefix();
        let mut stream = etcd_client_checker
            .watch(prefix, Some(config))
            .await
            .expect("Failed to watch etcd for zone changes");
        loop {
            match stream.message().await {
                Ok(Some(response)) => {
                    for event in response.events() {
                        match event.event_type() {
                            etcd_client::EventType::Put => {
                                if let Some(kv) = event.kv() {
                                    let zone_id = String::from_utf8_lossy(kv.key())
                                        .strip_prefix(prefix)
                                        .unwrap_or_default()
                                        .parse::<u64>()
                                        .unwrap_or_default();
                                    let url = String::from_utf8_lossy(kv.value())
                                        .parse::<String>()
                                        .unwrap_or_default();

                                    let Ok(client) =
                                        ZoneSyncServiceClient::connect(format!("http://{}", url))
                                            .await
                                    else {
                                        log::error!("Failed to connect to zone at {}", url);
                                        continue;
                                    };

                                    BACKEND_CLIENT_INSTANCE
                                        .get()
                                        .expect("Failed to get backend client")
                                        .add_zone(zone_id, client)
                                        .await;

                                    log::info!("Added zone client: {} -> {}", zone_id, url);
                                }
                            }
                            etcd_client::EventType::Delete => {
                                if let Some(kv) = event.kv() {
                                    let zone_id = String::from_utf8_lossy(kv.key())
                                        .strip_prefix(prefix)
                                        .unwrap_or_default()
                                        .parse::<u64>()
                                        .unwrap_or_default();

                                    BACKEND_CLIENT_INSTANCE
                                        .get()
                                        .expect("Failed to get backend client")
                                        .remove_zone(zone_id);
                                    log::info!("Removed zone client: {}", zone_id);
                                }
                            }
                        }
                    }
                }
                _ => {
                    log::error!("Failed to receive etcd watch message");
                }
            }
        }
    });

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
