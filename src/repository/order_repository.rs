use crate::data::delivery::Delivery;
use crate::data::item::Item;
use crate::data::order::Order;
use crate::data::payment::Payment;
use crate::services::{delivery_service, item_service, payment_service};
use chrono::{ NaiveDateTime, TimeZone, Utc};
use tokio_postgres::{Error, Transaction};

pub(crate) async fn save_order(order: &Order, transaction: &Transaction<'_>, delivery_id: i32, payment_id: i32) -> Result<(), Error> {
    let naive_date_time = order.date_created.naive_utc();
    transaction
        .execute(
            "
            INSERT INTO orders (order_uid, track_number, entry, delivery_id, payment_id,
            locale, internal_signature, customer_id, delivery_service, shardkey,
             sm_id, date_created, oof_shard)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13);
            ",
            &[
                &order.order_uid, &order.track_number, &order.entry, &delivery_id, &payment_id,
                &order.locale, &order.internal_signature, &order.customer_id, &order.delivery_service, &order.shardkey,
                &order.sm_id, &naive_date_time, &order.oof_shard
            ],
        )
        .await?;

    Ok(())
}

pub async fn get_order_by_uid(
    tx: &Transaction<'_>,
    order_uid: &str,
) -> Result<Order, Error> {
    let order_query = "
        SELECT * FROM orders
        WHERE order_uid = $1
    ";
    let order_row = tx.query_one(order_query, &[&order_uid]).await?;

    let delivery: Delivery = delivery_service::get_delivery_by_id(order_row.get("delivery_id"), &tx).await?;
    let payment: Payment = payment_service::get_payment_by_id(order_row.get("payment_id"), &tx).await?;
    let items: Vec<Item> = item_service::get_items_by_uid(order_uid, &tx).await?;

    let naive_date_time: NaiveDateTime = order_row.get("date_created");
    let date_created = Utc.from_utc_datetime(&naive_date_time);

    let order: Order = Order {
        order_uid: String::from(order_uid),
        track_number: order_row.get("track_number"),
        entry: order_row.get("entry"),
        delivery,
        payment,
        items,
        locale: order_row.get("locale"),
        internal_signature: order_row.get("internal_signature"),
        customer_id: order_row.get("customer_id"),
        delivery_service: order_row.get("delivery_service"),
        shardkey: order_row.get("shardkey"),
        sm_id: order_row.get("sm_id"),
        date_created,
        oof_shard: order_row.get("oof_shard"),
    };
    Ok(order)
}