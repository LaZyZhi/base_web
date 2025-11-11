use std::ptr::null;
use salvo::http::StatusError;
use sea_orm::DatabaseConnection;
use sea_orm::sqlx::encode::IsNull::No;
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
            Some(user) => Ok(user),
            None => {
                tracing::error!("用户{}不存在", user_id);
                Err(StatusError::unauthorized()
                    .brief("用户账号或者密码不正确")
                    .into())
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
        let input_password = utils::hash_password(input_password)?;
        if utils::verify_password(password, &input_password)
            .err()
            .is_some()
        {
            return Err(StatusError::unauthorized()
                .brief("用户账号或者密码不正确")
                .into());
        }
        Ok(())
    }

    pub async fn create_user(data: CreateReq, db: &DatabaseConnection) -> AppResult<()> {
        if user_repository::query_user_by_user_id(&data.user_id, db).await.is_some() {
            return Err(StatusError::internal_server_error().brief("用户已存在").into());
        }

        // 查询输入的工号是否存在
        let emp_info = employee_repository::query_employee_by_emp_no(&data.user_id, db).await;
        let mut data = data;
        match emp_info {
            None => {
                return Err(StatusError::internal_server_error().brief("请使用正确的工号进行注册").into());
            }
            Some(model) => {
                if !model.is_active() {
                    return Err(StatusError::internal_server_error().brief("该工号已无效").into());
                }

                data.user_name = model.empname.unwrap();
                if data.phone == None {
                    data.phone = model.mobileno;
                }
            }
        }

        data.password = utils::hash_password(&data.password)?;

        user_repository::create_user(data, db).await?;
        Ok(())
    }
}
