use crate::data::delivery::Delivery;
use crate::data::item::Item;
use crate::data::payment::Payment;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub order_uid: String,
    pub track_number: String,
    pub entry: String,
    pub delivery: Delivery,
    pub payment: Payment,
    pub items: Vec<Item>,
    pub locale: String,
    pub internal_signature: String,
    pub customer_id: String,
    pub delivery_service: String,
    pub shardkey: String,
    pub sm_id: i64,
    pub date_created: DateTime<Utc>,
    pub oof_shard: String,
}