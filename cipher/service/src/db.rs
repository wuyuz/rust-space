use std::ops::Deref;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

pub type PgPool = Pool<ConnectionManager<MysqlConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn get_conn(pool: &PgPool) -> Result<PgPooledConnection, &'static str> {
    pool.get().map_err(|_| "Can't get connection")
}

//https://github.com/actix/examples/blob/a66c05448eace8b1ea53c7495b27604e7e91281c/basics/todo/src/db.rs

