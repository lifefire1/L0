use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use redis::{Client, Commands};
use crate::data::item::Item;
use tokio_postgres::{Error, Transaction};
use crate::error::cache_error::CacheError;
use redis::AsyncCommands;
use tokio::sync::MutexGuard;

const MAX_ZSET_SIZE: isize = 100;
const POPULAR_ITEMS_KEY: &str = "popular_items";

pub(crate) async fn save_items(items: &Vec<Item>, transaction: &tokio_postgres::Transaction<'_>, order_uid: &str) -> Result<(), Error> {
    for item in items {
        transaction
            .execute(
                "
                INSERT INTO items (order_uid, chrt_id, track_number, price, rid, name, sale, size, total_price, nm_id, brand, status)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12);
                ",
                &[
                    &order_uid,
                    &item.chrt_id,
                    &item.track_number,
                    &item.price,
                    &item.rid,
                    &item.name,
                    &item.sale,
                    &item.size,
                    &item.total_price,
                    &item.nm_id,
                    &item.brand,
                    &item.status,
                ],
            )
            .await?;
    }
    Ok(())
}

pub(crate) async fn get_items_by_uid(transaction: &tokio_postgres::Transaction<'_>, order_uid: &str) -> Result<Vec<Item>, Error> {
    let items_query = "
            SELECT * FROM items
            WHERE order_uid = $1
        ";

    let item_rows = transaction.query(items_query, &[&order_uid]).await?;

    let items: Vec<Item> = item_rows.into_iter().map(|row| Item {
        chrt_id: row.get("chrt_id"),
        track_number: row.get("track_number"),
        price: row.get("price"),
        rid: row.get("rid"),
        name: row.get("name"),
        sale: row.get("sale"),
        size: row.get("size"),
        total_price: row.get("total_price"),
        nm_id: row.get("nm_id"),
        brand: row.get("brand"),
        status: row.get("status"),
    }).collect::<Vec<_>>();

    Ok(items)
}

pub(crate) async fn get_item_by_chrt_id(tx: &Transaction<'_>, chrt_id: &str) -> Result<Item, Error> {
    let chrt_id = chrt_id.parse::<i64>().unwrap();
    let query = "SELECT * FROM items WHERE chrt_id = $1 limit 1";

     let row = tx.query_one(query, &[&chrt_id]).await?;

    let item = Item {
        chrt_id: row.get("chrt_id"),
        track_number: row.get("track_number"),
        price: row.get("price"),
        rid: row.get("rid"),
        name: row.get("name"),
        sale: row.get("sale"),
        size: row.get("size"),
        total_price: row.get("total_price"),
        nm_id: row.get("nm_id"),
        brand: row.get("brand"),
        status: row.get("status"),
    };

    Ok(item)
}

pub(crate) async fn save_item_in_cache(item: &Item, client: &MutexGuard<'_,Client>) -> Result<(), redis::RedisError> {
    // Получаем мультиплексированное асинхронное соединение с Redis
    let mut con = client.get_multiplexed_async_connection().await?;

    // Добавляем товар в ZSET
    let _: () = con.zadd(POPULAR_ITEMS_KEY, item.chrt_id, 1).await?;

    // Сохраняем данные товара в HashMap
    let _: () = con.hset_multiple(
        item.chrt_id.to_string(), // ключ HashMap — это chrt_id
        &[
            ("chrt_id", item.chrt_id.to_string().as_str()),
            ("track_number", item.track_number.as_str()),
            ("price", item.price.to_string().as_str()),
            ("rid", item.rid.as_str()),
            ("name", item.name.as_str()),
            ("sale", item.sale.to_string().as_str()),
            ("size", item.size.as_str()),
            ("total_price", item.total_price.as_str()),
            ("nm_id", item.nm_id.to_string().as_str()),
            ("brand", item.brand.as_str()),
            ("status", item.status.to_string().as_str()),
        ]
    ).await?;

    // Устанавливаем TTL для ключа
    con.expire(item.chrt_id.to_string(), 300).await?;

    Ok(())
}

pub(crate) async fn get_item_from_cache(chrt_id: &str, client: &MutexGuard<'_,Client>) -> Result<Item, CacheError> {
    // Получаем асинхронное соединение с Redis
    let mut con = client.get_multiplexed_async_connection().await?;

    // Проверяем, есть ли товар в ZSET
    let rank: Option<isize> = con.zrank(POPULAR_ITEMS_KEY, chrt_id).await?;

    if rank.is_some() {
        con.zincr(POPULAR_ITEMS_KEY, chrt_id, 1).await?;
        // Если товар есть в ZSET, получаем данные товара из HashMap
        let item_data: HashMap<String, String> = con.hgetall(chrt_id).await?;

        let item = Item {
            chrt_id: item_data.get("chrt_id").unwrap_or(&"0".to_string()).parse().unwrap_or(0),
            track_number: item_data.get("track_number").unwrap_or(&"Unknown".to_string()).clone(),
            price: item_data.get("price").unwrap_or(&"0".to_string()).parse().unwrap_or(0),
            rid: item_data.get("rid").unwrap_or(&"Unknown".to_string()).clone(),
            name: item_data.get("name").unwrap_or(&"Unknown".to_string()).clone(),
            sale: item_data.get("sale").unwrap_or(&"0".to_string()).parse().unwrap_or(0),
            size: item_data.get("size").unwrap_or(&"Unknown".to_string()).clone(),
            total_price: item_data.get("total_price").unwrap_or(&"0".to_string()).clone(),
            nm_id: item_data.get("nm_id").unwrap_or(&"0".to_string()).parse().unwrap_or(0),
            brand: item_data.get("brand").unwrap_or(&"Unknown".to_string()).clone(),
            status: item_data.get("status").unwrap_or(&"0".to_string()).parse().unwrap_or(0),
        };

        return Ok(item);
    }

    // Если товара нет в ZSET
    Err(CacheError::CacheMiss)
}