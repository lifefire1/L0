use std::sync::Arc;
use axum::extract::{Path, State};
use axum::Json;
use crate::AppState;
use crate::data::order::Order;
use crate::data::delivery::Delivery;
use crate::data::payment::Payment;

pub async fn get_order_handler(State(state): State<Arc<AppState>>, Path(order_id): Path<u64>) -> Json<Order> {
    println!("{}", order_id);
    let order = Order {
        order_uid: String::from("123"),
        track_number: "".to_string(),
        entry: "".to_string(),
        delivery: Delivery {
            name: "".to_string(),
            phone: "".to_string(),
            zip: "".to_string(),
            city: "".to_string(),
            address: "".to_string(),
            region: "".to_string(),
            email: "".to_string(),
        },
        payment: Payment {
            transaction: "".to_string(),
            request_id: "".to_string(),
            currency: "".to_string(),
            provider: "".to_string(),
            amount: 0,
            payment_dt: 0,
            bank: "".to_string(),
            delivery_cost: 0,
            goods_total: 0,
            custom_fee: 0,
        },
        items: vec![],
        locale: "".to_string(),
        internal_signature: "".to_string(),
        customer_id: "".to_string(),
        delivery_service: "".to_string(),
        shardkey: "".to_string(),
        sm_id: 0,
        date_created: Default::default(),
        oof_shard: "".to_string(),
    };
    Json::from(order)
}