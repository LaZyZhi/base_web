//! Redis路由模块
//!
//! 定义Redis相关API路由

use salvo::prelude::*;

use crate::handlers::redis_handler::{set, get, delete};

/// Redis路由
pub fn redis_router() -> Router {
    Router::with_path("redis")
        .push(
            Router::with_path("set")
                .post(set)
        )
        .push(
            Router::with_path("get/<key>")
                .get(get)
        )
        .push(
            Router::with_path("delete/<key>")
                .delete(delete)
        )
}