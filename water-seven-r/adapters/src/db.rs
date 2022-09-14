use derive_new::new;
use sea_orm::{DatabaseConnection, DatabaseTransaction};



pub trait DatabaseService: Send + Sync {
    fn connection(&self) -> DatabaseConnection;
}


#[derive(Clone, new)]
pub struct DatabaseServiceImpl {
    /// データベースコネクション。
    pub conn: DatabaseConnection,
}

impl DatabaseService for DatabaseServiceImpl {

    fn connection(&self) -> DatabaseConnection {
        self.conn.clone()
    }
}