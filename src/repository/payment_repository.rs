use crate::data::payment::Payment;
use tokio_postgres::Error;

pub(crate) async fn save_payments(payment: &Payment, transaction: &tokio_postgres::Transaction<'_>) -> Result<i32, Error> {
    let row = transaction
        .query_one(
            "
            INSERT INTO payments (transaction, request_id, currency, provider, amount, payment_dt, bank, delivery_cost, goods_total, custom_fee)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id;
            ",
            &[
                &payment.transaction,
                &payment.request_id,
                &payment.currency,
                &payment.provider,
                &payment.amount,
                &payment.payment_dt,
                &payment.bank,
                &payment.delivery_cost,
                &payment.goods_total,
                &payment.custom_fee,
            ],
        )
        .await?;

    Ok(row.get(0))
}

pub(crate) async fn get_payment_by_id(payment_id: i32, transaction: &tokio_postgres::Transaction<'_>) -> Result<Payment, Error> {
    let payment_query = "
            SELECT * FROM payments
            WHERE id = $1
        ";

    let payment_row = transaction.query_one(payment_query, &[&payment_id]).await?;

    let payment: Payment = Payment {
        transaction: payment_row.get("transaction"),
        request_id: payment_row.get("request_id"),
        currency: payment_row.get("currency"),
        provider: payment_row.get("provider"),
        amount: payment_row.get("amount"),
        payment_dt: payment_row.get("payment_dt"),
        bank: payment_row.get("bank"),
        delivery_cost: payment_row.get("delivery_cost"),
        goods_total: payment_row.get("goods_total"),
        custom_fee: payment_row.get("custom_fee"),
    };
    Ok(payment)
}