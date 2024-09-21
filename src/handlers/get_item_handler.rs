use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use crate::AppState;
use crate::data::item::Item;
use crate::services::item_service::{get_item};
pub(crate) async fn get_item_handler(State(state): State<Arc<AppState>>,
                                     Path(chrt_id): Path<String>) -> Result<Json<Item>, (StatusCode, String)> {

    let redis_client = state.redis_client.lock().await;
    let mut db_client = state.db_client.lock().await;
    let tx = db_client.transaction().await.unwrap();
    let item = get_item(&chrt_id,&redis_client , &tx).await.unwrap();
    drop(tx);
    Ok(Json(item))
}