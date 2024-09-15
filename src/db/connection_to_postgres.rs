use tokio_postgres::{Client, NoTls};

pub async  fn connection() -> Result<Client, &'static str> {
    let (db_client, connection) =
    tokio_postgres::connect("host=localhost user=postgres dbname=postgres password=postgres", NoTls)
        .await.map_err(|_| "Error connecting to postgresql server")?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(db_client)
}