use serde::{Serialize, Deserialize, Serializer};
use bson::Document;
use bson::oid::ObjectId;
use mongodb::Cursor;

// 序列化Id
pub fn serialize_object_id<S>(oid: &Option<ObjectId>, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
    match oid.as_ref().map(|x| x.to_hex()) {
        Some(v) => s.serialize_str(&v),
        None => s.serialize_none()
    }
}


// // 查询游标转集合特质
// pub trait CursorAsVec {
//     fn as_vec<'a, T: Serialize + Deserialize<'a>>(&mut self) -> Vec<T>;
// }

// // 实现as_vec特质
// impl CursorAsVec for Cursor {
//     fn as_vec<'a, T: Serialize + Deserialize<'a>>(&mut self) -> Vec<T> {
//         self.map(|item| {
//             let doc: Document = item.unwrap();
//             let bson = bson::Bson::Document(doc);
//             return bson::from_bson(bson).unwrap();
//         })
//     }
// }



// 结构体转文档
pub fn struct_to_document<'a, T: Sized + Serialize + Deserialize<'a>>(
    t: &T
) -> Option<Document> {
    let mid: Option<Document> = bson::to_bson(t)
        .ok()
        .map(|x| x.as_document().unwrap().to_owned());

    mid.map(|mut doc| {
        let keys = doc.keys();
        let rm: Vec<String> = keys
            .filter(|k| doc.is_null(k))
            .map(|x| x.to_owned())
            .collect(); // 过滤空值集合
        // remove null value fields
        for x in rm {
            doc.remove(&x);
        }
        doc
    })
}