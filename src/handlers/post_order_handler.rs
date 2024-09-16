use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use chrono::Utc;
use crate::AppState;
use crate::data::delivery::Delivery;
use crate::data::item::Item;
use crate::data::order::Order;
use crate::data::payment::Payment;

pub async fn post_order_handler(State(state): State<Arc<AppState>>, order: Json<Order>) {
    println!("{:?}", order);

    // Сохранение данных о доставке
    let delivery_id = save_deliveries(&order.delivery, &state).await;

    // Сохранение данных о платеже
    let payment_id = save_payments(&order.payment, &state).await;

    // Сохранение данных о заказе
    save_order(&order, &state, delivery_id, payment_id).await;

    // Сохранение данных о товарах
    save_items(&order.items, &state, &order.order_uid).await;
}

async fn save_payments(payment: &Payment, state: &Arc<AppState>) -> i32 {
    let row = state.db_client
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
            ]
        )
        .await
        .expect("Error saving payment");

    row.get(0)
}

async fn save_deliveries(delivery: &Delivery, state: &Arc<AppState>) -> i32 {
    let row = state.db_client
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
            ]
        )
        .await
        .expect("Error saving delivery");

    row.get(0)
}

async fn save_items(items: &Vec<Item>, state: &Arc<AppState>, order_uid: &str) {
    for item in items {
        state.db_client
            .query(
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
                ]
            )
            .await
            .expect("Error saving item");
    }
}

async fn save_order(order: &Order, state: &Arc<AppState>, delivery_id: i32, payment_id: i32) {
    let naive_date_time = order.date_created.naive_utc();
    state.db_client
        .query(
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
            ]
        )
        .await
        .expect("Error saving order");
}
