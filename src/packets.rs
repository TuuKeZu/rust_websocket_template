use crate::server::{Board, Turn};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, strum_macros::Display)]
#[serde(tag = "type", content = "data")]
pub enum PacketType {
    Message(String),
    GetBoard(String),
    SetSquare(usize, usize),
    BoardUpdate(Board),
    TurnUpdate(Turn),
    RoleUpdate(Turn),
    Error(u64, String),
}
