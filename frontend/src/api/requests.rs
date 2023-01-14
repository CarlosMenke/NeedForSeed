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

/// this function returns raw html for finance, time and music summary.
pub async fn get_html(
    token: String,
    selected: shared::models::HtmlSuggestion,
) -> fetch::Result<shared::models::ResponseHtml> {
    fetch(
        Request::new(get_api_url(String::from("api/auth/get_html.json")))
            .method(Method::Post)
            .header(Header::bearer(token))
            .json(&selected)?,
    )
    .await?
    .check_status()?
    .json()
    .await
}

/// returns the suggestions for a html summary.
pub async fn get_html_suggestion(
    token: String,
) -> fetch::Result<shared::models::ResponseHtmlSuggestion> {
    Request::new(get_api_url(String::from(
        "api/auth/get_html_suggestions.json",
    )))
    .header(Header::bearer(token))
    .fetch()
    .await?
    .check_status()?
    .json()
    .await
}

/// returns the suggestions for a new Time Tracking entery.
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

/// returns the suggestions for a new Finance Tracking entery.
pub async fn get_finance_suggestion(
    token: String,
) -> fetch::Result<shared::models::FinanceEnterySuggestion> {
    Request::new(get_api_url(String::from(
        "api/auth/get_finance_suggestions.json",
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

/// this function returns a Vector, witch holds all history of time Enteries.
pub async fn get_history_entery(
    token: String,
    target: shared::models::RequestEnteryHistory,
) -> fetch::Result<shared::models::ResponseTimeEnteryHistory> {
    fetch(
        Request::new(get_api_url(String::from(
            "api/auth/get_entery_history.json",
        )))
        .method(Method::Post)
        .header(Header::bearer(token))
        .json(&target)?,
    )
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
    entery: shared::models::StopLedgerTimeEntery,
) -> fetch::Result<shared::models::ResponseStatus> {
    fetch(
        Request::new(get_api_url(String::from(
            "api/auth/set_time_entery_kill.json",
        )))
        .method(Method::Post)
        .header(Header::bearer(token))
        .json(&entery)?,
    )
    .await?
    .check_status()?
    .json()
    .await
}

pub async fn start_finance_entery(
    token: String,
    new_entery: shared::models::NewFinanceEntery,
) -> fetch::Result<shared::models::ResponseStatus> {
    fetch(
        Request::new(get_api_url(String::from(
            "api/auth/set_finance_entery_create.json",
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
