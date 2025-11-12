//! Redis连接管理模块
//!
//! 提供Redis连接池的初始化和获取连接的方法

use std::sync::OnceLock;
use anyhow::Result;
use r2d2::{Pool, PooledConnection};
use r2d2_redis::RedisConnectionManager;

use crate::config::get;

// Redis连接池静态实例
static REDIS_POOL: OnceLock<Pool<RedisConnectionManager>> = OnceLock::new();

/// 初始化Redis连接池
pub fn init_redis_pool() -> Result<()> {
    let config = get().redis.clone();
    let redis_url = if let Some(password) = &config.password {
        format!(
            "redis://:{}@{}:{}/{}",
            password, config.host, config.port, config.database
        )
    } else {
        format!(
            "redis://{}:{}/{}",
            config.host, config.port, config.database
        )
    };

    let manager = RedisConnectionManager::new(redis_url)?;
    
    let pool = r2d2::Pool::builder()
        .max_size(config.max_active)
        .min_idle(Some(config.min_idle))
        .build(manager)?;

    REDIS_POOL.set(pool).map_err(|_| anyhow::anyhow!("Failed to set Redis pool"))?;
    Ok(())
}

/// 获取Redis连接
pub fn get_redis_connection() -> Result<PooledConnection<RedisConnectionManager>> {
    let pool = REDIS_POOL.get().ok_or_else(|| anyhow::anyhow!("Redis pool not initialized"))?;
    pool.get().map_err(|e| anyhow::anyhow!("Failed to get Redis connection: {}", e))
}