use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchItem {
    pub number: String,
    pub message: String,
    pub status: String,
    pub dnd_active: bool,
    pub network: String,
    pub network_code: String,
}
