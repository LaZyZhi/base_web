use salvo::http::StatusError;
use sea_orm::DatabaseConnection;

use crate::{common::api_response::AppResult, entities::permission::sys_user, utils};

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
}
