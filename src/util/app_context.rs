use actix_web::middleware::Logger;
use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppContext {
    pub db: DatabaseConnection
}