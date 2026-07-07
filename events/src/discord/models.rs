use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub content: String,
}
