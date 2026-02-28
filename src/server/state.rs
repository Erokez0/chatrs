use std::collections::HashMap;

use axum::extract::ws::Message;
use tokio::sync::mpsc;

use crate::{
    storage::Storage,
    utils::{IdentifierGenerator, UserId},
};

#[derive(Clone)]
pub struct ServerState<S, G>
where
    S: Storage,
    G: IdentifierGenerator,
{
    connections: HashMap<UserId, mpsc::UnboundedSender<Message>>,
    pub storage: S,
    pub generator: G,
}

impl<S, G> ServerState<S, G>
where
    S: Storage,
    G: IdentifierGenerator,
{
    pub fn new() -> Self {
        Self {
            connections: HashMap::<UserId, mpsc::UnboundedSender<Message>>::new(),
            storage: S::new(),
            generator: G::new(),
        }
    }

    pub fn add_connection(&mut self, id: UserId, connection: mpsc::UnboundedSender<Message>) {
        self.connections.insert(id, connection);
    }

    pub fn connections(&self) -> &HashMap<UserId, mpsc::UnboundedSender<Message>> {
        &self.connections
    }
}
