//! Redis服务测试
//!
//! 测试Redis异步连接池的功能

#[cfg(test)]
mod tests {
    use super::super::redis_service::RedisService;
    use crate::config;

    #[tokio::test]
    async fn test_redis_connection() {
        // 初始化配置
        config::init();
        
        // 初始化Redis连接池
        let init_result = crate::cache::redis_manager::init_redis_pool();
        assert!(init_result.is_ok(), "Failed to initialize Redis pool");

        // 测试设置键值对
        let set_result = RedisService::set("test_key", "test_value", Some(60)).await;
        assert!(set_result.is_ok(), "Failed to set key-value pair");

        // 测试获取键值对
        let get_result = RedisService::get("test_key").await;
        assert!(get_result.is_ok(), "Failed to get key-value pair");
        assert_eq!(get_result.unwrap(), Some("test_value".to_string()), "Value mismatch");

        // 测试检查键是否存在
        let exists_result = RedisService::exists("test_key").await;
        assert!(exists_result.is_ok(), "Failed to check key existence");
        assert!(exists_result.unwrap(), "Key should exist");

        // 测试设置过期时间
        let expire_result = RedisService::expire("test_key", 30).await;
        assert!(expire_result.is_ok(), "Failed to set expiration");
        assert!(expire_result.unwrap(), "Expiration should be set");

        // 测试删除键
        let del_result = RedisService::del("test_key").await;
        assert!(del_result.is_ok(), "Failed to delete key");
        assert_eq!(del_result.unwrap(), 1, "Should delete one key");

        // 验证键已被删除
        let get_result_after_del = RedisService::get("test_key").await;
        assert!(get_result_after_del.is_ok(), "Failed to get key after deletion");
        assert_eq!(get_result_after_del.unwrap(), None, "Key should be deleted");
    }
}