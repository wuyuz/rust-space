use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};   
use askama::Template;
use actix_web::{web, App, HttpResponse, HttpServer, Result};

#[derive(Template)]
#[template(path = "email/send_email.html")]
struct EMAILSEND<'a> {
    code: &'a str,
    email: &'a str,
}

pub async fn email_send_user() -> Result<HttpResponse> {
    let r =  EMAILSEND{
            code: "1231212",
            email: "1417506149@qq.com"
        }.render().unwrap();
    return Ok(HttpResponse::Ok().content_type("text/html").body(r))
}



#[cfg(test)]
mod test {  
     
    #[test]
    fn email_send_1() {
        use lettre::transport::smtp::authentication::Credentials;
        use lettre::{Message, SmtpTransport, Transport};   
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
}