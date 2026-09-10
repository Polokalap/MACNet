pub mod database;
mod logger;
pub mod config;
pub mod console;

use axum::response::{IntoResponse, Response};
use axum::Router;
use axum::routing::get;
use serde::Deserialize;
use sqlx::SqlitePool;
use crate::config::Config;
use crate::logger::{info, warn};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
    config: Config,
    start: u128
}

#[tokio::main]
async fn main() {

    let version = String::from("0.1-PREVIEW");

    if !check_update(version).await {

        warn("A new version is available! You can download it at https://github.com/Polokalap/MACNet/releases/latest").await;

    }

    info("Starting MACNet!").await;

    let config = tokio::spawn(config::init());
    let database = tokio::spawn(database::init());

    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let config = config.await.unwrap();

    let state = AppState {
        db: database.await.unwrap(),
        config: config.clone(),
        start: since_epoch.as_millis()
    };

    info("-------------------------------------------------------").await;
    info(format!("Authorization key: {}", config.clone().key()).as_str()).await;

    let mut port = 6700;

    let listener = loop {

        match tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await {
            Ok(listener) => break listener,
            Err(_) => port += 1,
        }

    };

    info(format!("Starting server on port {}", port).as_str()).await;

    tokio::spawn(console::init(state.clone()));

    let now = SystemTime::now();
    let elapsed = now
        .duration_since(start)
        .expect("Time went backwards");

    info(format!("Started up in {}ms!", elapsed.as_millis()).as_str()).await;

    let app = Router::new()
        .route("/", get(home))
        .with_state(state);

    axum::serve(listener, app).await.unwrap();

}

async fn home() -> Response {

    "Test".into_response()

}

#[derive(Debug, Deserialize)]
struct LatestVersion {
    version: String,
}

async fn check_update(current: String) -> bool {

    let url = String::from("https://raw.githubusercontent.com/Polokalap/MACNet/refs/heads/main/latest.toml");

    let body = match reqwest::get(url).await {
        Ok(resp) => match resp.text().await {
            Ok(text) => text,
            Err(_) => return true,
        },
        Err(_) => return true,
    };

    let latest: LatestVersion = match toml::from_str(body.as_str()) {
        Ok(v) => v,
        Err(_) => return true,
    };

    current.eq(&latest.version)

}
