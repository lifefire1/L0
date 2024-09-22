use crate::data::order::Order;
use crate::services::order_service;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use std::sync::Arc;
use log::{error, info};

pub async fn get_order_handler(
    State(state): State<Arc<AppState>>,
    Path(order_uid): Path<String>,
) -> Result<Json<Order>, (StatusCode, String)> {
    info!("{:15} {}","GET /order/:order_id",order_uid);
    let mut client = state.db_client.lock().await;

    let transaction = client.transaction().await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    let result = {
        let tx = transaction;

        let order: Order = order_service::get_order_by_uid(&tx, &order_uid).await.map_err(|e| {
            error!("{:15} order_uid {} {}","GET /order/:order_id failed", order_uid ,e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

        info!("{:15} order_uid: {}","200 (OK) ",order_uid);
        Ok(Json(order))
    };

    drop(client);

    result
}
