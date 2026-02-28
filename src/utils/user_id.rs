use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Hash, Eq, PartialEq)]
pub struct UserId(String);

impl UserId {
    pub fn new(id: String) -> Self {
        UserId(id)
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.0)
    }
}
