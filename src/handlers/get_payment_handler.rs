use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use log::info;
use crate::AppState;
use crate::data::payment::Payment;
use crate::services::payment_service::get_payment_by_id;
pub(crate) async fn get_payment(Path(payment_id): Path<String>,
                                State(state): State<Arc<AppState>>) -> Result<Json<Payment>, (StatusCode, String)>{
    info!("GET /payment/{}", payment_id);
    let id = payment_id.parse::<i32>().unwrap();
    let mut client = state.db_client.lock().await;
    let transaction = client.transaction().await.unwrap();
    let payment = get_payment_by_id(id, &transaction).await.map_err(|_| (StatusCode::NOT_FOUND, "Payment not found".to_string()))?;
    info!("200 (OK) GET /payment/{}", payment_id);
    Ok(Json(payment))
}