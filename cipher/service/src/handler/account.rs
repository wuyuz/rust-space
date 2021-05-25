
use crate::models::user::LoginModel;
use actix_web::{web, App, HttpResponse, HttpServer, Result, http::StatusCode};
use crate::db::*;
use crate::redis::AsyncCommands;
use crate::error::ApiError;
use base;

 pub async fn verify_email(req: web::Form<LoginModel>, r_pool: web::Data<RPool>) -> Result<String, ApiError> {
    // 获取code值 验证redis
    let mut r_conn = r_pool.get().await.unwrap();
    match r_conn.get::<String,String>(req.code.to_string()).await {
        Ok(c) => {
            // let (f,b)= base::argon(&req.email,req.code.as_bytes()).unwrap();
            // if c == f {
            //     Ok("ok".into())
            // }else{
            //     Err(ApiError::new(800, "Invaild code".into()))
            // }
            Ok(c)
        },
        Err(e) => Err(ApiError::new(800, "Invaild code".into()))
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