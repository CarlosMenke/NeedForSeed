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

/// this function returns raw html for finance and music summary
pub async fn get_html(
    token: String,
    target: String,
    depth: String,
    timeframe: String,
) -> fetch::Result<shared::models::ResponseHtml> {
    Request::new(get_api_url(String::from(format!(
        "api/auth/get_{}/depth_{}/timeframe_{}.json",
        target, depth, timeframe
    ))))
    .header(Header::bearer(token))
    .fetch()
    .await?
    .check_status()?
    .json()
    .await
}

/// this function returns a HashMap, witch encodes the suggestions for a new time Tracking entery
pub async fn get_time_suggestion(token: String) -> fetch::Result<shared::models::ResponseHashMap> {
    Request::new(get_api_url(String::from(
        "api/auth/get_time_suggestions.json",
    )))
    .header(Header::bearer(token))
    .fetch()
    .await?
    .check_status()?
    .json()
    .await
}
