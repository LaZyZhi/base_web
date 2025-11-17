use salvo::http::StatusError;
use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    common::api_response::AppResult,
    entities::{permission::sys_user, prelude::SysUser},
    models::permission::user_dto::CreateReq, utils::error_util,
};

pub async fn query_user_by_user_id(
    user_id: &str,
    db: &DatabaseConnection,
) -> AppResult<Option<sys_user::Model>> {
    SysUser::find()
        .filter(sys_user::Column::UserId.eq(user_id.to_string()))
        .one(db)
        .await
        .map_err(|e| {
            tracing::error!("query_user_by_user_id error: {}", e);
            error_util::system_error()
        })
}

pub async fn create_user(data: CreateReq, db: &DatabaseConnection) -> AppResult<sys_user::Model> {
    let insert_data = sys_user::ActiveModel {
        user_id: Set(data.user_id),
        user_name: Set(data.user_name),
        password: Set(data.password),
        phone: Set(data.phone),
        email: Set(data.email),
        remark: Set(data.remark),
        email_password: Set(data.email_password),
        ..Default::default()
    };

    let res = SysUser::insert(insert_data)
        .exec_with_returning(db)
        .await
        .map_err(|e| {
            let error_msg = e.to_string();
            if error_msg.contains("duplicate key value violates unique constraint") {
                tracing::error!("create_user error: {}", e);
                return StatusError::internal_server_error()
                    .brief("用户已存在")
                    .into();
            }

            tracing::error!("create_user error: {}", e);
            error_util::system_error()
        })?;

    Ok(res)
}
