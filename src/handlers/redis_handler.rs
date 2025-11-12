//! Redis处理器示例
//!
//! 展示如何在处理器中使用Redis服务

use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use salvo::oapi::ToSchema;

use crate::common::api_response::{JsonResult, api_success};
use crate::services::redis_service::RedisService;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct SetRequest {
    pub key: String,
    pub value: String,
    pub ttl: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct GetResponse {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct OperationResponse {
    pub success: bool,
    pub message: String,
}

/// 设置Redis键值对
#[endpoint(tags("Redis操作"), summary = "设置键值对", description = "设置Redis键值对，可选过期时间")]
pub async fn set(req: &mut Request, _res: &mut Response) -> JsonResult<OperationResponse> {
    let data: SetRequest = req.parse_json().await?;
    
    match RedisService::set(&data.key, &data.value, data.ttl) {
        Ok(_) => {
            api_success(
                OperationResponse {
                    success: true,
                    message: "设置成功".to_string(),
                },
                "设置成功",
            )
        }
        Err(e) => {
            tracing::error!("设置Redis键值对失败: {}", e);
            Err(salvo::http::StatusError::internal_server_error()
                .brief("设置失败")
                .into())
        }
    }
}

/// 获取Redis键值
#[endpoint(tags("Redis操作"), summary = "获取键值", description = "根据键获取Redis中存储的值")]
pub async fn get(req: &mut Request, _res: &mut Response) -> JsonResult<GetResponse> {
    let key = req.param::<String>("key").unwrap_or_default();
    
    match RedisService::get(&key) {
        Ok(value) => {
            api_success(
                GetResponse {
                    key: key.clone(),
                    value,
                },
                "获取成功",
            )
        }
        Err(e) => {
            tracing::error!("获取Redis键值失败: {}", e);
            Err(salvo::http::StatusError::internal_server_error()
                .brief("获取失败")
                .into())
        }
    }
}

/// 删除Redis键
#[endpoint(tags("Redis操作"), summary = "删除键", description = "根据键删除Redis中的值")]
pub async fn delete(req: &mut Request, _res: &mut Response) -> JsonResult<OperationResponse> {
    let key = req.param::<String>("key").unwrap_or_default();
    
    match RedisService::del(&key) {
        Ok(count) => {
            api_success(
                OperationResponse {
                    success: true,
                    message: format!("删除成功，影响{}个键", count),
                },
                "删除成功",
            )
        }
        Err(e) => {
            tracing::error!("删除Redis键失败: {}", e);
            Err(salvo::http::StatusError::internal_server_error()
                .brief("删除失败")
                .into())
        }
    }
}