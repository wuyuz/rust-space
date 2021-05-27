use serde::{Deserialize, Serialize};
use crate::{
    schema::code::{self, dsl::*},
    db::PgPooledConnection,
    diesel::RunQueryDsl,
    error::ApiError,
};
use diesel::prelude::*;
use diesel::insert_or_ignore_into;
use chrono::{NaiveDateTime, Local};


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
    pub email : String,
    pub value : String,
    pub used_at: Option<NaiveDateTime>,
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
    pub fn update_by_code(conn: &PgPooledConnection, c: &str) -> Result<bool,ApiError> {
        let r = diesel::update(code
                .filter(value.eq(c))
                .filter(expired_at.ge(Local::now().naive_local()))
            ).set(used_at.eq(Some(Local::now().naive_local())))
            .execute(conn).unwrap();

        tracing::error!("xx{}",r);
        Ok(true)
    }

    pub fn find_by_code(conn: &PgPooledConnection, c: &str) -> Result<Code,ApiError> {
        code
            .filter(value.eq(c))
            .filter(expired_at.ge(Local::now().naive_local()))
            .get_result::<Code>(conn)
            .map_err(|err| ApiError::new(800, err.to_string()))
    }
}

mod test {
    use chrono::Datelike;

    #[test]
    fn t_find_by_code() {
        use crate::db::get_conn;
        use super::{Code,NewCode};
        use chrono::{NaiveDateTime, Utc,NaiveDate,Local,NaiveTime};
        let c = Local::now().timestamp()+8*3600;
        let conn = get_conn();
        let t = NaiveDateTime::from_timestamp(c+60*30,0);
        let ut = NaiveDateTime::from_timestamp(c,0);

        let n = NewCode {
            email: "xxxs".into(),
            value: "xxxs".into(),
            used_at: Some(ut),
            expired_at: t,
        };

        // println!{"insert :{:?}",n.insert_or_not(&conn)};
        let c = Code::find_by_code(&conn, "xxxs").unwrap();
        println!("{:?}",c)
    }

    #[test]
    fn t_t() {
        use chrono::{Utc, NaiveDateTime,Local};
        use chrono::Timelike;

        let ts = Local::now().timestamp();
        println!("{}",NaiveDateTime::from_timestamp(ts+300+8*3600,0));
    }
}