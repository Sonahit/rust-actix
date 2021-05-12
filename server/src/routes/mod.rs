pub mod index;

use actix_web::web;
use actix_web::App;

pub(crate) fn routes<T, U>(app: App<T, U>) -> App<T, U> {
    app
}
