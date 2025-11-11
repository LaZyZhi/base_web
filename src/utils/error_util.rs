use salvo::http::StatusError;

use crate::AppError;

pub fn system_error() -> AppError{
    StatusError::internal_server_error()
        .brief("系统异常,请联系管理员确认")
        .into()
}