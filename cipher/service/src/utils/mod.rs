pub mod crypto;
pub mod helper;


use crate::error::ApiError;
use actix_web::{body::Body, web::{HttpResponse, Json}};
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug)]
pub struct HttpCode<T>{
    code: u16,
    playload: T
}

impl <T>HttpCode<T> {
    pub fn new(code:u16, data: T) -> HttpCode<T> {
        HttpCode{code:code, playload:data}
    }
}

/// Helper function to reduce boilerplate of an OK/Json response
pub fn respond_json<T>(data: T,code: u16) -> Result<Json<HttpCode<T>>, ApiError>
where
    T: Serialize,
{
    Ok(Json(HttpCode::new(code, data)))
}