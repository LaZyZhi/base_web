# Kafka 集成指南

## 概述

本文档说明了如何在本项目中集成 Kafka 支持。由于在 Windows 环境下配置 Kafka 的复杂性，我们暂时移除了 Kafka 依赖，但保留了完整的配置结构，以便将来需要时可以轻松添加。

## 如何添加 Kafka 支持

### 1. 添加依赖

在 `Cargo.toml` 中添加以下依赖：

```toml
# Kafka依赖
rdkafka = { version = "0.36.2", features = ["dynamic-linking"] }
```

注意：在某些环境下，您可能需要使用不同的特性：
```toml
# 替代方案1：使用 cmake 构建
rdkafka = { version = "0.36.2", features = ["cmake-build"] }

# 替代方案2：静态链接
rdkafka = { version = "0.36.2", features = ["static-linking"] }
```

### 2. 环境要求

根据您选择的特性，您可能需要安装以下工具：

- **pkg-config**：如果使用 `dynamic-linking` 特性
- **CMake**：如果使用 `cmake-build` 特性
- **librdkafka 开发包**：在 Linux 系统上

在 Ubuntu/Debian 系统上，可以运行：
```bash
sudo apt-get install librdkafka-dev
```

在 CentOS/RHEL 系统上，可以运行：
```bash
sudo yum install librdkafka-devel
```

### 3. 恢复代码文件

将以下文件从备份中恢复或重新创建：

- [src/kafka/mod.rs](file:///c:/code/base_web/src/kafka/mod.rs)
- [src/kafka/producer.rs](file:///c:/code/base_web/src/kafka/producer.rs)
- [src/kafka/consumer.rs](file:///c:/code/base_web/src/kafka/consumer.rs)
- [src/services/kafka_service.rs](file:///c:/code/base_web/src/services/kafka_service.rs)
- [src/handlers/kafka_handler.rs](file:///c:/code/base_web/src/handlers/kafka_handler.rs)
- [src/routers/kafka_router.rs](file:///c:/code/base_web/src/routers/kafka_router.rs)

### 4. 更新模块引用

更新以下文件以包含 Kafka 模块：

1. 在 [src/main.rs](file:///c:/code/base_web/src/main.rs) 中添加 `mod kafka;` 并初始化 Kafka 生产者和消费者
2. 在 [src/services/mod.rs](file:///c:/code/base_web/src/services/mod.rs) 中添加 `pub mod kafka_service;`
3. 在 [src/handlers/mod.rs](file:///c:/code/base_web/src/handlers/mod.rs) 中添加 `pub mod kafka_handler;`
4. 在 [src/routers/mod.rs](file:///c:/code/base_web/src/routers/mod.rs) 中添加 `pub mod kafka_router;` 并在路由中包含 Kafka 路由

### 5. 配置

在 `config.toml` 中添加 Kafka 配置：

```toml
[kafka]
bootstrap_servers = "localhost:9092"
group_id = "base_web_group"
enable_auto_commit = true
auto_offset_reset = "latest"
ack_mode = "all"
transaction_id_prefix = "base_web_tx"
security_protocol = "plaintext"
```

## Kafka API 接口

项目提供了以下 Kafka 操作的 API 接口：

- `POST /kafka/send` - 发送消息到 Kafka 主题
- `POST /kafka/subscribe` - 订阅 Kafka 主题
- `GET /kafka/consume` - 从订阅的主题中消费消息

## 使用示例

### 发送消息
```bash
curl -X POST http://localhost:8008/kafka/send \
  -H "Content-Type: application/json" \
  -d '{
    "topic": "test_topic",
    "key": "test_key",
    "payload": "Hello, Kafka!"
  }'
```

### 订阅主题
```bash
curl -X POST http://localhost:8008/kafka/subscribe \
  -H "Content-Type: application/json" \
  -d '{
    "topics": ["test_topic"]
  }'
```

### 消费消息
```bash
curl -X GET http://localhost:8008/kafka/consume
```

## 直接在代码中使用

您也可以直接在代码中使用 Kafka 服务：

```rust
use crate::services::kafka_service::KafkaService;

// 发送消息
KafkaService::send_message("test_topic", Some("test_key"), "Hello, Kafka!")?;

// 订阅主题
KafkaService::subscribe_topics(&["test_topic"])?;

// 消费消息
if let Some((topic, key, payload)) = KafkaService::consume_message()? {
    println!("Received message from topic {}: {:?}", topic, payload);
}
```