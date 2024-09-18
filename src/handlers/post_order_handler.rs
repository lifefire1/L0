use crate::data::order::Order;
use crate::{services, AppState};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

pub async fn post_order_handler(
    State(state): State<Arc<AppState>>,
    Json(order): Json<Order>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("{:?}", order);

    let mut client = state.db_client.lock().await;

    let transaction = client.transaction().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let result = {
        let tx = transaction;

        let delivery_id = services::delivery_service::save_delivery(&order.delivery, &tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let payment_id = services::payment_service::save_payments(&order.payment, &tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        services::order_service::save_order(&order, &tx, delivery_id, payment_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        services::item_service::save_items(&order.items, &tx, &order.order_uid).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok("Order processed successfully".into_response())
    };

    drop(client);

    result
}