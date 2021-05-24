use serde_derive::Serialize;
use crate::schema::*;

#[derive(Serialize)]
// login PostModel
pub struct LoginModel{
    email : String
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub created_at: String,
    pub enabled: String,
    pub updated_at: u8,
}

#[derive(Insertable)]
#[table_name = "user"]
pub struct NewUser<'a> {
    pub email: &'a str
}