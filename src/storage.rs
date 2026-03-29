use std::fmt::Display;

use crate::message::{ServerChatMessage, UserChatMessage};

mod simple;
pub use simple::*;

pub trait Storage: Clone + Send + 'static + Sized + Sync {
    type Error: Display + Send + Sync;

    async fn find_messages(&self) -> Result<Vec<ServerChatMessage>, Self::Error>;
    fn save_message(
        &mut self,
        author_id: String,
        message: UserChatMessage,
    ) -> impl std::future::Future<Output = Result<ServerChatMessage, Self::Error>> + std::marker::Send;
    fn new() -> Self;
}
