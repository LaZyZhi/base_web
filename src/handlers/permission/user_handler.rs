use salvo::{
    Response,
    oapi::{endpoint, extract::JsonBody},
};

use crate::{
    common::api_response::{JsonResult, api_success}, db,
    hoops::jwt,
    models::permission::user_dto::{LogInRes, LoginReq},
    services::permission::user_service, utils::param_validation_util,
};
use salvo::Writer;

#[endpoint(
    tags("用户与权限相关"),
    summary = "分页查询用户信息",
    description = "List page description"
)]
pub async fn list_page() -> JsonResult<()> {
    todo!()
}

#[endpoint(tags("用户与权限相关"), summary = "用户登录", description = "用户登录")]
pub async fn login(idata: JsonBody<LoginReq>, res: &mut Response) -> JsonResult<LogInRes> {
    let data = idata.into_inner();
    // 执行数据验证，如果验证失败则返回错误
    param_validation_util::validate_param(&data).await?;

    let db = db::postgres::pool();

    // 根据user_id查询用户信息
    let user = user_service::UserService::get_user_by_id(&data.user_id, db).await?;

    // 校验用户状态
    user_service::UserService::verify_user_status(&user).await?;

    // 校验用户密码
    user_service::UserService::verify_user_credentials(&user.password, &data.password).await?;

    let (token, _) = jwt::get_token(&data.user_id)?;

    let _ = res.add_header("Authorization", &token, true);

    api_success(LogInRes {
        authorization: vec![format!("Bearer {}", token)],
    }, "登录成功")
}