use crate::controllers;
use actix_web::web;

pub fn routes(app: &mut web::ServiceConfig) {
    app.route("/", web::get().to(controllers::index::get))
        .route("/{name}", web::get().to(controllers::index::get_name))
        .route(
            "/{name}",
            json_post!().to(controllers::index::post_name_json),
        )
        .route(
            "/{name}",
            urlencoded_post!().to(controllers::index::post_name_form),
        );
}
