# Redis集成使用说明

## 概述

本项目已经集成了Redis支持，使用[r2d2](https://crates.io/crates/r2d2)作为连接池管理器，[r2d2-redis](https://crates.io/crates/r2d2-redis)作为Redis驱动。

## 配置

Redis配置在`config.toml`文件中：

```toml
[redis]
host = "10.22.2.9"
port = 6379
timeout = 10000
database = 0
max_active = 16
max_wait = -1
max_idle = 8
min_idle = 0
```

## 初始化

Redis连接池在应用启动时自动初始化，位于[main.rs](src/main.rs)中：

```rust
// 初始化Redis连接池
if let Err(e) = crate::cache::redis_manager::init_redis_pool() {
    tracing::error!("Failed to initialize Redis pool: {}", e);
}
```

## 使用方法

### 1. Redis服务层

Redis操作封装在[RedisService](src/services/redis_service.rs)中，提供了常用的操作方法：

- `set(key, value, ttl)` - 设置键值对，可选过期时间
- `get(key)` - 获取键值
- `del(key)` - 删除键
- `exists(key)` - 检查键是否存在
- `expire(key, ttl)` - 设置键的过期时间

### 2. 使用示例

```rust
use crate::services::redis_service::RedisService;

// 设置键值对
RedisService::set("my_key", "my_value", Some(60))?;

// 获取键值
let value = RedisService::get("my_key")?;

// 删除键
RedisService::del("my_key")?;
```

### 3. API接口

项目提供了Redis操作的API接口：

- `POST /redis/set` - 设置键值对
- `GET /redis/get/{key}` - 获取键值
- `DELETE /redis/delete/{key}` - 删除键

请求示例：
```bash
# 设置键值对
curl -X POST http://localhost:8008/redis/set \
  -H "Content-Type: application/json" \
  -d '{"key": "test_key", "value": "test_value", "ttl": 60}'

# 获取键值
curl -X GET http://localhost:8008/redis/get/test_key

# 删除键
curl -X DELETE http://localhost:8008/redis/delete/test_key
```

## 自定义Redis操作

如果需要执行RedisService中未包含的操作，可以直接获取连接：

```rust
use crate::cache::redis_manager;
use r2d2_redis::redis::Commands;

let mut conn = redis_manager::get_redis_connection()?;
// 执行自定义Redis命令
let _: () = conn.set("my_key", "my_value")?;
```

## 错误处理

所有Redis操作都返回`Result`类型，可以使用`?`操作符传播错误，或使用`match`进行处理。