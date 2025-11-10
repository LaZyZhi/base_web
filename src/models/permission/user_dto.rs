use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

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
#[allow(dead_code)]
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
