
mod entities;
use sea_orm::*;
// use std::fmt::format;


use futures::executor::block_on;

use entities::{prelude::*, *};

const DATABASE_URL: &str = "mysql://root:123456@150.158.92.84:31229";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    let db_name = "water_seven";

    let db = &match db.get_database_backend(){
        DbBackend::MySql => {

            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("SET GLOBAL time_zone=`+8:00`;"),
            ))
            .await?;

            let url = format!("{}/{}",DATABASE_URL,db_name);
            Database::connect(&url).await?
        }
        DbBackend::Postgres =>db,
        DbBackend::Sqlite => db,
    };

    // // 插入
    // let u = user::ActiveModel {
    //     username: ActiveValue::Set("test5".to_owned()),
    //     password: ActiveValue::Set("xxxx".to_owned()),
    //     ..Default::default()
    // };

    // let res = User::insert(u).exec(db).await?;
    // println!("{:?}",res);

    // 更新
    let u_update = user::ActiveModel {
        id: ActiveValue::Set(1),
        username: ActiveValue::Set("wang".to_owned()),
        ..Default::default()
    };
    let res = u_update.update(db).await?;
    println!("update: {:?}",res);

    // 查找
    let users = User::find().all(db).await?;
    println!("find all {:?}",users);

    
    let u_1: Option<user::Model> = User::find()
        .filter(user::Column::Username.eq("wang"))
        .one(db)
        .await?;

    println!("find username {:?}",u_1);

    // 删除
    let u_2 = user::ActiveModel {
        id: ActiveValue::Set(5), // The primary key must be set
        ..Default::default()
    };
    u_2.delete(db).await?;

    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}