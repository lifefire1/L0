use crate::data::item::Item;
use tokio_postgres::Error;

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