use std::sync::Arc;

use salvo::Writer;
use salvo::{
    Depot, Response,
    oapi::{ToSchema, endpoint, extract::JsonBody},
};

use crate::app::AppState;
use crate::common::api_response::{JsonResult, json_ok};
use crate::utils::param_validation_util;
use crate::{
    db,
    hoops::jwt,
    models::permission::user_dto::{CreateReq, LogInRes, LoginReq},
    services::permission::user_service,
    services::redis_service::RedisService,
};
use serde::Serialize;

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateResponse {
    pub user_id: String,
    pub user_name: String,
    pub phone: Option<String>,
}

#[endpoint(
    tags("用户与权限相关"),
    summary = "分页查询用户信息",
    description = "List page description"
)]
pub async fn list_page() -> JsonResult<()> {
    todo!()
}

#[endpoint(tags("用户与权限相关"), summary = "用户登录", description = "用户登录")]
pub async fn login(data: JsonBody<LoginReq>, res: &mut Response) -> JsonResult<LogInRes> {
    let data = data.into_inner();
    // 执行数据验证，如果验证失败则返回错误
    param_validation_util::validate_param(&data).await?;

    let db = db::postgres::pool();

    // 根据user_id查询用户信息
    let user = user_service::UserService::get_user_by_id(&data.user_id, db).await?;

    // 校验用户状态
    user_service::UserService::verify_user_status(&user).await?;

    // 校验用户密码
    user_service::UserService::verify_user_credentials(&user.password, &data.password).await?;

    let (token, exp) = jwt::get_token(&data.user_id)?;
    let token = format!("Bearer {}", token);

    let _ = RedisService::set_login(&token, &data.user_id, Some(exp as usize)).await;

    let _ = res.add_header("Authorization", &token, true);

    json_ok(LogInRes {
        authorization: vec![token],
    })
}

#[endpoint(tags("用户与权限相关"), summary = "用户注册", description = "用户注册")]
pub async fn create(data: JsonBody<CreateReq>) -> JsonResult<CreateResponse> {
    let data = data.into_inner();
    param_validation_util::validate_param(&data).await?;
    let db = db::postgres::pool();

    tracing::info!(user_id = %data.user_id, "user register start");
    
    let created = user_service::UserService::create_user(data, db).await?;
    tracing::info!(user_id = %created.user_id, "user register success");

    json_ok(CreateResponse {
        user_id: created.user_id,
        user_name: created.user_name,
        phone: created.phone,
    })
}