use salvo::Router;

use crate::handlers::permission::user_handler;

pub fn user_router() -> Router {
    Router::new()
        .push(
            Router::with_path("/auth").push(Router::with_path("/login").post(user_handler::login)),
        )
        .push(
            Router::with_path("/user")
                .push(Router::with_path("/create").post(user_handler::create))
                .push(Router::with_path("/page").post(user_handler::list_page)),
        )
}
