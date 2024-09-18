use crate::data::payment::Payment;
use crate::repository::payment_repository;
use tokio_postgres::Error;

pub(crate) async fn save_payments(payment: &Payment, transaction: &tokio_postgres::Transaction<'_>) -> Result<i32, Error> {
    payment_repository::save_payments(payment, transaction).await
}

pub(crate) async fn get_payment_by_id(payment_id: i32, transaction: &tokio_postgres::Transaction<'_>) -> Result<Payment, Error> {
    payment_repository::get_payment_by_id(payment_id, transaction).await
}