use std::collections::HashMap;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
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

pub async fn index(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let s = if let Some(name) = query.get("name") {
        UserTemplate {
            name,
            text: "Welcome!",
        }
        .render()
        .unwrap()
    } else {
        Index.render().unwrap()
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}