use crate::data::item::Item;
use crate::repository::item_repository;
use tokio_postgres::Error;
pub(crate) async fn save_items(items: &Vec<Item>, transaction: &tokio_postgres::Transaction<'_>, order_uid: &str) -> Result<(), Error> {
    item_repository::save_items(&items, &transaction, &order_uid).await
}

pub(crate) async fn get_items_by_uid(uid: &str, transaction: &tokio_postgres::Transaction<'_>) -> Result<Vec<Item>, Error> {
    item_repository::get_items_by_uid(transaction, uid).await
}