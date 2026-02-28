// use sqlx::sqlite;
//
// use crate::{message::chat, storage::Storage};
//
// #[derive(Clone)]
// pub struct SqliteStorage {
//     pool: sqlite::SqlitePool,
// }
//
// impl Storage for SqliteStorage {
//     type Error = sqlite::SqliteError;
//
//     async fn find_messages(&self) -> Result<&Vec<chat::ChatMessage>, Self::Error> {
//         sqlx::query_as::<_, Vec<chat::ChatMessage>>("SELECT * FROM messages")
//             .fetch_all(self.pool)
//             .await
//     }
//
//     async fn save_message(
//         &mut self,
//         message: crate::message::chat::ChatMessage,
//     ) -> Result<(), Self::Error> {
//         sqlx::query("INSERT INTO TABLE messages VALUES ($1)")
//             .bind(message)
//             .execute(self.pool)
//             .await//     }
// }
