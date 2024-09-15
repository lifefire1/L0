use axum::Router;
use axum::routing::{get, post};

mod data;
mod handlers;
use crate::handlers::get_order_handler::get_order_handler;
use crate::handlers::post_order_handler::post_order_handler;

#[tokio::main]
async fn main() {
    let routers = Router::new()
        .route("/order/:order_id", get(get_order_handler))
        .route("/order",post(post_order_handler));

    let listener = tokio::net
        ::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, routers).await.unwrap();
}
