use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum UserChatMessage {
    #[serde(rename = "text")]
    Text {
        #[serde(default = "default_destination")]
        dest: Option<String>,
        content: String,
    },

    #[serde(rename = "base64_image")]
    Image {
        #[serde(default = "default_destination")]
        dest: Option<String>,
        content: Vec<u8>,
    },
}

fn default_destination() -> Option<String> {
    Some("public".to_string())
}
