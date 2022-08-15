use actix::prelude::{Message, Recipient};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    pub id: Uuid,
    pub data: String,
}

impl Packet {
    pub fn new(id: Uuid, data: &str) -> Packet {
        Packet {
            id,
            data: String::from(data),
        }
    }
}
