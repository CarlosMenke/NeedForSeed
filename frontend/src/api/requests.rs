use crate::api::get_api_url;
use seed::prelude::*;
use shared;

pub async fn get_login(
    name: String,
    pwd: String,
) -> fetch::Result<shared::auth::UserLoginResponse> {
    fetch(
        Request::new(get_api_url(String::from("api/login.json")))
            .method(Method::Post)
            .json(&shared::auth::UserLogin {
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
pub async fn get_html_timepoint(
    token: String,
    target: String,
    depth: String,
    timeframe: String,
    timepoint: String,
) -> fetch::Result<shared::models::ResponseHtml> {
    Request::new(get_api_url(String::from(format!(
        "api/auth/get_{}/depth_{}/timeframe_{}/timepoint_{}.json",
        target, depth, timeframe, timepoint
    ))))
    .header(Header::bearer(token))
    .fetch()
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

/// this function returns a BTreeMap, witch encodes the suggestions for a new time Tracking entery
pub async fn get_time_suggestion(
    token: String,
) -> fetch::Result<shared::models::HeadlineSuggestion> {
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

/// this function returns a BTreeMap, witch encodes all running time Enteries
pub async fn get_time_running_entery(
    token: String,
) -> fetch::Result<shared::models::ResponseRunningLedgerTimeEntery> {
    Request::new(get_api_url(String::from(
        "api/auth/set_time_entery_running.json",
    )))
    .header(Header::bearer(token))
    .fetch()
    .await?
    .check_status()?
    .json()
    .await
}

pub async fn start_time_entery(
    token: String,
    new_entery: shared::models::StartTimeEntery,
) -> fetch::Result<shared::models::ResponseStatus> {
    fetch(
        Request::new(get_api_url(String::from(
            "api/auth/set_time_entery_start.json",
        )))
        .method(Method::Post)
        .header(Header::bearer(token))
        .json(&new_entery)?,
    )
    .await?
    .check_status()?
    .json()
    .await
}

pub async fn stop_time_entery(
    token: String,
    new_entery: shared::models::StopLedgerTimeEntery,
) -> fetch::Result<shared::models::ResponseStatus> {
    fetch(
        Request::new(get_api_url(String::from(
            "api/auth/set_time_entery_stop.json",
        )))
        .method(Method::Post)
        .header(Header::bearer(token))
        .json(&new_entery)?,
    )
    .await?
    .check_status()?
    .json()
    .await
}

pub async fn kill_time_entery(
    token: String,
    new_entery: shared::models::StopLedgerTimeEntery,
) -> fetch::Result<shared::models::ResponseStatus> {
    fetch(
        Request::new(get_api_url(String::from(
            "api/auth/set_time_entery_kill.json",
        )))
        .method(Method::Post)
        .header(Header::bearer(token))
        .json(&new_entery)?,
    )
    .await?
    .check_status()?
    .json()
    .await
}
