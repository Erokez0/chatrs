use crate::server::timestamp::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum ServerChatMessage {
    #[serde(rename = "text")]
    Text {
        author_id: String,
        content: String,
        sent_at: Timestamp,
    },

    #[serde(rename = "base64_image")]
    Image {
        author_id: String,
        content: Vec<u8>,
        sent_at: Timestamp,
    },
}
