use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payment {
    pub transaction: String,
    pub request_id: String,
    pub currency: String,
    pub provider: String,
    pub amount: i64,
    pub payment_dt: i64,
    pub bank: String,
    pub delivery_cost: i64,
    pub goods_total: i32,
    pub custom_fee: i64,

}