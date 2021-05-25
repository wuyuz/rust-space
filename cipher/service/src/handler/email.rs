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
use crate::model::{NewUser,User};


#[derive(Template)]
#[template(path = "email/send_email.html")]
pub struct EMAILSEND<'a> {
    code: &'a str,
    email: &'a str,
}

#[derive(Deserialize)]
pub struct EmailData {
    email: String,
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

pub async fn email_send_user(p: web::Form<EmailData>, pool : web::Data<PgPool>) -> Result<Json<HttpCode<String>>, ApiError> {
    // 查询数据库
    let conn = pool.get().expect("couldn't get db connection from pool");


    // 生成code， code为盐
    let code = base::utils::helper::rand_6_int(10);
    // 写入redis

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
        use crate::schema::user;
        use crate::model::{NewUser,User};
        use diesel::mysql::MysqlConnection;
        use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
        use crate::diesel::RunQueryDsl;

        pub type PgPool = Pool<ConnectionManager<MysqlConnection>>;
        type PgPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        println!("{}",database_url);
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let conn = Pool::builder().build(manager).unwrap().get().unwrap();

        let new_post = NewUser { 
            id : b"1",
            email: "xxxxx"
         };

        let u = diesel::insert_into(user::table)
                .values(&new_post)
<<<<<<< HEAD
                .execute(&conn)
                .expect("Error saving new post");
=======
                .execute(&conn);

>>>>>>> 49a46c4bcc39a37337a93ea9adc58052e7781b50
        println!("{:?}",u)
    }
}