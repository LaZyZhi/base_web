use salvo::Router;

use crate::handlers::permission::user_handler;

pub fn user_router() -> Router{
    Router::with_path("/user")
    .post(user_handler::login)
    .get(user_handler::list_page)
}