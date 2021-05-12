use actix_web::{web, App, HttpServer};
use macros::{pipe, pipe_fun};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
mod routes;

use routes::index;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8000));
    HttpServer::new(|| {
        let app = pipe!(
            App::new()
            => [route("/", web::get().to(index::get))]
        );
        app
    })
    .bind(addr)?
    .run()
    .await
}
