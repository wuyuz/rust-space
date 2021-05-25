use crate::schema::user::{self,dsl::*};
use diesel::prelude::*;
use diesel::insert_or_ignore_into;
use crate::db::PgPooledConnection;
use chrono::{NaiveDateTime, Utc};
use crate::error::ServError;

#[derive(Serialize,Deserialize)]
pub struct LoginModel{
    pub email : String,
    pub code: String
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u64,
    pub email: String,
    pub expired_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "user"]
pub struct NewUser {
    pub email: String
}

impl NewUser {
    pub fn exits(conn: &PgPooledConnection, e: &str) -> bool  {
        user
            .filter(email.eq(e))
            .load::<User>(conn)
            .unwrap()
            .len() == 1   
    }

    pub fn insert_or(&self, conn: &PgPooledConnection) -> bool {
        insert_or_ignore_into(user)
            .values(self)
            .execute(conn)
            .unwrap() == 1
    }
}


mod test {

    #[test]
    fn t_exits() {
        use crate::schema::user;
        use super::{NewUser,User};
        use crate::diesel::RunQueryDsl;
        use crate::db::get_conn;
        let conn = get_conn();
        
        let new_user = NewUser{
            email: "xxxdfa2s".into()
        };

        println!("insert: {}",new_user.insert_or(&conn));
        println!("exit: {}",NewUser::exits(&conn, "xxxxx"))
    }
}