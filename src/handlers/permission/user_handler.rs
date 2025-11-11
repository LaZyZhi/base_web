use salvo::{
    Response,
    oapi::{endpoint, extract::JsonBody},
};

use crate::common::api_response::json_ok;
use crate::{
    common::api_response::{JsonResult, api_success},
    db,
    hoops::jwt,
    models::permission::user_dto::{CreateReq, LogInRes, LoginReq},
    services::permission::user_service,
    utils::param_validation_util,
};
use salvo::Writer;
use crate::services::redis_service::RedisService;

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
    // 保存进redis
    let _ = RedisService::set(&token, &data.user_id, Some(exp as usize));
    
    let _ = res.add_header("Authorization", &token, true);

    json_ok(LogInRes {
        authorization: vec![token],
    })
}

#[endpoint(tags("用户与权限相关"), summary = "用户注册", description = "用户注册")]
pub async fn create(data: JsonBody<CreateReq>) -> JsonResult<&'static str> {
    let data = data.into_inner();
    param_validation_util::validate_param(&data).await?;
    let db = db::postgres::pool();

    user_service::UserService::create_user(data, db).await?;
    json_ok("注册成功")
}
