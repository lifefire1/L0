use crate::data::order::Order;
use crate::repository::order_repository;
use tokio_postgres::Error;

pub(crate) async fn save_order(order: &Order, transaction: &tokio_postgres::Transaction<'_>, delivery_id: i32, payment_id: i32) -> Result<(), Error> {
    order_repository::save_order(order, transaction, delivery_id, payment_id).await
}

pub(crate) async fn get_order_by_uid(transaction: &tokio_postgres::Transaction<'_>, uid: &str) -> Result<Order, Error> {
    order_repository::get_order_by_uid(transaction, uid).await
}