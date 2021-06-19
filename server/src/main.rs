use actix_web::{App, HttpServer, *};

use std::env;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
#[macro_use]
mod utils;

mod controllers;
mod core;
mod routes;

use routes::routes;

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
        let json_config = web::JsonConfig::default().error_handler(|err, _req| {
            error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
        });
        App::new().configure(routes).app_data(json_config)
    })
    .bind(addr)?
    .run()
    .await
}
