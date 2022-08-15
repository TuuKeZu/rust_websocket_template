use crate::messages::{Connect, Disconnect, Packet, WsMessage};
use crate::packets::*;
use actix::prelude::{Actor, Context, Handler, Recipient};
use serde_json::Result;
use std::collections::HashMap;
use uuid::Uuid;

type Socket = Recipient<WsMessage>;

#[derive(Debug, Default)]
pub struct Server {
    connections: HashMap<Uuid, User>,
}

impl Server {
    fn emit(&self, data: &str, id: &Uuid) {
        if let Some(player) = self.connections.get(id) {
            let _ = player.socket.do_send(WsMessage(data.to_owned()));
        } else {
            println!("Failed to locate target recipient");
        }
    }

    fn broadcast(&self, data: &str) {
        for id in self.connections.keys() {
            self.emit(data, id);
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub socket: Socket,
}

impl User {
    fn new(id: Uuid, socket: &Socket) -> User {
        User {
            id,
            socket: socket.to_owned(),
        }
    }
}

impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, packet: Disconnect, _: &mut Context<Self>) {
        self.broadcast(&to_json(PacketType::Message(
            "User has left the room".to_string(),
        )));

        if self.connections.contains_key(&packet.id) {
            self.connections.remove(&packet.id);
        }
    }
}

impl Handler<Connect> for Server {
    type Result = ();

    fn handle(&mut self, packet: Connect, _: &mut Context<Self>) -> Self::Result {
        self.connections
            .insert(packet.id, User::new(packet.id, &packet.addr));

        self.emit(
            &to_json(PacketType::Message(format!(
                "You have joined a room with an id of '{}'",
                packet.id
            ))),
            &packet.id,
        );

        self.broadcast(&to_json(PacketType::Message(
            "User has joined the room".to_string(),
        )))
    }
}

impl Handler<Packet> for Server {
    type Result = ();

    fn handle(&mut self, packet: Packet, _ctx: &mut Context<Self>) -> Self::Result {
        println!("{:#?}", packet);

        if let Some(_player) = self.connections.get(&packet.id) {
            let json: Result<PacketType> = serde_json::from_str(&packet.data);

            if let Ok(data) = json {
                match data {
                    PacketType::Message(content) => {
                        self.broadcast(&to_json(PacketType::Message(content)));
                    }
                    PacketType::Error(_, _) => {} // Shouldn't be received from clients
                }
            } else {
                self.emit(
                    &to_json(PacketType::Error(401, "Invalid request".to_string())),
                    &packet.id,
                )
            }
        }
    }
}

pub fn to_json(data: PacketType) -> String {
    serde_json::to_string(&data).unwrap()
}
