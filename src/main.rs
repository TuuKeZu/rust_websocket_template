mod packets;
mod server;
mod ws;
use server::Server;
mod messages;
mod start_connection;
use actix::Actor;
use actix_web::{App, HttpServer};
use start_connection::start_connection as start_connection_route;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat_server = Server::default().start();

    println!("Server Started!");

    HttpServer::new(move || {
        App::new()
            .service(start_connection_route)
            .data(chat_server.clone())
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await
}
