use actix_web::{web,HttpResponse,http::StatusCode,web::{block,Json},Responder,HttpRequest};
use crate::errors::ServiceError;
use crate::model::*;
use bson::{oid::ObjectId,Document};
use futures::future::{ready, Ready};
use crate::db;
use chrono::prelude::*;

use mongodb::{
    Database,
    bson::{doc, Bson},
    options::FindOptions,
};

use futures::stream::StreamExt;

pub fn test() -> HttpResponse {
    HttpResponse::Ok().body("view")
}

// 详情接口
pub fn detail(
    req: web::Query<DetailQuery>
) -> Result<String, ServiceError> {
    let response = req.into_inner();
    let res = serde_json::to_string(&response)?;
    Ok(res)
}

// 详情数据服务
pub async fn find_service(
    detail: web::Json<DetailQuery>,
    db: web::Data<Database>
) -> Result<Json<Vec<Document>>, ServiceError>  {
    let coll = db.collection("dt_view");
    let mut cursor = coll.find(None, None).await?;
    let mut data:Vec<Document> = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                data.push(document)
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(Json(data))
}

// 详情数据服务
pub async fn detail_service(
    query: web::Json<DetailQuery>,
    db: web::Data<Database>
) -> Result<Json<Document>, ServiceError>  {
    let coll = db.collection("dt_view");
    let mut p = "$plates".to_string();
    let qp = query.plate.as_str();
    let key = query.key.as_str();
    let mut day_time = query.day_time.as_str();
    let mut local: DateTime<Utc> = Utc::today().and_hms(0, 0, 0);
    if key == "" {
        return Err(ServiceError::new(StatusCode::BAD_REQUEST, "Must be require key".to_owned()))
    }

    if day_time != "" {
        let local = match day_time.parse::<DateTime<Utc>>() {
            Ok(r) => r,
            Err(e) => return Err(ServiceError::new(StatusCode::BAD_REQUEST, e.to_string()))
        };
    }

    if qp != "" {
        p = p +"."+ qp 
    }
    println!("{}",local);
    let pipeline = vec![
        doc! {
          "$match": {
            "_key": key,
            "day_time": local,
          }
        },
        doc! {
          "$addFields": {
            "plate_map":  p
          }
        },
        doc! {
          "$project": {
            "_id":        0,
            "_key":       0,
            "factory_id": 0,
            "out_put":    0,
            "plates":     0
          }
        },
    ];

    let mut cursor = coll.aggregate(pipeline, None).await.expect("aggregate should succeed");
    let mut data = Document::new();
    while let Some(doc) = cursor.next().await {
       data = doc.unwrap();
      }
    Ok(Json(data))
}


#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use crate::db;
    use crate::futures::StreamExt;
    use bson::{oid::ObjectId,Document};
    use crate::errors::ServiceError;

    #[actix_rt::test]
    async fn get_detail_test() -> Result<(),ServiceError> {
        let coll = db::get_db_test().await.collection("dt_view");
        let mut cursor = coll.find(None, None).await?;
        let mut data:Vec<Document> = Vec::new();
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    data.push(document);
                },
                Err(e) => return Err(e.into()),
            }
        };
        println!("{:?}",data);
        Ok(())
    }


    #[test]
    fn test_string() {
        use chrono::prelude::*;
        let local: DateTime<Utc> = Utc::now();
        println!("{}",Utc::today().and_hms(8, 0, 0))
    }
}