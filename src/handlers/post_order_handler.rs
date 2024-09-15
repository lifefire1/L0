use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use crate::AppState;
use crate::data::order::Order;

pub async fn post_order_handler(State(state): State<Arc<AppState>>, order: Json<Order>){
    println!("{:?}", order);

    let delivery_id_row = state.db_client
        .query_one(
            "
            WITH inserted AS (
                INSERT INTO deliveries (name, phone, zip, city, address, region, email)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id
            )
            SELECT id FROM inserted;
            ",
            &[
                &order.delivery.name,
                &order.delivery.phone,
                &order.delivery.zip,
                &order.delivery.city,
                &order.delivery.address,
                &order.delivery.region,
                &order.delivery.email
            ]
        )
        .await;

    println!("{:?}", delivery_id_row.unwrap());
}