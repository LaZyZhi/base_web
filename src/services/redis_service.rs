//! Redis服务层
//!
//! 提供对Redis操作的封装

use anyhow::Result;
use r2d2_redis::redis::Commands;
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
    pub fn set(key: &str, value: &str, ttl: Option<usize>) -> Result<()> {
        let mut conn = redis_manager::get_redis_connection()?;
        if let Some(expire) = ttl {
            let _: () = conn.set_ex(key, value, expire)?;
        } else {
            let _: () = conn.set(key, value)?;
        }
        Ok(())
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
    pub fn get(key: &str) -> Result<Option<String>> {
        let mut conn = redis_manager::get_redis_connection()?;
        let result: Option<String> = conn.get(key)?;
        Ok(result)
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
    pub fn del(key: &str) -> Result<u32> {
        let mut conn = redis_manager::get_redis_connection()?;
        let result: u32 = conn.del(key)?;
        Ok(result)
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
    pub fn exists(key: &str) -> Result<bool> {
        let mut conn = redis_manager::get_redis_connection()?;
        let result: bool = conn.exists(key)?;
        Ok(result)
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
    pub fn expire(key: &str, ttl: usize) -> Result<bool> {
        let mut conn = redis_manager::get_redis_connection()?;
        let result: bool = conn.expire(key, ttl)?;
        Ok(result)
    }
}