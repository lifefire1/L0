use axum::Json;
use crate::data::order::Order;

pub async fn post_order_handler(order: Json<Order>){
    println!("{:?}", order);
}