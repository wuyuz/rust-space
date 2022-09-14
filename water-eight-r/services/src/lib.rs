use std::{net::SocketAddr, sync::Arc};

use actix_web::{
    self,
    web::{self, Data},
    App, HttpServer,
};
use anyhow::anyhow;
use common::ENV_VALUES;

// mod db;
mod handlers;

pub async fn run(address: &SocketAddr) -> anyhow::Result<()> {
    // Web API启动
    HttpServer::new(move || {
        App::new()
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

