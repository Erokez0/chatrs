use crate::server::timestamp::Timestamp;
use std::fmt::Display;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Kind {
    #[serde(rename = "image")]
    Image,

    #[serde(rename = "text")]
    Text,
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind_str = match self {
            Self::Image => "image".to_string(),
            Self::Text => "text".to_string(),
        };
        write!(f, "{0}", kind_str)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessageContent {
    pub kind: Kind,
    pub value: String,
}
impl Display for ChatMessageContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\n\tkind: {0},\n\tvalue: {1} }}",
            self.kind, self.value
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChatMessage {
    Text {
        author_id: String,
        content: ChatMessageContent,
        sent_at: Timestamp,

        #[serde(default = "default_destination")]
        dest: String,
    },
    Image {
        author_id: String,
        content: ChatMessageContent,
        sent_at: Timestamp,

        #[serde(default = "default_destination")]
        dest: String,
    },
}

fn default_destination() -> String {
    "public".to_string()
}

impl Display for ChatMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text {
                author_id,
                content,
                sent_at,
                dest,
            } => {
                write!(
                    f,
                    "{{\n\tauthor_id: {0:#?},\n\tcontent: {1},\n\tsent_at: {2},\n\tdest: {3}\n}}",
                    author_id, content, sent_at, dest
                )
            }
            Self::Image {
                author_id,
                content,
                sent_at,
                dest,
            } => {
                write!(
                    f,
                    "{{\n\tauthor_id: {0:#?},\n\tcontent: ,\n\tsent_at: {1},\n\tdest: {2}\n}}",
                    author_id, sent_at, dest
                )
            }
        }
    }
}
