use std::collections::HashMap;
use actix_web::{web, App, HttpResponse, HttpServer, Result, HttpRequest};
use askama::Template;

#[derive(Template)]
#[template(path = "index/user.html")]
struct UserTemplate<'a> {
    name: &'a str,
    text: &'a str,
}

#[derive(Template)]
#[template(path = "index/index.html")]
struct Index;

pub async fn index(req: HttpRequest) -> Result<HttpResponse> {
    let s = if let Some(token) = req.headers().get("Authorization") {
        // 认证token，验证token，获取user信息
        "未认证".into()
    } else {
        // 登陆页面
        Index.render().unwrap()
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}