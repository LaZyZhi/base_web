use cookie::Cookie;
use salvo::oapi::extract::*;
use salvo::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::entities::users::Model;
use crate::entities::{prelude::Users, users};
use crate::hoops::jwt;
use crate::{db, common::api_response::json_ok, utils, common::api_response::AppResult, common::api_response::JsonResult};

#[handler]
pub async fn login_page(res: &mut Response) -> AppResult<()> {
    if let Some(cookie) = res.cookies().get("jwt_token") {
        let token = cookie.value().to_string();
        if jwt::decode_token(&token) {
            res.render(Redirect::other("/users"));
            return Ok(());
        }
    }
    // 返回简单的文本响应而不是HTML模板
    res.render(Text::Plain("Login page - please use the API to login"));
    Ok(())
}

#[derive(Deserialize, ToSchema, Default, Debug)]
pub struct LoginInData {
    pub username: String,
    pub password: String,
}
#[derive(Serialize, ToSchema, Default, Debug)]
pub struct LoginOutData {
    pub id: String,
    pub username: String,
    pub token: String,
    pub exp: i64,
}
#[endpoint(tags("auth"))]
pub async fn post_login(
    idata: JsonBody<LoginInData>,
    res: &mut Response,
) -> JsonResult<LoginOutData> {
    let idata = idata.into_inner();
    let conn = db::postgres::pool();
    let Some(Model {
        id,
        username,
        password,
    }) = Users::find()
        .filter(users::Column::Username.eq(idata.username))
        .one(conn)
        .await?
    else {
        return Err(StatusError::unauthorized()
            .brief("User does not exist.")
            .into());
    };

    if utils::verify_password(&idata.password, &password).is_err()
    {
        return Err(StatusError::unauthorized()
            .brief("Account not exist or password is incorrect.")
            .into());
    }

    let (token, exp) = jwt::get_token(&id)?;
    let odata = LoginOutData {
        id,
        username,
        token,
        exp,
    };
    let cookie = Cookie::build(("jwt_token", odata.token.clone()))
        .path("/")
        .http_only(true)
        .build();
    res.add_cookie(cookie);
    json_ok(odata)
}