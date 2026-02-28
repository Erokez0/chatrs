use crate::message::ChatMessage;

mod simple;
pub use simple::*;

pub mod sqlite;

pub trait Storage: Clone + Send + 'static + Sized + Sync {
    type Error;

    async fn find_messages(&self) -> Result<Vec<ChatMessage>, Self::Error>;
    async fn save_message(
        &mut self,
        author_id: &str,
        message: ChatMessage,
    ) -> Result<(), Self::Error>;
    fn new() -> Self;
}
