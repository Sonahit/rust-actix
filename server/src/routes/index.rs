use crate::controllers;
use actix_web::web;

pub fn routes(app: &mut web::ServiceConfig) {
    app.route("/", web::get().to(controllers::index::get))
        .route("/{name}", web::get().to(controllers::index::get_name))
        .route("/posts", web::post().to(controllers::index::create_post));
}
