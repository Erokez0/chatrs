use std::collections::HashMap;

use crate::message::ChatMessage;

use super::Storage;

#[derive(Clone)]
pub struct SimpleStorage {
    messages: HashMap<String, ChatMessage>,
}

impl SimpleStorage {
    pub fn new() -> SimpleStorage {
        SimpleStorage {
            messages: HashMap::<String, ChatMessage>::new(),
        }
    }
}
impl Storage for SimpleStorage {
    type Error = ();

    async fn find_messages(&self) -> Result<Vec<ChatMessage>, Self::Error> {
        Ok(self.messages.values().cloned().collect())
    }

    async fn save_message(
        &mut self,
        author_id: &str,
        message: ChatMessage,
    ) -> Result<(), Self::Error> {
        log::debug!("{message}");
        self.messages.insert(author_id.to_owned(), message);
        Ok(())
    }

    fn new() -> Self {
        Self::new()
    }
}
