use actix_web::{web, Responder};
use serde::Deserialize;

use crate::core::extractors::BodyExtractor;

pub async fn get() -> impl Responder {
    "Hello World"
}

#[derive(Deserialize)]
pub struct GetNameParams {
    pub name: String,
}

pub async fn get_name(params: web::Path<GetNameParams>) -> impl Responder {
    params.name.as_str().to_owned()
}

#[derive(Deserialize)]
pub struct PostName {
    pub name: String,
}

pub async fn post_name(body: BodyExtractor<PostName>) -> impl Responder {
    dbg!("json");
    body.name.as_str().to_owned()
}
