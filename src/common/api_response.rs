use salvo::{http::StatusCode, oapi::ToSchema, writing::Json};
use serde::Serialize;

use crate::AppError;

pub type AppResult<T> = Result<T, AppError>;
pub type JsonResult<T> = Result<Json<ApiResponse<T>>, AppError>;
pub type EmptyResult = Result<Json<Empty>, AppError>;

#[derive(Serialize, ToSchema, Clone, Debug)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub msg: String,
    pub data: T,
}

pub fn json_ok<T>(data: T) -> JsonResult<T> {
    Ok(Json(ApiResponse {
        code: StatusCode::OK.as_u16(),
        msg: "success".to_string(),
        data,
    }))
}

pub fn api_success<T>(data: T, msg: &str) -> JsonResult<T> {
    Ok(Json(ApiResponse {
        code: StatusCode::OK.as_u16(),
        msg: msg.to_string(),
        data,
    }))
}

#[allow(dead_code)]
pub fn api_error<T: Serialize>(data: T, msg: &str) -> JsonResult<T> {
    Ok(Json(ApiResponse {
        code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        msg: msg.to_string(),
        data,
    }))
}

#[allow(dead_code)]
pub fn api_error_with_code<T: Serialize>(code: StatusCode, data: T, msg: &str) -> JsonResult<T> {
    Ok(Json(ApiResponse {
        code: code.as_u16(),
        msg: msg.to_string(),
        data,
    }))
}

#[derive(Serialize, ToSchema, Clone, Copy, Debug)]
pub struct Empty {}
#[allow(dead_code)]
pub fn empty_ok() -> JsonResult<Empty> {
    Ok(Json(ApiResponse {
        code: StatusCode::OK.as_u16(),
        msg: "success".to_string(),
        data: Empty {},
    }))
}
