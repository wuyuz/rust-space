use std::{net::SocketAddr, sync::Arc};

use actix_web::{
    self,
    web::{self, Data},
    App, HttpServer,
};
use anyhow::anyhow;
use sea_orm::Database;
use comm::ENV_VALUES;

mod db;
mod handlers;
use db::{DatabaseService, DatabaseServiceImpl};



pub async fn run(address: &SocketAddr) -> anyhow::Result<()> {
    log::info!("Connecting to database...");
    let conn = Database::connect(&ENV_VALUES.database_url)
    .await
    .map_err(|_| {
        anyhow!("run函数错误，无法架子啊环境变量...")
    });
    let conn = conn.unwrap();
    log::info!("Connected to database...");

    let db_service: Arc<dyn DatabaseService> = Arc::new(DatabaseServiceImpl { conn });
    let db_service: Data<dyn DatabaseService> = Data::from(db_service);

    // Web API启动
    HttpServer::new(move || {
        App::new()
            .app_data(db_service.clone())
            .service(
                web::scope("/api/v1")
                    .service(web::resource("/health")
                    .route(web::get().to(handlers::health)))
            )
            // .service(user_scope())
    })
    .bind(address)?
    .run()
    .await?;

    Ok(())
}



// fn user_scope() -> actix_web::Scope {
//     web::scope("/user")
//         .route("/role", web::post().to(handlers::user::crt_role))
// }