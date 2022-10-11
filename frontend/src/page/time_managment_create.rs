use crate::api;
use enclose::enc;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use seed::{prelude::*, *};
use std::collections::BTreeMap;

// ------ ------
//     Init
// ------ ------

pub fn init(
    url: Url,
    orders: &mut impl Orders<Msg>,
    ctx: Option<shared::auth::UserLoginResponse>,
) -> Model {
    orders.skip().perform_cmd({
        let token = ctx.clone().unwrap().token;
        async { Msg::FetchedSuggestion(api::requests::get_time_suggestion(token).await) }
    });
    orders.skip().perform_cmd({
        let token = ctx.clone().unwrap().token;
        async { Msg::FetchedRunningEntery(api::requests::get_time_running_entery(token).await) }
    });
    Model {
        _base_url: url.to_base_url(),
        ctx,
        suggestions: None,
        start_entery: shared::models::StartTimeEntery::default(),
        running_entery: None,
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    _base_url: Url,
    ctx: Option<shared::auth::UserLoginResponse>,
    suggestions: Option<shared::models::ResponseBTreeMap>,
    start_entery: shared::models::StartTimeEntery,
    running_entery: Option<shared::models::ResponseRunningLedgerTimeEntery>,
}

// ------ Frequency ------

pub enum Msg {
    GetSuggestion,
    StartTimeEntery,
    FetchedSuggestion(fetch::Result<shared::models::ResponseBTreeMap>),
    FetchedRunningEntery(fetch::Result<shared::models::ResponseRunningLedgerTimeEntery>),
    FetchedStartTimeEntery(fetch::Result<shared::models::ResponseStatus>),
    SaveNewEnteryHeadline(String),
    SaveNewEnteryTarget(String),
    StopTimeEntery(String),
}
// ------ ------
//     Urls
// ------ ------

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SaveNewEnteryHeadline(content) => {
            model.start_entery.headline = content;
        }
        Msg::SaveNewEnteryTarget(content) => {
            model.start_entery.account_target = content;
        }
        Msg::GetSuggestion => {
            orders.skip().perform_cmd({
                let token = model.ctx.clone().unwrap().token;
                async { Msg::FetchedSuggestion(api::requests::get_time_suggestion(token).await) }
            });
        }
        Msg::StartTimeEntery => {
            orders.skip().perform_cmd({
                let token = model.ctx.clone().unwrap().token;
                let start_entery = model.start_entery.clone();
                async {
                    Msg::FetchedStartTimeEntery(
                        api::requests::start_time_entery(token.clone(), start_entery).await,
                    );
                    Msg::FetchedRunningEntery(api::requests::get_time_running_entery(token).await)
                }
            });
        }
        Msg::StopTimeEntery(remove_line) => {
            orders.skip().perform_cmd({
                let token = model.ctx.clone().unwrap().token;
                let new_entery = model
                    .running_entery
                    .as_ref()
                    .unwrap()
                    .running_entery
                    .get(&remove_line)
                    .unwrap()
                    .clone();
                let stop_entery = shared::models::StopLedgerTimeEntery {
                    remove_line,
                    new_entery,
                };
                log!(stop_entery);
                async {
                    Msg::FetchedStartTimeEntery(
                        api::requests::stop_time_entery(token.clone(), stop_entery).await,
                    );
                    Msg::FetchedRunningEntery(api::requests::get_time_running_entery(token).await)
                }
            });
        }
        Msg::FetchedStartTimeEntery(Ok(_response_data)) => {
            model.start_entery = shared::models::StartTimeEntery::default();
        }
        Msg::FetchedSuggestion(Ok(response_data)) => {
            model.suggestions = Some(response_data);
        }
        Msg::FetchedRunningEntery(Ok(response_data)) => {
            log!(response_data);
            model.running_entery = Some(response_data);
        }
        Msg::FetchedSuggestion(Err(fetch_error))
        | Msg::FetchedRunningEntery(Err(fetch_error))
        | Msg::FetchedStartTimeEntery(Err(fetch_error)) => {
            log!("Fetch error:", fetch_error);
            orders.skip();
        }
    }
}
// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    let suggestions = match model.suggestions.clone() {
        Some(m) => m.map,
        None => BTreeMap::new(),
    };
    let running_entery = match model.running_entery.clone() {
        Some(m) => m.running_entery,
        None => BTreeMap::new(),
    };
    let matcher = SkimMatcherV2::default();
    let threshhold: i64 = model
        .start_entery
        .account_target
        .replace(" ", "")
        .chars()
        .count() as i64
        * 5;

    div![
        "Create new time Tracking Entery",
        div![
            input![
                C!["input-content_origin"],
                input_ev(Ev::Input, Msg::SaveNewEnteryHeadline),
                attrs! {
                    At::Placeholder => "Origin",
                    At::AutoFocus => true.as_at_value();
                    At::Value => &model.start_entery.headline,
                    At::List => "suggestions_origin",
                },
            ],
            datalist![
                id!["suggestions_origin"],
                suggestions
                    .iter()
                    .filter(|(content, _headline)| matcher
                        .fuzzy_match(content, &model.start_entery.account_target.replace(" ", ""))
                        .unwrap_or(0)
                        > threshhold)
                    .map(|(_content, headline)| { option![headline] })
            ],
            input![
                C!["input-content_target"],
                input_ev(Ev::Input, Msg::SaveNewEnteryTarget),
                attrs! {
                    At::Placeholder => "Target",
                    //TODO change to true if it is nicer
                    At::AutoFocus => AtValue::None,
                    At::Value => &model.start_entery.account_target,
                    At::List => "suggestions_target",
                }
            ],
            datalist![
                id!["suggestions_target"],
                suggestions
                    .iter()
                    .map(|(content, _headline)| { option![content] })
            ],
            button![ev(Ev::Click, |_| Msg::StartTimeEntery), "Start Entery"],
            ul![running_entery.iter().filter_map(|(remove_line, entery)| {
                Some(view_runing_enteries(remove_line.to_string(), entery))
            })]
        ],
    ]
}

//TODO add change name / headline of running entery
fn view_runing_enteries(remove_line: String, entery: &shared::models::NewTimeEntery) -> Node<Msg> {
    //TODO use entery for button name
    div![
        style! {St::Display => "flex", St::FlexDirection => "column", St::MaxWidth => px(500), St::Margin => "auto", St::MarginTop => px(30)},
        p![entery.headline.clone()],
        p![entery.account_target.clone()],
        button![
            "Stop",
            ev(
                Ev::Click,
                enc!((remove_line) move |_| Msg::StopTimeEntery(remove_line))
            )
        ]
    ]
}
