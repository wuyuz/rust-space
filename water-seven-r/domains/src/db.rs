
use derive_new::new;
use sea_orm::{DatabaseConnection, DatabaseTransaction};

pub trait DatabaseService: Send + Sync {
    fn connection(&self) -> DatabaseConnection;
}
