use salvo::prelude::*;

pub mod permission;
pub mod redis_router;

pub fn root() -> Router {
    let router = Router::new()
        .hoop(RequestId::new())// 添加链路ID
        .hoop(Logger::new())// 添加日志
        .push(
            Router::with_path("rust")
                .push(permission::user_router::user_router()),
        )
        .push(redis_router::redis_router());
    let doc = OpenApi::new("salvo web api", "0.1.1").merge_router(&router);
    router
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(Scalar::new("/api-doc/openapi.json").into_router("scalar"))
}