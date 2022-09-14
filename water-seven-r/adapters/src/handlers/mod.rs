
pub mod user;

use actix_web::{ Responder, web};
use serde_json::json;

/// `Hello world!`を返却する。
pub async fn health() -> impl Responder {
   web::Json(json!({"code":200,"data":"health"}))
}
