//! Redis服务测试

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::redis_service::RedisService;
    use crate::cache::redis_manager;
    use crate::config;
    
    #[test]
    fn test_redis_connection() {
        // 初始化配置
        config::init();
        
        // 初始化Redis连接池
        let result = redis_manager::init_redis_pool();
        assert!(result.is_ok());
        
        // 测试获取连接
        let conn = redis_manager::get_redis_connection();
        assert!(conn.is_ok());
    }
    
    #[test]
    fn test_redis_operations() {
        // 初始化配置
        config::init();
        
        // 初始化Redis连接池
        let _ = redis_manager::init_redis_pool();
        
        let key = "test_key";
        let value = "test_value";
        
        // 测试设置值
        let result = RedisService::set(key, value, Some(60));
        assert!(result.is_ok());
        
        // 测试获取值
        let result = RedisService::get(key);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap(), value);
        
        // 测试检查键是否存在
        let result = RedisService::exists(key);
        assert!(result.is_ok());
        assert!(result.unwrap());
        
        // 测试删除键
        let result = RedisService::del(key);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
        
        // 再次检查键是否存在
        let result = RedisService::exists(key);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }
}