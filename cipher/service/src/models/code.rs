use serde::{Deserialize, Serialize};
use crate::{
    schema::code::{self, dsl::*},
    db::PgPooledConnection,
    diesel::RunQueryDsl,
    error::ApiError,
};
use diesel::prelude::*;
use diesel::insert_or_ignore_into;
use chrono::{NaiveDateTime};


#[derive(Serialize, Queryable, Debug)]
pub struct Code {
    pub id :    u64,
    pub email : String,
    pub value : String,
    pub used_at: Option<NaiveDateTime>,
    pub created_at : NaiveDateTime,
    pub expired_at : NaiveDateTime,
}


#[derive(Insertable)]
#[table_name="code"]
pub struct NewCode {
    pub id :    u64,
    pub email : String,
    pub value : String,
    pub used_at: NaiveDateTime,
    pub created_at : NaiveDateTime,
    pub expired_at : NaiveDateTime,
}


impl NewCode {
    pub fn insert_or_not(&self, conn: &PgPooledConnection) -> bool {
        insert_or_ignore_into(code)
            .values(self)
            .execute(conn)
            .unwrap() == 1
    }
}


impl Code {
    pub fn find_by_code(conn: &PgPooledConnection, c: &str) ->Result<Code, ApiError> {
        code
            .filter(value.eq(c))
            .get_result::<Code>(conn)
            .map_err(|err| ApiError::new(800, err.to_string()))
    }
}



mod test {

    #[test]
    fn t_find_by_code() {
        use crate::db::get_conn;
        use super::Code;

        let conn = get_conn();
        let c = Code::find_by_code(&conn, "xxx").unwrap();
        println!("{:?}",c)

    }
}