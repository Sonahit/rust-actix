use actix_web::{guard, web, App, HttpServer};
use macros::{pipe, pipe_fun};
use std::env;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

// Locals
mod routes;
#[macro_use]
mod utils;

use routes::index;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let port_index = args.iter().position(|v| v == "--port" || v == "-p");
    let port = if let Some(index) = port_index {
        args.get(index + 1)
            .unwrap_or(&"8000".to_string())
            .parse::<u16>()
            .unwrap_or(8000_u16)
    } else {
        8000_u16
    };
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port));
    HttpServer::new(|| {
        let app = pipe!(
            App::new()
            => [route("/", web::get().to(index::get))]
            => [route("/{name}", web::get().to(index::get_name))]
            => [route("/{name}", json_post!().to(index::post_name_json))]
            => [route("/{name}", urlencoded_post!().to(index::post_name_form))]
        );
        app
    })
    .bind(addr)?
    .run()
    .await
}
