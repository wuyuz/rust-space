
use crate::model::LoginModel;
use actix_web::{web, App, HttpResponse, HttpServer, Result};

// 登陆验证： 邮箱
pub fn login_with_email(user: web::Form<LoginModel>) {
    // 获取到邮箱

    // 判断是否有用户，新用户提示，第一次密码为主密码，不可修改

    // 生成验证码，存储redis，发送邮件，以user_id为key
}


pub fn verify_email() {

    // 获取code值

    // 验证redis

    // 成功返回user页面，添加auth header

}


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