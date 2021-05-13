
#![allow(unused_must_use)]
#![allow(unused)]

#[macro_use]
extern crate bson;
#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate actix_cors;
extern crate env_logger;
extern crate serde;
extern crate dotenv;
extern crate futures;
extern crate failure;
extern crate derive_more;
extern crate uuid;
extern crate diesel;

mod logger;
mod errors;
mod model;
mod app;
mod handler;
mod db;

use actix_web::{http, HttpServer, App,dev::ServiceResponse,HttpResponse,Error};
use futures::FutureExt;
use std::{io, env};
use std::default::Default;
use actix_cors::Cors;
use crate::logger::*;
use crate::errors::ServiceError;
// use crate::errors::ApiError;



#[actix_web::main]
async fn main() -> io::Result<()> {
    // 加载配置文件，设置日志
    dotenv::dotenv().expect("Failed to read .env file");
    env::set_var("RUST_LOG", "actix_web=debug");
    init_logger();

    // 读取环境变量
    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);

    // 加载配置
    let db = db::get_db().await;

    HttpServer::new(move || {
        App::new()
            .data(
                // 配置全局的json提取器的配置，以及报错响应
                actix_web::web::JsonConfig::default()
                .limit(4096)
                .error_handler(  
                    |err, _| {
                        ServiceError::new(http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).error_response()
                    }
                )
            )
            .wrap(Cors::default() // allowed_origin return access-control-allow-origin: * by default
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_origin("http://localhost:3000")
                .send_wildcard()
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600))
            .data(db.clone())
            .wrap(actix_web::middleware::Logger::default())
            // If you want to use yew-address-book-frontend, please comment auth_middleware wrapping code
            
            // 服务加载
            .configure(app::services)
        })
    .bind(&app_url)?
    .run()
    .await
}