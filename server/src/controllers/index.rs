use actix_web::{web, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

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
pub struct CreatePostDto {
    name: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostDto {
    name: String,
    content: String,
}

pub async fn create_post(body: BodyExtractor<CreatePostDto>) -> impl Responder {
    let CreatePostDto { name, content } = body.into_inner();
    json!(PostDto { name, content }).to_string()
}
