use actix_web::web;
use actix_web::Responder;
use serde::Deserialize;

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

pub async fn post_name_json(body: web::Json<PostName>) -> impl Responder {
    dbg!("json");
    body.name.as_str().to_owned()
}

pub async fn post_name_form(body: web::Form<PostName>) -> impl Responder {
    dbg!("form");
    body.name.as_str().to_owned()
}
