//! Redis服务层
//!
//! 提供对Redis操作的封装

use anyhow::Result;
use deadpool_redis::{redis::AsyncCommands, Pool};

use crate::cache::redis_manager;

/// Redis服务结构体
pub struct RedisService;

impl RedisService {
    /// 设置键值对
    ///
    /// # Arguments
    ///
    /// * `key` - 键
    /// * `value` - 值
    /// * `ttl` - 过期时间（秒），None表示不过期
    ///
    /// # Returns
    ///
    /// 返回操作结果
    pub async fn set(key: &str, value: &str, ttl: Option<usize>) -> Result<()> {
        let mut conn = redis_manager::get_redis_connection().await?;
        set_inner(&mut conn, key, value, ttl).await
    }

    /// 设置登录信息，默认过期时间为8小时
    pub async fn set_login(key: &str, value: &str, mut ttl: Option<usize>) -> Result<()> {
        let mut conn = redis_manager::get_redis_connection().await?;
        if ttl == None {
            ttl = Some(60 * 60 * 8 as usize);
        }
        set_inner(&mut conn, key, value, ttl).await
    }

    /// 依赖指定连接池设置键值对，便于注入 AppState。
    pub async fn set_with_pool(pool: &Pool, key: &str, value: &str, ttl: Option<usize>) -> Result<()> {
        let mut conn = pool.get().await?;
        set_inner(&mut conn, key, value, ttl).await
    }

    /// 获取指定键的值
    ///
    /// # Arguments
    ///
    /// * `key` - 键
    ///
    /// # Returns
    ///
    /// 返回键对应的值，如果键不存在则返回None
    pub async fn get(key: &str) -> Result<Option<String>> {
        let mut conn = redis_manager::get_redis_connection().await?;
        get_inner(&mut conn, key).await
    }

    /// 依赖指定连接池获取值。
    pub async fn get_with_pool(pool: &Pool, key: &str) -> Result<Option<String>> {
        let mut conn = pool.get().await?;
        get_inner(&mut conn, key).await
    }

    /// 删除指定键
    ///
    /// # Arguments
    ///
    /// * `key` - 要删除的键
    ///
    /// # Returns
    ///
    /// 返回删除成功的键数量
    pub async fn del(key: &str) -> Result<u32> {
        let mut conn = redis_manager::get_redis_connection().await?;
        del_inner(&mut conn, key).await
    }

    /// 依赖指定连接池删除键。
    pub async fn del_with_pool(pool: &Pool, key: &str) -> Result<u32> {
        let mut conn = pool.get().await?;
        del_inner(&mut conn, key).await
    }

    /// 检查键是否存在
    ///
    /// # Arguments
    ///
    /// * `key` - 要检查的键
    ///
    /// # Returns
    ///
    /// 如果键存在返回true，否则返回false
    pub async fn exists(key: &str) -> Result<bool> {
        let mut conn = redis_manager::get_redis_connection().await?;
        exists_inner(&mut conn, key).await
    }

    pub async fn exists_with_pool(pool: &Pool, key: &str) -> Result<bool> {
        let mut conn = pool.get().await?;
        exists_inner(&mut conn, key).await
    }

    /// 给键设置过期时间
    ///
    /// # Arguments
    ///
    /// * `key` - 键
    /// * `ttl` - 过期时间（秒）
    ///
    /// # Returns
    ///
    /// 如果设置成功返回true，否则返回false
    pub async fn expire(key: &str, ttl: usize) -> Result<bool> {
        let mut conn = redis_manager::get_redis_connection().await?;
        expire_inner(&mut conn, key, ttl).await
    }

    pub async fn expire_with_pool(pool: &Pool, key: &str, ttl: usize) -> Result<bool> {
        let mut conn = pool.get().await?;
        expire_inner(&mut conn, key, ttl).await
    }

}

async fn set_inner(conn: &mut deadpool_redis::Connection, key: &str, value: &str, ttl: Option<usize>) -> Result<()> {
    if let Some(expire) = ttl {
        let _: () = conn.set_ex(key, value, expire as u64).await?;
    } else {
        let _: () = conn.set(key, value).await?;
    }
    Ok(())
}

async fn get_inner(conn: &mut deadpool_redis::Connection, key: &str) -> Result<Option<String>> {
    let result: Option<String> = conn.get(key).await?;
    Ok(result)
}

async fn del_inner(conn: &mut deadpool_redis::Connection, key: &str) -> Result<u32> {
    let result: u32 = conn.del(key).await?;
    Ok(result)
}

async fn exists_inner(conn: &mut deadpool_redis::Connection, key: &str) -> Result<bool> {
    let result: bool = conn.exists(key).await?;
    Ok(result)
}

async fn expire_inner(conn: &mut deadpool_redis::Connection, key: &str, ttl: usize) -> Result<bool> {
    let result: bool = conn.expire(key, ttl as i64).await?;
    Ok(result)
}
