use crate::api::get_api_url;
use seed::prelude::*;
use shared::*;

pub async fn get_login(name: String, pwd: String) -> fetch::Result<auth::UserLoginResponse> {
    fetch(
        Request::new(get_api_url(String::from("api/login.json")))
            .method(Method::Post)
            .json(&auth::UserLogin {
                username: name,
                password: pwd,
            })?,
    )
    .await?
    .check_status()?
    .json()
    .await
}

pub async fn get_music(
    token: String,
    _depth: String,
) -> fetch::Result<shared::models::ResponseHtml> {
    Request::new(get_api_url(String::from("api/auth/get_music.json")))
        .header(Header::bearer(token))
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}
