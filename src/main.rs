pub mod database;
mod logger;

use axum::response::{IntoResponse, Response};
use axum::Router;
use axum::routing::get;
use sqlx::SqlitePool;
use crate::logger::info;

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

#[tokio::main]
async fn main() {

    let version = "0.1-PREVIEW";

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

async fn check_update() -> bool {

    false

}
