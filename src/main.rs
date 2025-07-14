use axum::{Router, routing::get};
use uuid::Uuid;
use tokio::*;

pub mod domains;
pub mod repos;
pub mod routes;
pub mod services;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn handler() -> &'static str {
    "Hello, world"
}
