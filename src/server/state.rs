use std::collections::HashMap;
use std::sync::Arc; // Add this

use axum::extract::ws::Message;
use tokio::sync::{RwLock, RwLockReadGuard, mpsc};

use crate::{storage::Storage, utils::IdentifierGenerator};

#[derive(Clone)] // Now this works because Arc is always Clone
pub struct ServerState<S, G>
where
    S: Storage + Clone, // S and G likely need Clone too for Axum State
    G: IdentifierGenerator + Clone,
{
    // Wrap the RwLock in an Arc
    pub connections: Arc<RwLock<HashMap<String, mpsc::UnboundedSender<Message>>>>,
    pub storage: S,
    pub generator: G,
}

impl<S, G> ServerState<S, G>
where
    S: Storage + Clone,
    G: IdentifierGenerator + Clone,
{
    pub fn new() -> Self {
        Self {
            // Initialize with Arc::new
            connections: Arc::new(RwLock::new(HashMap::new())),
            storage: S::new(),
            generator: G::new(),
        }
    }

    pub async fn add_connection(&mut self, connection: mpsc::UnboundedSender<Message>) -> String {
        let id = self.generator.new_id();
        self.connections
            .write()
            .await
            .insert(id.clone(), connection);
        id
    }

    pub async fn connections(
        &self,
    ) -> RwLockReadGuard<'_, HashMap<String, mpsc::UnboundedSender<Message>>> {
        self.connections.read().await
    }

    pub async fn remove_connection(&self, id: String) {
        self.connections.write().await.remove(&id);
    }
}

