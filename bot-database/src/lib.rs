#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use crate::error::DatabaseResult;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;

pub mod database;
pub mod error;
pub mod models;
pub mod schema;

pub static VERSION: &str = env!("CARGO_PKG_VERSION");
pub use database::Database;

type PoolConnection = Pool<ConnectionManager<PgConnection>>;

embed_migrations!("../bot-database/migrations");

fn get_connection() -> DatabaseResult<PoolConnection> {
    dotenv::dotenv()?;
    let database_url = env::var("DATABASE_URL").expect("No DATABASE_URL in path");
    log::debug!("Establishing database connection...");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().max_size(16).build(manager)?;
    let connection = pool.get()?;
    log::debug!("Running migrations...");
    embedded_migrations::run(&connection)?;
    log::debug!("Migrations finished");
    log::info!("Database connection initialized");

    Ok(pool)
}

pub fn get_database() -> DatabaseResult<Database> {
    let conn = get_connection()?;
    Ok(Database::new(conn))
}
