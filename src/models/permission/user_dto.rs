use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{entities::permission::sys_user, utils};

#[derive(Debug, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct UserPageReq {
    pub cur_page: i32,
    pub page_size: i32,
    
    pub user_id: Option<String>,
    pub user_name: Option<String>,
    pub locked: Option<String>,
    pub is_valid: Option<String>
}

#[derive(Debug, ToSchema, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginReq {
    /// 用户id(工号)
    #[validate(length(min = 1, message = "用户id不能为空"))]
    pub user_id : String,

    /// 密码
    #[validate(length(min = 1, message = "密码不能为空"))]
    pub password: String
}

#[derive(Debug, ToSchema, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogInRes {
    #[serde(rename = "Authorization")]
    pub authorization: Vec<String>
}

#[derive(Debug, ToSchema, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct RegisterReq {
    /// 用户id(工号)
    #[validate(length(min = 1, message = "用户id不能为空"))]
    pub user_id : String,

    /// 密码
    #[validate(length(min = 1, message = "密码不能为空"))]
    pub password: String
}

impl RegisterReq {
    pub fn into_model(self) -> sys_user::Model {
        sys_user::Model {
            user_id: self.user_id,
            password: utils::hash_password(&self.password).unwrap(),
            ..Default::default()
        }
    }
}


#[derive(Debug, ToSchema, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct CreateReq {
    /// 用户id(工号)
    #[validate(length(min = 1, message = "用户id不能为空"))]
    pub user_id : String,

    /// 用户名称
    #[validate(length(min = 1, message = "用户名称不能为空"))]
    pub user_name : String,

    /// 密码
    #[validate(length(min = 1, message = "密码不能为空"))]
    pub password: String,

    /// 手机号
    pub phone: Option<String>,

    /// 邮箱
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: Option<String>,

    /// 用户邮箱密码
    pub email_password: Option<String>,

    ///备注
    pub remark: Option<String>,

    /// 头像图片地址
    pub image_url: Option<String>,

    /// 是否锁定
    pub locked: Option<i16>,

    /// 是否有效
    pub is_valid: Option<i16>,
}