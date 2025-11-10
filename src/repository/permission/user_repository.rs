use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::{permission::sys_user, prelude::SysUser};

pub async fn query_user_by_user_id(
    user_id: &str,
    db: &DatabaseConnection,
) -> Option<sys_user::Model> {
    SysUser::find()
        .filter(sys_user::Column::UserId.eq(user_id.to_string()))
        .one(db)
        .await
        .ok()?
}
