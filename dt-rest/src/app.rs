use actix_web::{web,HttpResponse};

use crate::handler::view::*;

#[get("/ping")]
fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}


pub fn services(cfg: &mut web::ServiceConfig) {
    info!("[] Configuring routes...");

    cfg.service(
        web::scope("/api")
        .service(ping)
        .service(
            web::scope("/view")
            .service(
                web::resource("/detail")
                .route(web::get().to(detail_service))
            )
        )
 
    );
}