use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, Default)]
#[sea_orm(table_name="sys_user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment= true)]
    pub auto_id: i64,

    #[sea_orm(unique)]
    pub user_id: String,
    pub user_name: String,

    #[serde(skip_serializing)]
    pub password: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub remark: Option<String>,
    pub last_login: Option<DateTime>,
    pub login_ip: Option<String>,
    pub image_url: Option<String>,
    pub reg_time: Option<DateTime>,
    pub locked: i32,
    pub is_valid: i32,
    pub email_password: Option<String>,

}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// 检查用户是否被锁定,1为锁定
    pub fn is_locked(&self) -> bool {
        self.locked == 1
    }

    /// 检查用户是否有效,0为有效
    pub fn is_valid(&self) -> bool {
        self.is_valid == 0
    }
}