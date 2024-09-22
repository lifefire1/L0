use axum::routing::{get, post};
use axum::Router;
use env_logger::Env;
use log::info;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client;

mod data;
mod handlers;
mod db;
mod services;
mod repository;
mod error;

use crate::db::connection_to_postgres::connection;
use crate::db::connection_to_redis::connect_to_redis;
use crate::handlers::get_item_handler::get_item_handler;
use crate::handlers::get_order_handler::get_order_handler;
use crate::handlers::post_order_handler::post_order_handler;

struct AppState {
    db_client: Arc<Mutex<Client>>,
    redis_client: Arc<Mutex<redis::Client>>,
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let client = connection().await.unwrap();
    info!("{:20}", "connect to postgres successfully");
    let redis_client = connect_to_redis().await.unwrap();
    info!("{:20}", "connect to redis successfully");
    let client_r = Arc::new(Mutex::new(redis_client));
    let state = AppState {
        db_client: Arc::new(Mutex::new(client)), // Wrap in Arc<Mutex>
        redis_client: client_r,
    };
    let shared_state = Arc::new(state);

    let app = Router::new()
        .route("/order/:order_id", get(get_order_handler))
        .route("/order", post(post_order_handler))
        .route("/item/:chrt_id", get(get_item_handler))
        .with_state(shared_state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("{:30}","Registered routes:");
    info!("{:30}","Created GET /order/:order_id");
    info!("{:30}","Created POST /order");
    info!("{:30}","Created GET /item/:chrt_id");
    info!("{:20} {:10}", "Server is listening on ", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}



