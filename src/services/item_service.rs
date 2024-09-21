use std::sync::{Arc, Mutex};
use redis::Client;
use tokio::sync::MutexGuard;
use crate::data::item::Item;
use crate::repository::item_repository;
use tokio_postgres::{Error, Transaction};
use crate::repository::item_repository::{get_item_from_cache, save_item_in_cache};
pub(crate) async fn save_items(items: &Vec<Item>, transaction: &tokio_postgres::Transaction<'_>, order_uid: &str) -> Result<(), Error> {
    item_repository::save_items(&items, &transaction, &order_uid).await
}

pub(crate) async fn get_items_by_uid(uid: &str, transaction: &tokio_postgres::Transaction<'_>) -> Result<Vec<Item>, Error> {
    item_repository::get_items_by_uid(transaction, uid).await
}

pub(crate) async fn get_item_by_chrt_id(chrt_id: &str, transaction: &tokio_postgres::Transaction<'_>) -> Result<Item, Error> {
    item_repository::get_item_by_chrt_id(transaction, chrt_id).await
}

pub(crate) async fn get_item(chrt_id: &str, mut client: &MutexGuard<'_ ,Client>, tx: &Transaction<'_>) -> Result<Item, Error> {
    let result_cache = get_item_from_cache(chrt_id, client).await;
    if result_cache.is_err() {
        let item = get_item_by_chrt_id(chrt_id, tx).await?;
        save_item_in_cache(&item, client).await.expect("TODO: panic message");
        return Ok(item);
        // после запроса в бд нужно добавить товар в кэш
    }


    // TODO не забыть поменять
    Ok(result_cache.unwrap())
}