use crate::messages::{Connect, Disconnect, Packet, WsMessage};
use crate::packets::*;
use actix::prelude::{Actor, Context, Handler, Recipient};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use uuid::Uuid;

type Socket = Recipient<WsMessage>;

#[derive(Debug, Default)]
pub struct Server {
    connections: HashMap<Uuid, User>,
    board: Board,
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

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default)]
pub struct Board {
    rows: [[Square; 3]; 3],
    active: bool,
}

impl Board {
    fn start(&mut self) {
        self.active = true;
    }

    fn get_current_turn(&self) -> Turn {
        if self.get_empty_squares() % 2 == 0 {
            Turn::O
        } else {
            Turn::X
        }
    }

    fn get_empty_squares(&self) -> usize {
        self.rows
            .iter()
            .flatten()
            .filter(|square| square == &&Square::Empty)
            .count()
    }

    fn get_square(&self, row: usize, column: usize) -> Square {
        self.rows[row][column]
    }

    fn set_square(&mut self, row: usize, column: usize, square: Square) {
        self.rows[row][column] = square
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Square {
    Empty,
    X,
    O,
}

impl Default for Square {
    fn default() -> Square {
        Square::Empty
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Turn {
    X,
    O,
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

        if self.board.active {
            self.emit(
                &to_json(PacketType::Error(
                    401,
                    "Game has already started".to_string(),
                )),
                &packet.id,
            );

            self.connections.remove(&packet.id);
            return;
        }

        if self.connections.len() >= 2 {
            self.board.start();

            self.broadcast(&to_json(PacketType::TurnUpdate(
                self.board.get_current_turn(),
            )));

            self.broadcast(&to_json(PacketType::BoardUpdate(self.board)));
        }

        self.emit(
            &to_json(PacketType::RoleUpdate(if self.connections.len() == 1 {
                Turn::X
            } else {
                Turn::O
            })),
            &packet.id,
        );
    }
}

impl Handler<Packet> for Server {
    type Result = ();

    fn handle(&mut self, packet: Packet, _ctx: &mut Context<Self>) -> Self::Result {
        if let Some(_player) = self.connections.get(&packet.id) {
            let json: Result<PacketType> = serde_json::from_str(&packet.data);
            println!("{:#?}", json);

            if let Ok(data) = json {
                match data {
                    PacketType::Message(content) => {
                        self.broadcast(&to_json(PacketType::Message(content)));
                    }
                    PacketType::GetBoard(_) => {
                        self.emit(&to_json(PacketType::BoardUpdate(self.board)), &packet.id)
                    }
                    PacketType::SetSquare(row, column, Square) => {
                        self.board.set_square(row, column, Square);

                        self.broadcast(&to_json(PacketType::BoardUpdate(self.board)));
                        self.broadcast(&to_json(PacketType::TurnUpdate(
                            self.board.get_current_turn(),
                        )));
                    }
                    PacketType::BoardUpdate(_) => {} // Will never be recieved by server
                    PacketType::Error(_, _) => {}    // Will never be recieved by server
                    PacketType::TurnUpdate(_) => {}  // Will never be recieved by server
                    PacketType::RoleUpdate(_) => {}  // Will never be recieved by server
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
