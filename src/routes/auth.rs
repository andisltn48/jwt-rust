use actix_web::{post, HttpResponse, Responder};
use serde_json::json;

use crate::security::jwt::create_jwt;

#[post("/login")]
pub async fn login() -> impl Responder {
    let token = create_jwt().unwrap();
    return HttpResponse::Ok().json(json!({"token": token}));
}