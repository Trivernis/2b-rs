use crate::error::DatabaseResult;
use std::env;

pub mod database;
pub mod entity;
pub mod error;
pub mod models;

pub static VERSION: &str = env!("CARGO_PKG_VERSION");
pub use database::BotDatabase as Database;
use migration::MigratorTrait;
use sea_orm::{ConnectOptions, Database as SeaDatabase, DatabaseConnection};

#[tracing::instrument]
async fn get_connection() -> DatabaseResult<DatabaseConnection> {
    let database_url = env::var("DATABASE_URL").expect("No DATABASE_URL in path");
    tracing::debug!("Establishing database connection...");
    let opt = ConnectOptions::new(database_url);
    let db = SeaDatabase::connect(opt).await?;
    tracing::debug!("Running migrations...");
    migration::Migrator::up(&db, None).await?;
    tracing::debug!("Migrations finished");
    tracing::info!("Database connection initialized");

    Ok(db)
}

pub async fn get_database() -> DatabaseResult<Database> {
    let conn = get_connection().await?;
    Ok(Database::new(conn))
}
