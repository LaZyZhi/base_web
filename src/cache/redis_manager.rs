//! Redis连接管理模块
//!
//! 提供Redis连接池的初始化和获取连接的方式。

use anyhow::{anyhow, Result};
use deadpool_redis::{Config, Pool, Runtime};
use tokio::sync::OnceCell;

use crate::config::{get, RedisConfig};

// Redis连接池静态实例
static REDIS_POOL: OnceCell<Pool> = OnceCell::const_new();

/// 使用默认配置初始化 Redis 连接池。
#[allow(dead_code)]
pub fn init_redis_pool() -> Result<()> {
    let config = get().redis.clone();
    let _ = init_redis_pool_with(&config)?;
    Ok(())
}

/// 使用指定配置初始化 Redis 连接池，重复调用会返回已存在的连接池。
pub fn init_redis_pool_with(config: &RedisConfig) -> Result<Pool> {
    if let Some(existing) = REDIS_POOL.get() {
        return Ok(existing.clone());
    }

    let redis_url = build_redis_url(config);
    let cfg = Config::from_url(redis_url);
    let pool = cfg.create_pool(Some(Runtime::Tokio1))?;

    REDIS_POOL
        .set(pool.clone())
        .map_err(|_| anyhow!("Failed to set Redis pool"))?;

    Ok(pool)
}

/// 获取全局 Redis 连接池引用（需先初始化）。
#[allow(dead_code)]
pub fn pool() -> &'static Pool {
    REDIS_POOL
        .get()
        .expect("Redis pool should be initialized before use")
}

fn build_redis_url(config: &RedisConfig) -> String {
    if let Some(password) = &config.password {
        format!(
            "redis://:{}@{}:{}/{}",
            password, config.host, config.port, config.database
        )
    } else {
        format!("redis://{}:{}/{}", config.host, config.port, config.database)
    }
}

/// 获取Redis连接
pub async fn get_redis_connection() -> Result<deadpool_redis::Connection> {
    let conn = pool()
        .get()
        .await
        .map_err(|e| anyhow!("Redis pool not initialized: {}", e))?;
    Ok(conn)
}
