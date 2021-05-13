pub mod index;

use actix_web::web;

pub fn routes(app: &mut web::ServiceConfig) {
    index::routes(app);
}
