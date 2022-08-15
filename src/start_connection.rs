use crate::server::Server;
use crate::ws::WsConn;
use actix::Addr;
use actix_web::{get, web::Data, web::Path, web::Payload, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use uuid::Uuid;

#[get("/{group_id}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    path: Path<Uuid>,
    srv: Data<Addr<Server>>,
) -> Result<HttpResponse, Error> {
    let ws = WsConn::new(srv.get_ref().clone());

    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}
