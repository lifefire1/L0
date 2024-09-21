use redis::Commands;

pub (crate) async fn connect_to_redis() -> Result<redis::Client, redis::RedisError>{
    let client = redis::Client::open("redis://127.0.0.1/")?;
    Ok(client)
}