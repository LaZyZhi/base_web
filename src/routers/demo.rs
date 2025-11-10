use salvo::prelude::*;

use crate::common::api_response::AppResult;

#[handler]
pub async fn hello(req: &mut Request) -> AppResult<Text<String>> {
    let name = req.query::<&str>("name").unwrap_or("World");
    Ok(Text::Plain(format!("Hello {}!", name)))
}