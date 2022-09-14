
use actix_web::{ Responder, web};
use serde_json::json;

/// `Hello world!`
pub async fn health() -> impl Responder {
   log::warn!("visit health...");
   web::Json(json!({"code":200,"data":"health"}))
}