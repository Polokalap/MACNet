pub mod database;
mod logger;

use axum::response::{IntoResponse, Response};
use axum::Router;
use axum::routing::get;
use serde::Deserialize;
use sqlx::SqlitePool;
use crate::logger::{info, warn};

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

#[tokio::main]
async fn main() {

    let version = String::from("0.1-PREVIEW");

    if !check_update(version).await {

        warn("A new version is available! You can download it at https://github.com/Polokalap/MACNet/releases/latest").await;

    }

    info("Starting MACNET!").await;

    let state = database::init().await;

    let mut port = 6700;

    let listener = loop {

        match tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await {
            Ok(listener) => break listener,
            Err(_) => port += 1,
        }

    };

    info(format!("Starting server on port {}", port).as_str()).await;

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
