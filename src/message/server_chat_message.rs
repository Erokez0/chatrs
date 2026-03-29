use std::fmt::Display;

use crate::server::timestamp::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum ServerChatMessage {
    #[serde(rename = "text")]
    Text {
        id: u128,
        author_id: String,
        content: String,
        sent_at: Timestamp,
        dest: String,
    },

    #[serde(rename = "base64_image")]
    Image {
        id: u128,
        author_id: String,
        content: Vec<u8>,
        sent_at: Timestamp,
        dest: String,
    },
}
impl Display for ServerChatMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text {
                id: _,
                author_id,
                content,
                sent_at,
                dest: _,
            } => {
                write!(
                    f,
                    "{{\n\tauthor_id: {0:#?},\n\tcontent: {1},\n\tsent_at: {2},\n}}",
                    author_id, content, sent_at
                )
            }
            Self::Image {
                id: _,
                author_id,
                content: _,
                sent_at,
                dest: _,
            } => {
                write!(
                    f,
                    "{{\n\tauthor_id: {0:#?},\n\tcontent: ,\n\tsent_at: {1},\n}}",
                    author_id, sent_at
                )
            }
        }
    }
}
