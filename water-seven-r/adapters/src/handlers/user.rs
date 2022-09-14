use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sea_orm::{entity::*, query::*};

use crate::db::DatabaseService;
use domains::entity::role::{Model as RM, self, Entity as R};

pub async fn crt_role(
    db_serv: web::Data<dyn DatabaseService>,
    r: web::Form<RM>,
) -> impl Responder {
    let conn = db_serv.connection();
    // let f = r.into_inner();
    // println!("{:?}",f);

    // let rm = role::ActiveModel {
    //     id: Set(f.id.to_owned()),
    //     role_name: Set(f.role_name.to_owned()),
    // };
    // .insert(&conn)
    // .await
    // .expect("could not insert role");

    let re: Option<serde_json::Value> = R::find_by_id(1).into_json().one(&conn).await.expect("xxx");
    
    let res = rm.insert(&conn).await.expect("xxx");


    web::Json(json!({"id":rp}))
}