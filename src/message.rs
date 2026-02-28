use std::fmt::Display;

use axum::extract::ws;
use serde::{Deserialize, Serialize};

mod chat_message;
pub use chat_message::*;

mod user_chat_message;
pub use user_chat_message::*;

mod server_chat_message;
pub use server_chat_message::*;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "kind")]
pub enum RequestMessage {
    UserJoined { user_id: String, at: u64 },
    ServerGaveId { user_id: String, at: u64 },
    UserLeft { user_id: String, at: u64 },
    AllMessages { messages: Vec<ServerChatMessage> },
    UserSentMessage { message: UserChatMessage },
    ServerResponseStatus { ok: bool, text: String },
}

impl RequestMessage {
    fn kind(&self) -> String {
        match self {
            &Self::UserJoined { user_id: _, at: _ } => "user_join".to_string(),
            &Self::ServerGaveId { user_id: _, at: _ } => "server_gave_id".to_string(),
            &Self::UserLeft { user_id: _, at: _ } => "user_left".to_string(),
            &Self::AllMessages { messages: _ } => "all_messages".to_string(),
            &Self::UserSentMessage { message: _ } => "user_sent_message".to_string(),
            &Self::ServerResponseStatus { ok: _, text: _ } => "server_response_status".to_string(),
        }
    }
}

impl Display for RequestMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n\tkind: {0},\n", self.kind())
    }
}

impl TryInto<ws::Message> for RequestMessage {
    fn try_into(self) -> Result<ws::Message, serde_json::Error> {
        match serde_json::to_vec(&self) {
            Ok(data) => Ok(ws::Message::Binary(data.into())),
            Err(err) => {
                log::error!("error serializing message:\nmessage: ${self}\nerror: {err}");
                Err(err)
            }
        }
    }
    type Error = serde_json::Error;
}

impl From<ws::Message> for RequestMessage {
    fn from(value: ws::Message) -> Self {
        let bytes = value.into_data();
        match serde_json::from_slice(&bytes) {
            Ok(msg) => msg,
            Err(err) => RequestMessage::ServerResponseStatus {
                ok: false,
                text: err.to_string(),
            },
        }
    }
}
