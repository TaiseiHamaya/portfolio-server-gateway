use std::{collections::HashMap, future::Future, hash::Hash, pin::Pin, sync::Arc};

use tokio::sync::RwLock;

use crate::{
    client::{
        client_main::Client,
        proto_handler::handle_func::{
            endgame_handler::endgame_handler, forward_to_session::forward_to_session,
            forward_to_zone::forward_to_zone, start_game_handler::start_game_handler,
        },
    },
    network::proto::proto::{ToServerMessage, to_server_message::MessageCase},
};

impl Hash for MessageCase {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (*self as u32).hash(state);
    }
}

type HandlerFuture = Pin<Box<dyn Future<Output = ()> + Send>>;

pub struct ClientMessageHandler {
    handlers: HashMap<
        MessageCase,
        Box<dyn Fn(ToServerMessage, Arc<RwLock<Client>>) -> HandlerFuture + Send + Sync>,
    >,
}

impl ClientMessageHandler {
    pub fn new() -> Self {
        let handlers = HashMap::new();

        let mut result = Self { handlers };

        result.register_handler(MessageCase::LoginRequest, forward_to_session);
        result.register_handler(MessageCase::LogoutRequest, forward_to_session);
        result.register_handler(MessageCase::SignupRequest, forward_to_session);

        result.register_handler(MessageCase::StartGame, start_game_handler);

        result.register_handler(MessageCase::EndGame, endgame_handler);

        result.register_handler(MessageCase::TextMessage, forward_to_zone);
        result.register_handler(MessageCase::PlayAction, forward_to_zone);
        result.register_handler(MessageCase::TransformSync, forward_to_zone);

        result
    }

    fn register_handler<F, Fut>(&mut self, message_case: MessageCase, handler: F)
    where
        F: Fn(ToServerMessage, Arc<RwLock<Client>>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.handlers.insert(
            message_case,
            Box::new(move |msg, client| Box::pin(handler(msg, client))),
        );
    }

    pub async fn handle_message(&self, message: ToServerMessage, client: Arc<RwLock<Client>>) {
        let message_case = message.message_case();

        if let Some(handler) = self.handlers.get(&message_case) {
            handler(message, client.clone()).await;
        }
    }
}
