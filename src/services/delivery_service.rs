use crate::data::delivery::Delivery;
use crate::repository;
use tokio_postgres::Error;

pub(crate) async fn save_delivery(delivery: &Delivery, transaction: &tokio_postgres::Transaction<'_>) -> Result<i32, Error> {
    repository::delivery_repository::save_deliveries(&delivery, &transaction).await
}

pub(crate) async fn get_delivery_by_id(delivery_id: i32, transaction: &tokio_postgres::Transaction<'_>) -> Result<Delivery, Error> {
    repository::delivery_repository::get_delivery_by_id(delivery_id, transaction).await
}