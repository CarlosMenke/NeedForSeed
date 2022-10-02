use crate::api;
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
    Model {
        _base_url: url.to_base_url(),
        ctx,
        suggestions: None,
        new_entery: shared::models::NewTimeEntery::default(),
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    _base_url: Url,
    ctx: Option<shared::auth::UserLoginResponse>,
    suggestions: Option<shared::models::ResponseBTreeMap>,
    new_entery: shared::models::NewTimeEntery,
}

// ------ Frequency ------

pub enum Msg {
    GetSummary,
    FetchedSuggestion(fetch::Result<shared::models::ResponseBTreeMap>),
    SaveNewEnteryTarget(String),
    SaveNewEnteryOrigin(String),
}
// ------ ------
//     Urls
// ------ ------

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SaveNewEnteryTarget(content) => {
            model.new_entery.account_target = content;
        }
        Msg::SaveNewEnteryOrigin(content) => {
            model.new_entery.account_origin = content;
        }
        Msg::GetSummary => {
            orders.skip().perform_cmd({
                let token = model.ctx.clone().unwrap().token;
                async { Msg::FetchedSuggestion(api::requests::get_time_suggestion(token).await) }
            });
        }
        Msg::FetchedSuggestion(Ok(response_data)) => {
            model.suggestions = Some(response_data);
        }
        Msg::FetchedSuggestion(Err(fetch_error)) => {
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
    let matcher = SkimMatcherV2::default();
    let threshhold: i64 = model
        .new_entery
        .account_target
        .replace(" ", "")
        .chars()
        .count() as i64
        * 10;

    div![
        "Create new time Tracking Entery",
        div![
            input![
                C!["input-content_origin"],
                input_ev(Ev::Input, Msg::SaveNewEnteryOrigin),
                attrs! {
                    At::Placeholder => "Origin",
                    At::AutoFocus => AtValue::None,
                    At::Value => &model.new_entery.account_origin,
                    At::List => "suggestions_origin",
                }
            ],
            datalist![
                id!["suggestions_origin"],
                suggestions
                    .iter()
                    .filter(|(content, _headline)| matcher
                        .fuzzy_match(content, &model.new_entery.account_target.replace(" ", ""))
                        .unwrap_or(0)
                        > threshhold)
                    .map(|(_content, headline)| {
                        option![format!(
                            "{:?}-{:?}",
                            matcher
                                .fuzzy_match(
                                    _content,
                                    &model.new_entery.account_target.replace(" ", "")
                                )
                                .unwrap_or(0),
                            headline
                        )]
                    })
            ],
            input![
                C!["input-content_target"],
                input_ev(Ev::Input, Msg::SaveNewEnteryTarget),
                attrs! {
                    At::Placeholder => "Target",
                    At::AutoFocus => AtValue::None,
                    At::Value => &model.new_entery.account_target,
                    At::List => "suggestions_target",
                }
            ],
            datalist![
                id!["suggestions_target"],
                suggestions
                    .iter()
                    .map(|(content, _headline)| { option![content] })
            ]
        ],
    ]
}
