use crate::api;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use itertools::Itertools;
use seed::{prelude::*, *};

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
        async { Msg::FetchedSuggestion(api::requests::get_finance_suggestion(token).await) }
    });
    Model {
        _base_url: url.to_base_url(),
        ctx,
        suggestions: None,
        new_entery: shared::models::NewFinanceEntery::default(),
        suggestion_filter: "".to_string(),
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    _base_url: Url,
    ctx: Option<shared::auth::UserLoginResponse>,
    suggestions: Option<shared::models::FinanceEnterySuggestion>,
    new_entery: shared::models::NewFinanceEntery,
    suggestion_filter: String,
}

// ------ Frequency ------

pub enum Msg {
    GetSuggestion,

    FetchedNewFinanceEntery(fetch::Result<shared::models::ResponseStatus>),
    FetchedSuggestion(fetch::Result<shared::models::FinanceEnterySuggestion>),

    SaveNewEnteryHeadline(String),
    SaveNewEnteryTarget(String),
    SaveNewEnteryOrigin(String),
    SaveNewEnteryAmmount(String),
    SaveNewEnteryDate(String),

    NewFinanceEntery,
}
// ------ ------
//     Urls
// ------ ------

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SaveNewEnteryHeadline(content) => {
            model.new_entery.headline = content;
            update_suggestion_filter(model);
        }
        Msg::SaveNewEnteryTarget(content) => {
            model.new_entery.account_target = content;
            update_suggestion_filter(model);
        }
        Msg::SaveNewEnteryOrigin(content) => {
            model.new_entery.account_origin = content;
            update_suggestion_filter(model);
        }
        Msg::SaveNewEnteryAmmount(content) => {
            model.new_entery.ammount = match content.parse::<f32>() {
                Ok(n) => n,
                Err(_) => 0.0,
            };
        }
        Msg::SaveNewEnteryDate(content) => {
            model.new_entery.date = if content == "".to_string() {
                None
            } else {
                Some(content)
            };
            log!(model.new_entery.date);
        }

        Msg::GetSuggestion => {
            orders.skip().perform_cmd({
                let token = model.ctx.clone().unwrap().token;
                async { Msg::FetchedSuggestion(api::requests::get_finance_suggestion(token).await) }
            });
        }
        Msg::NewFinanceEntery => {
            if &model.new_entery.account_target == "" {
                return;
            }
            orders.skip().perform_cmd({
                let token = model.ctx.clone().unwrap().token;
                let mut new_entery = model.new_entery.clone();
                new_entery.date = match new_entery.date {
                    Some(e) => Some(e.replace("-", "/")),
                    None => None,
                };
                async {
                    Msg::FetchedNewFinanceEntery(
                        api::requests::start_finance_entery(token, new_entery).await,
                    )
                }
            });
        }
        Msg::FetchedNewFinanceEntery(Ok(_response_data)) => {
            model.new_entery = shared::models::NewFinanceEntery::default();
        }
        Msg::FetchedSuggestion(Ok(response_data)) => {
            //log!("{}", &response_data);
            model.suggestions = Some(response_data);
        }
        Msg::FetchedSuggestion(Err(fetch_error))
        | Msg::FetchedNewFinanceEntery(Err(fetch_error)) => {
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
        Some(m) => m.suggestions,
        None => Vec::new(),
    };
    let matcher = SkimMatcherV2::default();
    let threshhold: i64 = model
        .new_entery
        .account_target
        .replace(" ", "")
        .chars()
        .count() as i64
        * 5;

    let suggestion_custom = suggestions.iter().rev().filter(|s| {
        (&model.suggestion_filter == "account_target"
            && matcher
                .fuzzy_match(
                    &s.account_target,
                    &model.new_entery.account_target.replace(" ", ""),
                )
                .unwrap_or(0)
                > threshhold)
            || (&model.suggestion_filter == "account_origin"
                && matcher
                    .fuzzy_match(
                        &s.account_origin,
                        &model.new_entery.account_origin.replace(" ", ""),
                    )
                    .unwrap_or(0)
                    > threshhold)
            || (&model.suggestion_filter == "headline"
                && matcher
                    .fuzzy_match(&s.headline, &&model.new_entery.headline.replace(" ", ""))
                    .unwrap_or(0)
                    > threshhold)
    });

    let empty = if &model.suggestion_filter == "" {
        true
    } else {
        false
    };

    //TODO add unique_by where usefull
    div![
        "Create new Finance Tracking Entery",
        div![
            input![
                C!["input-content_headline"],
                input_ev(Ev::Input, Msg::SaveNewEnteryHeadline),
                attrs! {
                    At::Placeholder => "Headline",
                    At::AutoFocus => true.as_at_value();
                    At::Value => &model.new_entery.headline,
                    At::List => "suggestions_headline",
                },
            ],
            datalist![
                id!["suggestions_headline"],
                suggestions
                    .iter()
                    .rev()
                    .filter(|_s| empty)
                    .unique_by(|s| &s.account_origin)
                    .map(|s| { option![s.headline.clone()] }),
                suggestion_custom
                    .clone()
                    .unique_by(|s| &s.headline)
                    .map(|s| { option![s.headline.clone()] })
            ],
            input![
                C!["input-content_target"],
                input_ev(Ev::Input, Msg::SaveNewEnteryTarget),
                attrs! {
                    At::Placeholder => "Target",
                    At::AutoFocus => true.as_at_value();
                    At::Value => &model.new_entery.account_target,
                    At::List => "suggestions_target",
                }
            ],
            datalist![
                id!["suggestions_target"],
                suggestions
                    .iter()
                    .rev()
                    .filter(|_s| empty)
                    .unique_by(|s| &s.account_target)
                    .map(|s| { option![s.account_target.clone()] }),
                suggestion_custom
                    .clone()
                    .unique_by(|s| &s.account_target)
                    .rev()
                    .map(|s| { option![s.account_target.clone()] })
            ],
            input![
                C!["input-content_origin"],
                input_ev(Ev::Input, Msg::SaveNewEnteryOrigin),
                attrs! {
                    At::Placeholder => "Origin Account",
                    At::AutoFocus => true.as_at_value();
                    At::Value => &model.new_entery.account_origin,
                    At::List => "suggestions_origin",
                }
            ],
            datalist![
                id!["suggestions_origin"],
                //show all if nothing is enterd yet
                suggestions
                    .iter()
                    .rev()
                    .filter(|_s| empty)
                    .unique_by(|s| &s.account_origin)
                    .map(|s| { option![s.account_origin.clone()] }),
                suggestion_custom
                    .clone()
                    .unique_by(|s| &s.account_origin)
                    .rev()
                    .map(|s| { option![s.account_origin.clone()] })
            ],
            input![
                C!["input-content_origin"],
                input_ev(Ev::Input, Msg::SaveNewEnteryAmmount),
                attrs! {
                    At::Placeholder => "Ammount",
                    At::AutoFocus => true.as_at_value();
                    At::Value => &model.new_entery.ammount,
                    At::List => "suggestions_ammount",
                }
            ],
            datalist![
                id!["suggestions_ammount"],
                suggestions
                    .iter()
                    .rev()
                    .filter(|s| matcher
                        .fuzzy_match(
                            &s.account_target,
                            &model.new_entery.account_target.replace(" ", "")
                        )
                        .unwrap_or(0)
                        > threshhold)
                    .map(|s| { option![format!("{:.3}", s.ammount.clone())] }),
            ],
            input![
                C!["input-content_date"],
                input_ev(Ev::Input, Msg::SaveNewEnteryDate),
                attrs! {
                    At::Placeholder => "Date",
                    At::AutoFocus => true.as_at_value();
                    At::Type => "date",
                    At::Value => &model.new_entery.date.clone().unwrap_or("".to_string()),
                }
            ],
            button![ev(Ev::Click, |_| Msg::NewFinanceEntery), "Start Entery"],
        ],
    ]
}

fn update_suggestion_filter(model: &mut Model) {
    model.suggestion_filter = if &model.new_entery.account_origin == ""
        && &model.new_entery.account_target == ""
        && &model.new_entery.headline != ""
    {
        "headline".to_string()
    } else if &model.new_entery.account_target == ""
        && &model.new_entery.account_origin != ""
        && &model.new_entery.headline == ""
    {
        "account_origin".to_string()
    } else if &model.new_entery.account_target != ""
        && &model.new_entery.account_origin == ""
        && &model.new_entery.headline == ""
    {
        "account_target".to_string()
    } else {
        return;
    };
}
