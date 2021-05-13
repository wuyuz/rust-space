pub mod response;
pub mod mongo;

use mongo::*;
use bson::oid::ObjectId;
use serde::{Serialize, Deserialize, Serializer};

#[derive(Deserialize, Serialize, Debug)]
pub struct DetailQuery {
    pub key: String,

    day_time: String,

    #[serde(default)]
    plate: String,
}

// 数据模型
#[derive(Deserialize, Serialize, Debug)]
pub struct DataModle {
    #[serde(serialize_with="serialize_object_id", rename="_id")]
    id: Option<ObjectId>,

    day_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<String>,
    plates: String,
}


