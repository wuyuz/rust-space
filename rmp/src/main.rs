#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;


mod config;
mod utils;

// mod middlewares

use crate::utils::csv::{load_csv, walk_csv};
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use actix_casbin::casbin::{
    function_map::key_match2, CachedEnforcer, CoreApi, DefaultModel, MgmtApi, Result,
};
use actix::{Supervisor};
use actix_casbin::CasbinActor;
use actix_casbin_auth::CasbinService;

use diesel_adapter::DieselAdapter;
use std::env;


#[derive(Deserialize)]
pub struct Visitor {
    name: String,
}



#[actix_web::main]
async fn main() -> Result<()>  {
    dotenv::dotenv().expect("Failed to read .env file, please add it");
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool_size: u32 = std::env::var("POOL_SIZE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8);

    let pool = config::db::migrate_and_config_db(&database_url, pool_size);
    let model = DefaultModel::from_file("casbin_config/casbin.conf").await?;
    let adapter = DieselAdapter::new(database_url, pool_size)?;
    let mut casbin_middleware = CasbinService::new(model, adapter).await.unwrap();
    casbin_middleware
        .write()
        .await
        .get_role_manager()
        .write()
        .matching_fn(Some(key_match2), None);

    let share_enforcer = casbin_middleware.get_enforcer();
    let clone_enforcer = share_enforcer.clone();
    let casbin_actor = CasbinActor::<CachedEnforcer>::set_enforcer(share_enforcer)?;
    let started_actor = Supervisor::start(|_| casbin_actor);

    let preset_rules = load_csv(walk_csv("."));
    for mut policy in preset_rules {
        let ptype = policy.remove(0);
        if ptype.starts_with('p') {
            match clone_enforcer.write().await.add_policy(policy).await {
                Ok(_) => info!("Preset policies(p) add successfully"),
                Err(err) => error!("Preset policies(p) add error: {}", err.to_string()),
            };
            continue;
        } else if ptype.starts_with('g') {
            match clone_enforcer
                .write()
                .await
                .add_named_grouping_policy(&ptype, policy)
                .await
            {
                Ok(_) => info!("Preset policies(p) add successfully"),
                Err(err) => error!("Preset policies(g) add error: {}", err.to_string()),
            };
            continue;
        } else {
            unreachable!()
        }
    }

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(started_actor.clone())
            // .wrap(
            //     Cors::new()
            //         .send_wildcard()
            //         .allowed_methods(vec!["GET", "POST", "DELETE"])
            //         .allowed_headers(vec![
            //             http::header::AUTHORIZATION,
            //             http::header::ACCEPT,
            //         ])
            //         .allowed_header(http::header::CONTENT_TYPE)
            //         .max_age(3600)
            //         .finish(),
            // )
            .wrap(middleware::Logger::default())
            .wrap(casbin_middleware.clone())
            // .wrap(crate::middleware::auth::Authentication)
            .route("/", web::get().to(index))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    Ok(())

}

async fn index(me: web::Query<Visitor>) -> impl Responder {
    // if grant(&me.name, "index", "read").await.is_err() {
    //     return HttpResponse::Forbidden().body("Forbidden");
    // };

    HttpResponse::Ok().body("OK")
}
