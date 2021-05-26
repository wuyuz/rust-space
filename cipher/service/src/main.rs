#![allow(unused)]

#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate actix_cors;
extern crate serde;
extern crate dotenv;
extern crate futures;
extern crate failure;
extern crate derive_more;
extern crate uuid;
#[macro_use]
extern crate diesel;
extern crate redis;

mod logger;
mod db;
mod handler;
mod router;
mod models;
mod error;
mod utils;
pub mod schema;

use actix_web::{web, App, HttpServer,http};
use std::{io,env};
use tracing::{instrument, info, error};
use actix_cors::Cors;
use tracing_actix_web::TracingLogger;
// use tera::Tera;

#[actix_web::main]
async fn main() -> io::Result<()> {
    logger::init_telemetry();
    dotenv::dotenv().expect("Failed to read .env file");
    env::set_var("RUST_LOG", "actix_web=debug");
    // 读取环境变量
    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);

    // 加载配置
    let pool = db::init_pool(&db_url).expect("DB pool init error");
    let r_pool= db::get_redis(&redis_url);
    // let teras =Tera::new("templates/**/*").unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .data(pool.clone())
            // .data(teras.clone())
            .data(r_pool.clone())
            .wrap(cors)
            .wrap(TracingLogger)
            .configure(router::services)
    })
    .bind(&app_url)?
    .run()
    .await?;

    Ok(())
}