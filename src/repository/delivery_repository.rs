use crate::data::delivery::Delivery;
use tokio_postgres::Error;

pub(crate) async fn save_deliveries(delivery: &Delivery, transaction: &tokio_postgres::Transaction<'_>) -> Result<i32, Error> {
    let row = transaction
        .query_one(
            "
            INSERT INTO deliveries (name, phone, zip, city, address, region, email)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id;
            ",
            &[
                &delivery.name,
                &delivery.phone,
                &delivery.zip,
                &delivery.city,
                &delivery.address,
                &delivery.region,
                &delivery.email,
            ],
        )
        .await?;

    Ok(row.get(0))
}

pub(crate) async fn get_delivery_by_id(delivery_id: i32, transaction: &tokio_postgres::Transaction<'_>) -> Result<Delivery, Error> {
    let delivery_query = "
            SELECT * FROM deliveries
            WHERE id = $1
        ";
    let delivery_row = transaction.query_one(delivery_query, &[&delivery_id]).await?;
    let delivery: Delivery = Delivery {
        name: delivery_row.get("name"),
        phone: delivery_row.get("phone"),
        zip: delivery_row.get("zip"),
        city: delivery_row.get("city"),
        address: delivery_row.get("address"),
        region: delivery_row.get("region"),
        email: delivery_row.get("email"),
    };
    Ok(delivery)
}