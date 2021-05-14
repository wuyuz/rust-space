use mongodb::{Client, Collection, Database};
use lazy_static::lazy_static;
use std::env;
use mongodb::error::Error;

// TODO: possibly remove lazy static reference to simplify
// lazy_static! {
//     pub static ref MONGO: Client = create_mongo_client();
// }


async fn create_mongo_client() -> Client {
    let mongo_connection_string = get_connection_string();
    Client::with_uri_str(&mongo_connection_string).await.expect("Client error")
}

fn get_connection_string() -> String {
    env::var("Mongo_DATABASE_URL").expect("DB Connection error")
}

pub async fn get_db() -> Database {
    // MONGO.database("db_name").collection(coll_name)
    let db_name = env::var("Mongo_DB_NAME").expect("No db_name");
    create_mongo_client().await.database(&db_name)
}

pub async fn get_db_test() -> Database {
    let c = Client::with_uri_str("mongodb://114.215.84.163:27017/?readPreference=primary&appname=MongoDB%20Compass&ssl=false".into()).await.expect("Client error");
    c.database("dt")
}