use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use tokio_postgres::{Error};
use crate::AppState;
use crate::data::delivery::Delivery;
use crate::data::item::Item;
use crate::data::order::Order;
use crate::data::payment::Payment;


pub async fn post_order_handler(
    State(state): State<Arc<AppState>>,
    Json(order): Json<Order>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("{:?}", order);

    let mut client = state.db_client.lock().await;

    let transaction = client.transaction().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let result = {
        let tx = transaction;

        let delivery_id = save_deliveries(&order.delivery, &tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let payment_id = save_payments(&order.payment, &tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        save_order(&order, &tx, delivery_id, payment_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        save_items(&order.items, &tx, &order.order_uid).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok("Order processed successfully".into_response())
    };

    drop(client);

    result
}
async fn save_payments(payment: &Payment, transaction: &tokio_postgres::Transaction<'_>) -> Result<i32, Error> {
    let row = transaction
        .query_one(
            "
            INSERT INTO payments (transaction, request_id, currency, provider, amount, payment_dt, bank, delivery_cost, goods_total, custom_fee)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id;
            ",
            &[
                &payment.transaction,
                &payment.request_id,
                &payment.currency,
                &payment.provider,
                &payment.amount,
                &payment.payment_dt,
                &payment.bank,
                &payment.delivery_cost,
                &payment.goods_total,
                &payment.custom_fee,
            ],
        )
        .await?;

    Ok(row.get(0))
}

async fn save_deliveries(delivery: &Delivery, transaction: &tokio_postgres::Transaction<'_>) -> Result<i32, Error> {
    let row = transaction
        .query_one(
            "
            INSERT INTO deliveries (name, phone, zip, city, address, region, email)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id;
            ",
            &[
                &delivery.name,
                &delivery.phone,
                &delivery.zip,
                &delivery.city,
                &delivery.address,
                &delivery.region,
                &delivery.email,
            ],
        )
        .await?;

    Ok(row.get(0))
}

async fn save_items(items: &Vec<Item>, transaction: &tokio_postgres::Transaction<'_>, order_uid: &str) -> Result<(), Error> {
    for item in items {
        transaction
            .execute(
                "
                INSERT INTO items (order_uid, chrt_id, track_number, price, rid, name, sale, size, total_price, nm_id, brand, status)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12);
                ",
                &[
                    &order_uid,
                    &item.chrt_id,
                    &item.track_number,
                    &item.price,
                    &item.rid,
                    &item.name,
                    &item.sale,
                    &item.size,
                    &item.total_price,
                    &item.nm_id,
                    &item.brand,
                    &item.status,
                ],
            )
            .await?;
    }
    Ok(())
}

async fn save_order(order: &Order, transaction: &tokio_postgres::Transaction<'_>, delivery_id: i32, payment_id: i32) -> Result<(), Error> {
    let naive_date_time = order.date_created.naive_utc();
    transaction
        .execute(
            "
            INSERT INTO orders (order_uid, track_number, entry, delivery_id, payment_id,
            locale, internal_signature, customer_id, delivery_service, shardkey,
             sm_id, date_created, oof_shard)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13);
            ",
            &[
                &order.order_uid, &order.track_number, &order.entry, &delivery_id, &payment_id,
                &order.locale, &order.internal_signature, &order.customer_id, &order.delivery_service, &order.shardkey,
                &order.sm_id, &naive_date_time, &order.oof_shard
            ],
        )
        .await?;

    Ok(())
}
