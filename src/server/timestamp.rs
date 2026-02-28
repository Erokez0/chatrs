use std::{
    fmt::Display,
    time::{self, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Timestamp(u64);

impl Timestamp {
    pub fn now() -> u64 {
        time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.0)
    }
}
