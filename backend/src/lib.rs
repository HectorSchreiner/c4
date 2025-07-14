#![allow(unused_imports)]
#![allow(dead_code)]

use axum::{Router, routing::get};
use uuid::Uuid;
use tokio::*;

pub mod domains;
pub mod repos;
pub mod routes;
pub mod services;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .merge(routes::init_router());

    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[tauri::command]
pub fn get_backend_info() -> String {
    format!("Backend version: {}", env!("CARGO_PKG_VERSION"))
}

async fn handler() -> &'static str {
    "Hello, world"
}
