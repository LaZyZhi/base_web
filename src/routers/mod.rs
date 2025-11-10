use salvo::prelude::*;

mod auth;
mod demo;
mod user;
pub mod permission;

use crate::{config, hoops};

pub fn root() -> Router {
    let router = Router::new()
        .hoop(Logger::new())
        .get(demo::hello)
        .push(Router::with_path("login").get(auth::login_page))
        .push(Router::with_path("users").get(user::list_page))
        .push(
            Router::with_path("rust")
                .push(permission::user_router::user_router())
                .push(Router::with_path("login").post(auth::post_login))
                .push(
                    Router::with_path("users")
                        .hoop(hoops::auth_hoop(&config::get().jwt))
                        .get(user::list_users)
                        .post(user::create_user)
                        .push(
                            Router::with_path("{user_id}")
                                .put(user::update_user)
                                .delete(user::delete_user),
                        ),
                ),
        );
    let doc = OpenApi::new("salvo web api", "0.0.1").merge_router(&router);
    return router
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(Scalar::new("/api-doc/openapi.json").into_router("scalar"));
}