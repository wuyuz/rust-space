use actix_web::{web,HttpResponse};
use tracing::info;
use crate::handler::*;

#[get("/ping")]
fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

pub fn services(cfg: &mut web::ServiceConfig) {
    info!("[+] Configuring routes...");
    // api
    cfg.service(
        web::scope("/api")
        .service(ping)
    );
    // web
    cfg.service(
        web::scope("web")
        .service(
            web::scope("/email")
            .service(
                web::resource("/send")
                .route(web::get().to(email::email_send_user))
            )
        )
    );
}