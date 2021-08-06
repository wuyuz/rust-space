
mod config;
mod db;

use crate::config::CONFIG;
use anyhow::Result;
use crate::db::{DB,UrlMap};

#[tokio::main]
async fn main() {
    println!("Hello, world! port: {}, databaseurl: {}", CONFIG.port, CONFIG.database.url);

    let db = DB::new().await.unwrap();
    let res = sqlx::query_as::<_,UrlMap>("Select * from url_maps").fetch_all(&db.pool).await;
    println!("result: {:?}", res);
}
