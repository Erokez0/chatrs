use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum UserChatMessage {
    #[serde(rename = "text")]
    Text {
        #[serde(default = "default_destination")]
        dest: String,
        content: String,
    },

    #[serde(rename = "base64_image")]
    Image {
        #[serde(default = "default_destination")]
        dest: String,
        content: Vec<u8>,
    },
}

impl Display for UserChatMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserChatMessage::Text { dest, content } => {
                write!(
                    f,
                    "{{\n\tkind: text,\n\tdest: {dest},\n\tcontent: {content}\n}}"
                )
            }
            UserChatMessage::Image { dest, content: _ } => {
                write!(f, "{{\n\tkind: base64_image,\n\tdest: {dest},\n}}")
            }
        }
    }
}

fn default_destination() -> String {
    "public".to_string()
}
