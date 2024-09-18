use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

mod data;
mod handlers;
mod db;
mod services;
mod repository;

use crate::db::connection_to_postgres::connection;
use crate::handlers::get_order_handler::get_order_handler;
use crate::handlers::post_order_handler::post_order_handler;

struct AppState {
    db_client: Arc<Mutex<Client>>, // Wrap Client in Arc<Mutex>
}

#[tokio::main]
async fn main() {
    let client = connection().await.unwrap();
    let state = AppState {
        db_client: Arc::new(Mutex::new(client)), // Wrap in Arc<Mutex>
    };
    let shared_state = Arc::new(state);

    let app = Router::new()
        .route("/order/:order_id", get(get_order_handler))
        .route("/order", post(post_order_handler))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server is running");
    axum::serve(listener, app).await.unwrap();
}



