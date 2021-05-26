use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport,message::IntoBody,message::header}; 
  
use askama::Template;
use actix_web::{web, App, HttpResponse, HttpServer, Result, web::{Json,block}};
use crate::utils::{respond_json,HttpCode};
use tracing::{error};
use crate::error::ApiError;
use serde::Deserialize;
use base;
use crate::db::*;
use crate::models::{
    user::{NewUser,User},
    code::{NewCode,Code}
};
use chrono::{NaiveDateTime,Local};
use redis::AsyncCommands;
use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

#[derive(Template)]
#[template(path = "email/send_email.html")]
pub struct EMAILSEND<'a> {
    code: &'a str,
    email: &'a str,
}
#[derive(Deserialize)]
pub struct UserParam<'a> {
    pub email: &'a str
}

// send function
fn send_emails<T: IntoBody>(to: &str, body: T) -> Result<Json<HttpCode<String>>, ApiError>  {
    let email = Message::builder()
            .from("cipher <cipher_cn@163.com>".parse().unwrap())
            .to(to.parse().unwrap())
            .subject("Cipher 登陆验证")
            .header(header::ContentType::TEXT_HTML)
            .body(body)
            .unwrap();

    let creds = Credentials::new("cipher_cn@163.com".to_string(), "UMXIEEGNJWQEVYAA".to_string());
    let mailer = SmtpTransport::relay("smtp.163.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => respond_json("send successfully".into(), 200),
        Err(e) => {
            error!("{}",e.to_string());
            respond_json(e.to_string(), 400)
        },
    } 
}

pub async fn email_send_redis(p: web::Form<NewUser>, pool: web::Data<PgPool>, r_pool: web::Data<RPool>) -> Result<Json<HttpCode<String>>, ApiError> {
    // 查询数据库
    let conn = pool.get().expect("couldn't get db connection from pool");
    p.insert_or(&conn);

    // 生成code， code为盐
    let mut rng = thread_rng();
    let code: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(10)
        .collect();
    // 写入redis, code+email -> argon
    let (f, b) = base::argon(&p.email, code.as_bytes()).unwrap();

    let mut r_conn = r_pool.get().await.unwrap();
    let s: String = r_conn.set_ex(&code, f,500).await.unwrap();

    // 渲染html
    let r =  EMAILSEND{
            code: &code,
            email: &p.email
        }.render().unwrap();
    
    // 发送
    let ret = block(move || send_emails(&p.email, r)).await?;
    Ok(ret)
}

pub async fn email_send(p: web::Form<NewUser>, pool: web::Data<PgPool>) -> Result<Json<HttpCode<String>>, ApiError> {
    // 查询数据库
    let conn = pool.get().expect("couldn't get db connection from pool");
    p.insert_or(&conn)   ;

    let mut rng = thread_rng();
    let code: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(10)
        .collect();

    let new_code = NewCode {
        email: (&p.email).to_string(),
        value: (&code).to_string(),
        used_at: None,
        expired_at: NaiveDateTime::from_timestamp( Local::now().timestamp()+8*3600+60*30,0)
    };

    // 更新code
    new_code.insert_or_not(&conn);

    // 渲染html
    let r =  EMAILSEND{
            code: &code,
            email: &p.email
        }.render().unwrap();
    
    // 发送
    let ret = block(move || send_emails(&p.email, r)).await?;
    Ok(ret)
}


#[cfg(test)]
mod test {  

    #[test]
    fn t_send() {
        use rand_core::{RngCore, OsRng};
        let mut code = [0u8; 10];
        OsRng.fill_bytes(&mut code);
        println!("{}",String::from_utf8(code.to_vec()).unwrap());
    }

    #[test]
    fn email_send_1() {
        use lettre::transport::smtp::authentication::Credentials;
        use lettre::{Message, SmtpTransport, Transport}; 
        use diesel::mysql::MysqlConnection;
        use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
        type PgPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;  

        let email = Message::builder()
            .from("cipher <cipher_cn@163.com>".parse().unwrap())
            .to("Hei <1417506149@qq.com>".parse().unwrap())
            .subject("Happy new year")  
            .body(String::from("Be happy!"))
            .unwrap();

        let creds = Credentials::new("cipher_cn@163.com".to_string(), "UMXIEEGNJWQEVYAA".to_string());
        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay("smtp.163.com")
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        } 
    }
   
    #[test]
    fn create_user() {
        use crate::models::user::{NewUser,User};
        use crate::diesel::RunQueryDsl;
        use crate::schema::user;
        use crate::db::get_conn;
        let conn = get_conn();

        let new_post = NewUser { 
            email: "xxxxx".into()
         };

        let u = diesel::insert_into(user::table)
                .values(&new_post)
                .execute(&conn);
        println!("{:?}",u)
    }

    #[actix_rt::test]
    async fn t_redis() {
        use dotenv::dotenv;
        use crate::db::*;
        use std::env;
        use redis::AsyncCommands;
        dotenv().ok();
        let r_url = env::var("REDIS_URL").expect("DATABASE_URL must be set");

        let mut r_conn = get_redis(&r_url).get().await.unwrap();
        let s: String = r_conn.set_ex("xxx", "dsdf",40).await.unwrap();
        println!("{:?}",s);
    }
}