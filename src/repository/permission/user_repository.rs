use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    common::api_response::AppResult,
    entities::{permission::sys_user, prelude::SysUser},
    models::permission::user_dto::CreateReq,
};

pub async fn query_user_by_user_id(
    user_id: &str,
    db: &DatabaseConnection,
) -> Option<sys_user::Model> {
    match SysUser::find()
        .filter(sys_user::Column::UserId.eq(user_id.to_string()))
        .one(db)
        .await {
        Ok(Some(user)) => Some(user),
        Ok(None) => None,
        Err(msg) => {
            tracing::error!("{}", msg);
            None
        },
    }
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

    SysUser::insert(insert_data).exec(db).await?;

    Ok(())
}
