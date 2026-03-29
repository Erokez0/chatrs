use std::collections::HashMap;

use crate::message::{ServerChatMessage, UserChatMessage};
use crate::server::timestamp::Timestamp;

use super::Storage;

#[derive(Clone)]
pub struct SimpleStorage {
    messages: HashMap<String, ServerChatMessage>,
}

impl SimpleStorage {
    pub fn new() -> SimpleStorage {
        SimpleStorage {
            messages: HashMap::<String, ServerChatMessage>::new(),
        }
    }
}
impl Storage for SimpleStorage {
    type Error = String;

    async fn find_messages(&self) -> Result<Vec<ServerChatMessage>, Self::Error> {
        Ok(self.messages.values().cloned().collect())
    }

    async fn save_message(
        &mut self,
        author_id: String,
        message: UserChatMessage,
    ) -> Result<ServerChatMessage, Self::Error> {
        log::debug!("{message}");
        let id: u128 = (self.messages.len() + 2) as u128;
        let server_message: ServerChatMessage = match message {
            UserChatMessage::Text { dest, content } => ServerChatMessage::Text {
                id,
                author_id,
                content,
                sent_at: Timestamp::now(),
                dest,
            },
            UserChatMessage::Image { dest, content } => ServerChatMessage::Image {
                id,
                author_id,
                content,
                sent_at: Timestamp::now(),
                dest,
            },
        };

        Ok(server_message)
    }

    fn new() -> Self {
        Self::new()
    }
}
