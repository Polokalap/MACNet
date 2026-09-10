use std::str::FromStr;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use crate::AppState;
use crate::logger::info;

pub async fn init() -> SqlitePool {

    info("Initializing database...").await;

    let database_url = "sqlite://data.db";

    let options = SqliteConnectOptions::from_str(database_url)
        .unwrap()
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .expect("Failed to connect to the database");

    sqlx::query("CREATE TABLE IF NOT EXISTS players (id TEXT PRIMARY KEY, name TEXT, trust INTEGER)")
        .execute(&pool)
        .await
        .expect("Failed to create table");

    info("Database initialized!").await;

    pool

}