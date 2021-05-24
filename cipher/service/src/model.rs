use serde_derive::Serialize;
use crate::schema::user::{self,dsl::*};
use diesel::prelude::*;
use crate::db::PgPooledConnection;
use chrono::{NaiveDateTime, Utc};

#[derive(Serialize)]
pub struct LoginModel{
    email : String
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub email: String,
    pub expired_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "user"]
pub struct NewUser<'a> {
    pub email: &'a str
}

impl <'a>NewUser<'a> {
    pub fn exits(conn: &PgPooledConnection)  {
        // user.find(1).get_result::<User>(conn).is_ok()
    }
}