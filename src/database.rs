use std::str::FromStr;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use crate::AppState;
use crate::logger::info;

pub async fn init() -> AppState {

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

    info("Database initialized!").await;

    AppState { db: pool }

}