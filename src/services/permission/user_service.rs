use salvo::http::StatusError;
use sea_orm::DatabaseConnection;
use crate::utils::error_util;
use crate::{
    common::api_response::AppResult, entities::permission::sys_user,
    models::permission::user_dto::CreateReq, repository::permission::user_repository, utils,
};
use crate::repository::permission::employee_repository;

pub struct UserService;

impl UserService {
    /// 根据用户ID获取用户信息
    pub async fn get_user_by_id(
        user_id: &str,
        db: &DatabaseConnection,
    ) -> AppResult<sys_user::Model> {
        let user =
            crate::repository::permission::user_repository::query_user_by_user_id(user_id, db)
                .await;
        match user {
            Ok(Some(user)) => Ok(user),
            Ok(None) => {
                tracing::error!("用户{}不存在", user_id);
                Err(StatusError::unauthorized()
                    .brief("用户账号或者密码不正确")
                    .into())
            }
            Err(_) => {
                Err(error_util::system_error())
            }
        }
    }

    /// 验证用户状态
    pub async fn verify_user_status(user: &sys_user::Model) -> AppResult<()> {
        // 校验用户是否有效
        if !user.is_valid() {
            tracing::error!("无效用户{}", user.user_id);
            return Err(StatusError::unauthorized().brief("无效用户").into());
        }

        // 校验用户是否被锁定
        if user.is_locked() {
            tracing::error!("用户已锁定{}", user.user_id);
            return Err(StatusError::unauthorized().brief("用户已锁定").into());
        }

        Ok(())
    }

    /// 校验用户账号密码
    pub async fn verify_user_credentials(password: &str, input_password: &str) -> AppResult<()> {
        // 密码校验业务逻辑
        if utils::verify_password(input_password, password)
            .err()
            .is_some()
        {
            return Err(StatusError::unauthorized()
                .brief("用户账号或者密码不正确")
                .into());
        }
        Ok(())
    }

    pub async fn create_user(mut data: CreateReq, db: &DatabaseConnection) -> AppResult<sys_user::Model> {
        // 查询输入的工号是否存在
        let employee = employee_repository::query_employee_by_emp_no(&data.user_id, db).await?
            .ok_or_else(|| StatusError::internal_server_error().brief("请使用正确的工号进行注册"))?;

        // 检查工号是否有效
        if !employee.is_active() {
            return Err(StatusError::internal_server_error().brief("该工号已无效").into());
        }

        // 设置用户名和手机号
        data.user_name = employee.empname.unwrap_or_default();
        if data.phone.is_none() {
            data.phone = employee.mobileno;
        }
        // 密码哈希处理
        data.password = utils::hash_password(&data.password)?;

        // 创建用户，依赖数据库层面的唯一性约束来处理重复用户的情况
        user_repository::create_user(data, db).await.map_err(|e| {
            tracing::error!("create_user error: {}", e);
            e
        })
    }
}