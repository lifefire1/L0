use crate::data::order::Order;
use crate::services::order_service;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use std::sync::Arc;

pub async fn get_order_handler(
    State(state): State<Arc<AppState>>,
    Path(order_uid): Path<String>,
) -> Result<Json<Order>, (StatusCode, String)> {
    let mut client = state.db_client.lock().await;

    let transaction = client.transaction().await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    let result = {
        let tx = transaction;

        let order: Order = order_service::get_order_by_uid(&tx, &order_uid).await.map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

        Ok(Json(order))
    };

    drop(client);

    result
}
