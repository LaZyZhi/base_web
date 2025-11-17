use std::sync::Arc;

use salvo::oapi::ToSchema;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::app::AppState;
use crate::common::api_response::{api_success, JsonResult};
use crate::services::redis_service::RedisService;
use crate::utils::param_validation_util::validate_param;
use crate::AppError;

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct SetRequest {
    #[validate(length(min = 1, message = "key 不能为空"))]
    pub key: String,
    #[validate(length(min = 1, message = "value 不能为空"))]
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

/// 设置Redis键值对（带校验与状态注入）。
#[endpoint(tags("Redis操作"), summary = "设置键值对", description = "设置Redis键值对，可选过期时间")]
pub async fn set(req: &mut Request, depot: &mut Depot) -> JsonResult<OperationResponse> {
    let data: SetRequest = req.parse_json().await?;
    validate_param(&data).await?;
    let state = get_state(depot)?;

    match RedisService::set_with_pool(&state.redis, &data.key, &data.value, data.ttl).await {
        Ok(_) => api_success(
            OperationResponse {
                success: true,
                message: "设置成功".to_string(),
            },
            "设置成功",
        ),
        Err(e) => {
            tracing::error!("设置Redis键值对失败: {}", e);
            Err(salvo::http::StatusError::internal_server_error()
                .brief("设置失败")
                .into())
        }
    }
}

/// 获取Redis键值（依赖 AppState 中的连接池）。
#[endpoint(tags("Redis操作"), summary = "获取键值", description = "根据键获取Redis中存储的值")]
pub async fn get(req: &mut Request, depot: &mut Depot) -> JsonResult<GetResponse> {
    let key = req.param::<String>("key").unwrap_or_default();
    let state = get_state(depot)?;

    match RedisService::get_with_pool(&state.redis, &key).await {
        Ok(value) => api_success(
            GetResponse {
                key: key.clone(),
                value,
            },
            "获取成功",
        ),
        Err(e) => {
            tracing::error!("获取Redis键值失败: {}", e);
            Err(salvo::http::StatusError::internal_server_error()
                .brief("获取失败")
                .into())
        }
    }
}

/// 删除Redis键（依赖 AppState 中的连接池）。
#[endpoint(tags("Redis操作"), summary = "删除键", description = "根据键删除Redis中的值")]
pub async fn delete(req: &mut Request, depot: &mut Depot) -> JsonResult<OperationResponse> {
    let key = req.param::<String>("key").unwrap_or_default();
    let state = get_state(depot)?;

    match RedisService::del_with_pool(&state.redis, &key).await {
        Ok(count) => api_success(
            OperationResponse {
                success: true,
                message: format!("删除成功，影响{}个键", count),
            },
            "删除成功",
        ),
        Err(e) => {
            tracing::error!("删除Redis键失败: {}", e);
            Err(salvo::http::StatusError::internal_server_error()
                .brief("删除失败")
                .into())
        }
    }
}

fn get_state(depot: &mut Depot) -> Result<Arc<AppState>, AppError> {
    depot
        .get::<Arc<AppState>>("app_state")
        .ok()
        .cloned()
        .ok_or_else(|| AppError::internal("AppState 未注入"))
}
