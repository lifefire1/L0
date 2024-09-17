use std::sync::Arc;
use axum::extract::{Path, State};
use axum::Json;
use axum::http::StatusCode;
use chrono::{DateTime, NaiveDateTime, Utc};
use crate::AppState;
use crate::data::order::Order;
use crate::data::delivery::Delivery;
use crate::data::payment::Payment;
use crate::data::item::Item;

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

        let order_query = "
            SELECT * FROM orders
            WHERE order_uid = $1
        ";
        let order_row = tx.query_one(order_query, &[&order_uid]).await.map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

        let delivery_id: i32 = order_row.get("delivery_id");
        let payment_id: i32 = order_row.get("payment_id");

        let delivery_query = "
            SELECT * FROM deliveries
            WHERE id = $1
        ";
        let delivery_row = tx.query_one(delivery_query, &[&delivery_id]).await.map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
        let delivery = Delivery {
            name: delivery_row.get("name"),
            phone: delivery_row.get("phone"),
            zip: delivery_row.get("zip"),
            city: delivery_row.get("city"),
            address: delivery_row.get("address"),
            region: delivery_row.get("region"),
            email: delivery_row.get("email"),
        };


        let payment_query = "
            SELECT * FROM payments
            WHERE id = $1
        ";
        let payment_row = tx.query_one(payment_query, &[&payment_id]).await.map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
        let payment = Payment {
            transaction: payment_row.get("transaction"),
            request_id: payment_row.get("request_id"),
            currency: payment_row.get("currency"),
            provider: payment_row.get("provider"),
            amount: payment_row.get("amount"),
            payment_dt: payment_row.get("payment_dt"),
            bank: payment_row.get("bank"),
            delivery_cost: payment_row.get("delivery_cost"),
            goods_total: payment_row.get("goods_total"),
            custom_fee: payment_row.get("custom_fee"),
        };


        let items_query = "
            SELECT * FROM items
            WHERE order_uid = $1
        ";
        let item_rows = tx.query(items_query, &[&order_uid]).await.map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
        let items: Vec<Item> = item_rows.into_iter().map(|row| Item {
            chrt_id: row.get("chrt_id"),
            track_number: row.get("track_number"),
            price: row.get("price"),
            rid: row.get("rid"),
            name: row.get("name"),
            sale: row.get("sale"),
            size: row.get("size"),
            total_price: row.get("total_price"),
            nm_id: row.get("nm_id"),
            brand: row.get("brand"),
            status: row.get("status"),
        }).collect();

        let naive_date_time: NaiveDateTime = order_row.get("date_created");
        let date_created = DateTime::<Utc>::from_utc(naive_date_time, Utc);

        let order = Order {
            order_uid,
            track_number: order_row.get("track_number"),
            entry: order_row.get("entry"),
            delivery,
            payment,
            items,
            locale: order_row.get("locale"),
            internal_signature: order_row.get("internal_signature"),
            customer_id: order_row.get("customer_id"),
            delivery_service: order_row.get("delivery_service"),
            shardkey: order_row.get("shardkey"),
            sm_id: order_row.get("sm_id"),
            date_created,
            oof_shard: order_row.get("oof_shard"),
        };


        Ok(Json(order))
    };

    drop(client);

    result
}
