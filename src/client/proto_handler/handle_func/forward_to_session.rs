use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    backend::backend_client,
    client::client_main::{BackendServerType, Client, ClientStatus},
    generated::proto_client::{
        PayloadLobbyEnterRequest, PayloadLoginRequest, PayloadPlayerCreateRequest,
        PayloadSignupRequest, SessionId,
    },
    network::proto::proto::{self, to_server_message::MessageCase},
};

pub async fn forward_to_session(packet: proto::ToServerMessage, client: Arc<RwLock<Client>>) {
    let message_case = packet.message_case();
    let mut user_db_client = backend_client::BACKEND_CLIENT_INSTANCE
        .get()
        .expect("Uninitialized BackendClient instance")
        .user_db
        .clone();

    match message_case {
        MessageCase::LoginRequest => {
            let session_id = packet.login_request().session_id().to_owned();
            let high = session_id.high();
            let low = session_id.low();

            tokio::spawn(async move {
                log::info!(
                    "Received login request. Client {:016x}{:016x}",
                    session_id.high(),
                    session_id.low()
                );

                let result = match user_db_client
                    .login(PayloadLoginRequest {
                        session_id: Some(SessionId { high, low }),
                    })
                    .await
                {
                    Ok(res) => res,
                    Err(e) => {
                        log::error!("Failed to forward LoginRequest to session: {}", e);
                        return;
                    }
                };

                let is_succeeded = result.get_ref().is_succeeded;
                let user_id = result.get_ref().user_id;
                let session_id = result.into_inner().session_id;
                if is_succeeded && session_id.is_some() {
                    let session_id = session_id.unwrap();
                    log::info!(
                        "Login successful. Session ID: {:016x}{:016x}",
                        session_id.high,
                        session_id.low
                    );

                    client.write().await.user_id = Some(user_id);

                    let enter_result = match backend_client::BACKEND_CLIENT_INSTANCE
                        .get()
                        .unwrap()
                        .lobby
                        .clone()
                        .lobby_enter(PayloadLobbyEnterRequest { user_id })
                        .await
                    {
                        Ok(res) => res,
                        Err(e) => {
                            log::error!("Failed to forward LobbyEnterRequest to lobby: {}", e);
                            return;
                        }
                    };

                    log::info!(
                        "Lobby enter result for user_id {}: is_succeeded={}, character_name={}",
                        user_id,
                        enter_result.get_ref().is_succeeded,
                        enter_result.get_ref().character_name
                    );

                    client.write().await.status = ClientStatus::Lobby;
                    log::info!("Client status updated to Lobby for user_id {}", user_id);
                    let locked_client = client.read().await;

                    let mut response = proto::ToClientMessage::new();
                    response.clear_lobby_enter_response();
                    {
                        let mut lobby_enter_response = response.lobby_enter_response_mut();
                        lobby_enter_response.set_is_succeeded(true);
                        lobby_enter_response
                            .set_character_name(enter_result.get_ref().character_name.to_string());
                    }
                    let _ = locked_client
                        .tx
                        .send((BackendServerType::DB, response))
                        .await;
                } else {
                    log::warn!("Login failed.");
                }
            });
        }
        MessageCase::SignupRequest => {
            let username = packet.signup_request().username().to_string();

            tokio::spawn(async move {
                log::info!(
                    "Received signup request. Client user_id: {:?}",
                    client.read().await.user_id
                );

                let result = match user_db_client
                    .signup(PayloadSignupRequest {
                        username: username.clone(),
                    })
                    .await
                {
                    Ok(res) => res,
                    Err(e) => {
                        log::error!("Failed to forward SignupRequest to session: {}", e);
                        return;
                    }
                };

                let user_id = result.get_ref().user_id;
                let is_succeeded = result.get_ref().is_succeeded;
                let session_id = result.into_inner().session_id;

                let mut record_db = backend_client::BACKEND_CLIENT_INSTANCE
                    .get()
                    .unwrap()
                    .record_player_db
                    .clone();
                let _create_player_result = record_db
                    .create_player(PayloadPlayerCreateRequest { user_id, username })
                    .await;

                if is_succeeded && session_id.is_some() {
                    let session_id = session_id.unwrap();
                    log::info!(
                        "Signup successful. Session ID: {:016x}{:016x}",
                        session_id.high,
                        session_id.low
                    );

                    let locked_client = client.read().await;

                    let mut response = proto::ToClientMessage::new();
                    response.clear_signup_response();
                    {
                        let mut signup_response = response.signup_response_mut();
                        signup_response.set_is_succeeded(true);
                        signup_response.session_id_mut().set_high(session_id.high);
                        signup_response.session_id_mut().set_low(session_id.low);
                    }
                    let _ = locked_client
                        .tx
                        .send((BackendServerType::DB, response))
                        .await;
                } else {
                    log::warn!("Signup failed.");
                }
            });
        }
        _ => {}
    }
}
