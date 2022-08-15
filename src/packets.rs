use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, strum_macros::Display)]
#[serde(tag = "type", content = "data")]
pub enum PacketType {
    Message(String),
    Error(u64, String),
}
