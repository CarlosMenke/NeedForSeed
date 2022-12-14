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
        async { Msg::GetSuggestion(token) }
    });
    Model {
        _base_url: url.to_base_url(),
        ctx,
        suggestions: None,
        new_entery: shared::models::NewFinanceEntery::default(),
        suggestion_filter: "".to_string(),
        ammount: "".to_string(),
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
    ammount: String,
}

// ------ Frequency ------

pub enum Msg {
    GetSuggestion(String),
    FetchedNewFinanceEntery(fetch::Result<shared::models::ResponseStatus>),
    FetchedSuggestion(fetch::Result<shared::models::FinanceEnterySuggestion>),

    SaveNewEnteryHeadline(String),
    SaveNewEnteryTarget(String),
    SaveNewEnteryOrigin(String),
    SaveNewEnteryAmmount(String),
    SaveNewEnteryDate(String),
    SaveNewEnteryTargetFile(String),

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
            autofill(orders, model);
        }
        Msg::SaveNewEnteryTarget(content) => {
            model.new_entery.account_target = content;
            update_suggestion_filter(model);
            autofill(orders, model);
        }
        Msg::SaveNewEnteryOrigin(content) => {
            model.new_entery.account_origin = content;
            update_suggestion_filter(model);
            autofill(orders, model);
        }
        Msg::SaveNewEnteryAmmount(content) => {
            model.ammount = content;
        }
        Msg::SaveNewEnteryDate(content) => {
            model.new_entery.date = if content == "".to_string() {
                None
            } else {
                Some(content)
            };
            log!(model.new_entery.date);
        }
        Msg::SaveNewEnteryTargetFile(content) => {
            model.new_entery.target_file = content;
        }

        Msg::GetSuggestion(token) => {
            orders.skip().perform_cmd({
                let token = token;
                async { Msg::FetchedSuggestion(api::requests::get_finance_suggestion(token).await) }
            });
        }
        Msg::NewFinanceEntery => {
            if &model.new_entery.account_target == "" {
                return;
            }
            orders.skip().perform_cmd({
                model.new_entery.ammount = match model.ammount.parse::<f32>() {
                    Ok(n) => n,
                    Err(_) => 0.0,
                };
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
                    .unique_by(|s| &s.headline)
                    .map(|s| { option![s.headline.clone()] }),
                custom_suggestion(&suggestions, model)
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
                custom_suggestion(&suggestions, model)
                    .unique_by(|s| &s.account_target)
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
                custom_suggestion(&suggestions, model)
                    .unique_by(|s| &s.account_origin)
                    .map(|s| { option![s.account_origin.clone()] })
            ],
            input![
                C!["input-content_ammount"],
                input_ev(Ev::Input, Msg::SaveNewEnteryAmmount),
                attrs! {
                    At::Placeholder => "Ammount",
                    At::AutoFocus => true.as_at_value();
                    At::Value => &model.ammount,
                    At::List => "suggestions_ammount",
                }
            ],
            datalist![
                id!["suggestions_ammount"],
                custom_suggestion(&suggestions, model)
                    .unique_by(|s| s.ammount.to_string())
                    .map(|s| { option![format!("{:.2}", s.ammount)] }),
            ],
            input![
                C!["input-content-targetFile"],
                input_ev(Ev::Input, Msg::SaveNewEnteryTargetFile),
                attrs! {
                    At::Placeholder => "Target File",
                    At::AutoFocus => true.as_at_value();
                    At::Value => &model.new_entery.target_file,
                    At::List => "suggestions_target_file",
                }
            ],
            datalist![
                id!["suggestions_target_file"],
                custom_suggestion(&suggestions, model)
                    .unique_by(|s| &s.target_file)
                    .map(|s| { option![s.target_file.clone()] })
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
            button![ev(Ev::Click, |_| Msg::NewFinanceEntery), "Hinzufuegen"],
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
        model.suggestion_filter.clone()
    };
}

fn autofill(orders: &mut impl Orders<Msg>, model: &Model) {
    let suggestions = match model.suggestions.clone() {
        Some(m) => m.suggestions,
        None => Vec::new(),
    };

    let suggestion_custom = custom_suggestion(&suggestions, model)
        .unique_by(|s| &s.headline)
        .map(|s| &s.headline)
        .collect_vec();
    if &suggestion_custom.len() == &(1 as usize) && &model.new_entery.headline == "" {
        let autofill = suggestion_custom[0].to_string().clone();
        orders
            .skip()
            .perform_cmd(async { Msg::SaveNewEnteryHeadline(autofill) });
    }
    let suggestion_custom = custom_suggestion(&suggestions, model)
        .unique_by(|s| &s.account_origin)
        .map(|s| &s.account_origin)
        .collect_vec();
    if &suggestion_custom.len() == &(1 as usize) && &model.new_entery.account_origin == "" {
        let autofill = suggestion_custom[0].to_string().clone();
        orders
            .skip()
            .perform_cmd(async { Msg::SaveNewEnteryOrigin(autofill) });
    }
    let suggestion_custom = custom_suggestion(&suggestions, model)
        .unique_by(|s| &s.account_target)
        .map(|s| &s.account_target)
        .collect_vec();
    if &suggestion_custom.len() == &(1 as usize) && &model.new_entery.account_target == "" {
        let autofill = suggestion_custom[0].to_string().clone();
        orders
            .skip()
            .perform_cmd(async { Msg::SaveNewEnteryTarget(autofill) });
    }
    let suggestion_custom = custom_suggestion(&suggestions, model)
        .map(|s| &s.ammount)
        .collect_vec();
    if &suggestion_custom.len() == &(1 as usize) && &model.ammount == "" {
        let autofill = suggestion_custom[0].to_string().clone();
        orders
            .skip()
            .perform_cmd(async { Msg::SaveNewEnteryAmmount(autofill) });
    }
    let suggestion_custom = custom_suggestion(&suggestions, model)
        .unique_by(|s| &s.target_file)
        .map(|s| &s.target_file)
        .collect_vec();
    if &suggestion_custom.len() == &(1 as usize) && &model.new_entery.target_file == "" {
        let autofill = suggestion_custom[0].to_string().clone();
        orders
            .skip()
            .perform_cmd(async { Msg::SaveNewEnteryTargetFile(autofill) });
    }
}

pub fn custom_suggestion<'a>(
    suggestions: &'a Vec<shared::models::NewFinanceEntery>,
    model: &'a Model,
) -> impl Iterator<Item = &'a shared::models::NewFinanceEntery> {
    let matcher = SkimMatcherV2::default();
    let threshhold: i64 = model
        .new_entery
        .account_target
        .replace(" ", "")
        .chars()
        .count() as i64
        * 5;
    //autofill
    return suggestions
        .iter()
        .rev()
        .filter(move |s| {
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
        })
        .rev();
}
