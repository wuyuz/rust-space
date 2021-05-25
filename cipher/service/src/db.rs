use std::ops::Deref;

use mobc_redis::RedisConnectionManager;
use mobc_redis::redis;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

pub type RPool = mobc::Pool<RedisConnectionManager>;
pub type PgPool = Pool<ConnectionManager<MysqlConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn get_conn() -> PgPooledConnection {
        use dotenv::dotenv;
        use std::env;
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        Pool::builder().build(manager).unwrap().get().unwrap()
}

//https://github.com/actix/examples/blob/a66c05448eace8b1ea53c7495b27604e7e91281c/basics/todo/src/db.rs


// redis
pub fn get_redis(h : &str) -> RPool {
    let client = redis::Client::open(h).unwrap();
    let manager = RedisConnectionManager::new(client);
    RPool::builder().max_open(100).build(manager)
}