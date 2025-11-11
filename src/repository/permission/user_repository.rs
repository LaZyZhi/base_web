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

pub async fn create_user(data: CreateReq, db: &DatabaseConnection) -> AppResult<()> {
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

    SysUser::insert(insert_data)
        .exec(db)
        .await
        .map(|_| ())
        .map_err(|e| {
            tracing::error!("create_user error: {}", e);
            error_util::system_error()
        })
}