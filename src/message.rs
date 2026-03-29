use std::fmt::Display;

use axum::extract::ws;
use serde::{Deserialize, Serialize};

mod user_chat_message;
pub use user_chat_message::*;

mod server_chat_message;
pub use server_chat_message::*;

use crate::server::timestamp::Timestamp;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "kind")]
pub enum RequestMessage {
    UserJoined { user_id: String, at: Timestamp },
    ServerGaveId { user_id: String, at: Timestamp },
    UserLeft { user_id: String, at: Timestamp },
    AllMessages { messages: Vec<ServerChatMessage> },
    UserSentMessage { message: UserChatMessage },
    ServerSentMessage { message: ServerChatMessage },
    ServerResponseStatus { ok: bool, text: String },
}

impl RequestMessage {
    pub fn kind(&self) -> String {
        match *self {
            Self::UserJoined { user_id: _, at: _ } => "user_join".to_string(),
            Self::ServerGaveId { user_id: _, at: _ } => "server_gave_id".to_string(),
            Self::UserLeft { user_id: _, at: _ } => "user_left".to_string(),
            Self::AllMessages { messages: _ } => "all_messages".to_string(),
            Self::UserSentMessage { message: _ } => "user_sent_message".to_string(),
            Self::ServerResponseStatus { ok: _, text: _ } => "server_response_status".to_string(),
            Self::ServerSentMessage { message: _ } => "server_sent_message".to_string(),
        }
    }
    pub fn allowed_from_client(&self) -> bool {
        match *self {
            Self::UserJoined { user_id: _, at: _ } => false,
            Self::ServerGaveId { user_id: _, at: _ } => false,
            Self::UserLeft { user_id: _, at: _ } => false,
            Self::AllMessages { messages: _ } => false,
            Self::UserSentMessage { message: _ } => true,
            Self::ServerResponseStatus { ok: _, text: _ } => false,
            Self::ServerSentMessage { message: _ } => true,
        }
    }
}

impl Display for RequestMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = self.kind();
        match self.clone() {
            RequestMessage::UserJoined { user_id, at } => write!(
                f,
                "{{\n\tkind: {kind},\n\tuser_id: {user_id}\n\tat: {at}\n}}"
            ),
            RequestMessage::ServerGaveId { user_id, at } => write!(
                f,
                "{{\n\tkind: {kind},\n\tuser_id: {user_id}\n\tat: {at}\n}}"
            ),
            RequestMessage::UserLeft { user_id, at } => write!(
                f,
                "{{\n\tkind: {kind},\n\tuser_id: {user_id}\n\tat: {at}\n}}"
            ),
            RequestMessage::AllMessages { messages } => {
                let concat_msgs = |mut acc: String, msg: &ServerChatMessage| {
                    acc.push_str(&format!("\n\t{msg},"));
                    acc
                };
                let messages_str = messages.iter().fold(String::new(), concat_msgs);
                write!(f, "{{\n\tkind: {kind},\n\tmessages: {messages_str}\n}}")
            }
            RequestMessage::UserSentMessage { message } => {
                write!(f, "{{\n\tkind: {kind},\n\tmessage: {message}\n}}")
            }
            RequestMessage::ServerSentMessage { message } => {
                write!(f, "{{\n\tkind: {kind},\n\tmessage: {message}\n}}")
            }
            RequestMessage::ServerResponseStatus { ok, text } => {
                write!(f, "{{\n\tkind: {kind},\n\tok: {ok},\n\ttext: {text}\n}}")
            }
        }
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

impl TryFrom<ws::Message> for RequestMessage {
    type Error = serde_json::Error;
    fn try_from(value: ws::Message) -> Result<RequestMessage, Self::Error> {
        let bytes = value.into_data();
        serde_json::from_slice(&bytes)
    }
}
