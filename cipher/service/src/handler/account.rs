
use crate::models::{
    user::LoginModel,
    code::Code
    };
use actix_web::{web, App, HttpResponse, HttpServer, Result, http::StatusCode, web::Json};
use crate::db::*;
use crate::redis::AsyncCommands;
use crate::error::ApiError;
use crate::utils::{respond_json,HttpCode};
use base;

 pub async fn verify_email_redis(req: web::Form<LoginModel>, r_pool: web::Data<RPool>) -> Result<String, ApiError> {
    // 获取code值 验证redis
    let mut r_conn = r_pool.get().await.unwrap();
    match r_conn.get::<String,String>(req.code.to_string()).await {
        Ok(c) => {
            Ok(c)
        },
        Err(e) => Err(ApiError::new(800, "Invaild code".into()))
    }
}

pub async fn verify_email(req: web::Form<LoginModel>, pool: web::Data<PgPool>) -> Result<Json<HttpCode<Code>>, ApiError> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    match Code::find_by_code(&conn, &req.code) {
        Ok(o) => respond_json(o, 200),
        Err(e) => Err(ApiError::new(801, "Invaild code".into()))
    }
}

// OwYGRTFcVt  ZR1JURmNWdA$fH1f
#[cfg(test)]
mod test {
    use std::iter;
    use rand::{Rng, thread_rng};
    use rand::distributions::Alphanumeric;

    #[test]
    fn rand_6_int() {
        let mut rng = thread_rng();
        let chars: String = iter::repeat(())
                .map(|()| rng.sample(Alphanumeric))
                .map(char::from)
                .take(7)
                .collect();
        println!("Random chars: {}", chars);
    }
}