
pub type RedisPool = bb8::Pool<bb8_redis::RedisConnectionManager>;
pub type RedisConnection<'a> = bb8::PooledConnection<'a, bb8_redis::RedisConnectionManager>;

#[derive(Clone)]
pub struct RedisClient {
    pool: RedisPool,
}

impl RedisClient {
    pub async fn new() -> RedisResult<Self> {
        let config = crate::config::get();
        let redis_config = &config.redis;

        let manager = bb8_redis::RedisConnectionManager::new(redis_config.to_url())?;
        let pool = bb8::Pool::builder().build(manager).await?;

        Ok(Self { pool })
    }
}