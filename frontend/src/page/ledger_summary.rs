use crate::api;
use chrono::*;
use itertools::Itertools;
use seed::{prelude::*, *};

use crate::design::General;

// ------ ------
//     Init
// ------ ------

pub fn init(
    _url: Url,
    orders: &mut impl Orders<Msg>,
    ctx: Option<shared::auth::UserLoginResponse>,
    api_target: String,
) -> Model {
    log!(api_target);
    let dt = chrono::Local::now();
    let selected = shared::models::HtmlSuggestion {
        target: api_target.clone(),
        date: format!("{:02}_{:02}_01", dt.year(), dt.month()),
        timespan: if FINANCE == &api_target {
            "month".to_string()
        } else {
            "day".to_string()
        },
        depth: "all".to_string(),
    };
    log!(selected);
    //TODO make more general
    let mut selection_input = selected.clone();
    selection_input.target = api_target.clone();
    orders.skip().perform_cmd({
        let token = ctx.clone().unwrap().token;
        let selected = selected.clone();
        async { Msg::FetchedSummary(api::requests::get_html(token, selected).await) }
    });
    orders.skip().perform_cmd({
        let token = ctx.clone().unwrap().token;
        async { Msg::FetchedSuggestion(api::requests::get_html_suggestion(token).await) }
    });
    Model {
        ctx,
        _api_target: api_target,

        selected,
        selection_input,
        suggestions: None,

        summary: None,
        suggestion_filter: "".to_string(),
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    ctx: Option<shared::auth::UserLoginResponse>,
    _api_target: String,

    selected: shared::models::HtmlSuggestion,
    summary: Option<shared::models::ResponseHtml>,
    suggestions: Option<shared::models::ResponseHtmlSuggestion>,

    selection_input: shared::models::HtmlSuggestion,
    suggestion_filter: String,
}

const FINANCE: &str = "finance";

pub enum Msg {
    FetchedSummary(fetch::Result<shared::models::ResponseHtml>),
    FetchedSuggestion(fetch::Result<shared::models::ResponseHtmlSuggestion>),

    SaveTimespan(String),
    SaveDate(String),
    SaveDepth(String),
    ClearTimespan,
    ClearDate,
    ClearDepth,
    SaveSelection,
}

// ------ ------
//     Update
// ------ ------

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ClearTimespan => {
            model.selection_input.timespan = String::new();
            model.selection_input.date = String::new();
        }
        Msg::ClearDate => {
            model.selection_input.date = String::new();
        }
        Msg::ClearDepth => {
            model.selection_input.depth = String::new();
        }
        Msg::SaveTimespan(content) => {
            model.selection_input.timespan = content;
            update_suggestion_filter(model);
            autofill(orders, model);
            orders.skip().perform_cmd(async { Msg::SaveSelection });
        }
        Msg::SaveDate(content) => {
            model.selection_input.date = content;
            update_suggestion_filter(model);
            autofill(orders, model);
            orders.skip().perform_cmd(async { Msg::SaveSelection });
        }
        Msg::SaveDepth(content) => {
            model.selection_input.depth = content;
            update_suggestion_filter(model);
            autofill(orders, model);
            orders.skip().perform_cmd(async { Msg::SaveSelection });
        }
        Msg::SaveSelection => {
            if model.selected == model.selection_input {
                return;
            };
            model.selected = model.selection_input.clone();
            orders.skip().perform_cmd({
                let token = model.ctx.clone().unwrap().token;
                let selected = model.selected.clone();
                async { Msg::FetchedSummary(api::requests::get_html(token, selected).await) }
            });
        }
        Msg::FetchedSuggestion(Ok(response_data)) => {
            model.suggestions = Some(response_data);
        }
        Msg::FetchedSummary(Ok(response_data)) => {
            model.summary = Some(response_data);
            model.suggestion_filter = "".to_string();
        }
        Msg::FetchedSummary(Err(fetch_error)) | Msg::FetchedSuggestion(Err(fetch_error)) => {
            log!("Fetch error:", fetch_error);
            orders.skip();
        }
    }
}
// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    let summary_html = match model.summary.clone() {
        Some(m) => m.html,
        None => "".to_string(),
    };
    let suggestions = match model.suggestions.clone() {
        Some(m) => m.suggestions,
        None => Vec::new(),
    };
    let general = General::default();
    let empty_timespan = if &model.selection_input.timespan == "" {
        true
    } else {
        false
    };
    div![
        div![
            C!["selection"],
            style! {
                St::Padding => "25px 15px",
                St::Margin => "0px auto",
                St::Width => px(250),
            },
            input![
                C!["input-content-timespan"],
                input_ev(Ev::Input, Msg::SaveTimespan),
                ev(Ev::Click, |_| Msg::ClearTimespan),
                attrs! {
                    At::Placeholder => "Timespan",
                    At::AutoFocus => true.as_at_value();
                    At::Value => &model.selection_input.timespan,
                    At::List => "suggestions-timespan",
                },
                &general.input,
                style! {
                    St::BorderRadius => px(25),
                    St::BackgroundColor => "#04a9b5",
                    St::Color => "04a9b5",
                },
            ],
            datalist![
                id!["suggestions-timespan"],
                suggestions
                    .iter()
                    .filter(|s| empty_timespan && s.target == model.selected.target)
                    .unique_by(|s| &s.timespan)
                    .map(|s| { option![s.timespan.clone()] }),
            ],
            input![
                C!["input-content-date"],
                input_ev(Ev::Input, Msg::SaveDate),
                ev(Ev::Click, |_| Msg::ClearDate),
                attrs! {
                    At::Placeholder => "Date",
                    At::AutoFocus => true.as_at_value();
                    At::Value => &model.selection_input.date,
                    At::List => "suggestions-date",
                },
                &general.input,
                style! {
                        St::BorderRadius => px(25),
                        St::BackgroundColor => "#04a9b5",
                        St::Color => "04a9b5",
                },
            ],
            datalist![
                id!["suggestions-date"],
                suggestions
                    .iter()
                    .filter(|s| empty_timespan && s.target == model.selected.target)
                    .unique_by(|s| &s.date)
                    .map(|s| { option![s.date.clone()] }),
                suggestions
                    .iter()
                    .filter(move |s| {
                        model.selection_input.target == s.target
                            && model.selection_input.timespan == s.timespan
                    })
                    .unique_by(|s| &s.date)
                    .map(|s| { option![s.date.clone()] })
                    .rev()
            ],
            input![
                C!["input-content-depth"],
                input_ev(Ev::Input, Msg::SaveDepth),
                ev(Ev::Click, |_| Msg::ClearDepth),
                attrs! {
                    At::Placeholder => "Depth",
                    At::AutoFocus => true.as_at_value();
                    At::Value => &model.selection_input.depth,
                    At::List => "suggestions-depth",
                },
                &general.input,
                style! {
                        St::BorderRadius => px(25),
                        St::BackgroundColor => "#04a9b5",
                        St::Color => "04a9b5",
                },
            ],
            datalist![
                id!["suggestions-depth"],
                suggestions
                    .iter()
                    .filter(|s| empty_timespan && s.target == model.selected.target)
                    .unique_by(|s| &s.depth)
                    .map(|s| { option![s.depth.clone()] }),
                suggestions
                    .iter()
                    .filter(move |s| {
                        model.selection_input.target == s.target
                            && model.selection_input.timespan == s.timespan
                            && if model.selection_input.date != "" {
                                model.selection_input.date == s.date
                            } else {
                                true
                            }
                    })
                    .unique_by(|s| &s.depth)
                    .map(|s| { option![s.depth.clone()] })
            ],
        ],
        div![
            raw![&summary_html],
            style! { St::Margin => "40px 40px 40px 40px"},
        ],
    ]
}

fn update_suggestion_filter(model: &mut Model) {
    model.suggestion_filter = if &model.selection_input.timespan == ""
        && &model.selection_input.date == ""
        && &model.selection_input.depth != ""
    {
        "depth".to_string()
    } else if &model.selection_input.date == ""
        && &model.selection_input.timespan != ""
        && &model.selection_input.depth == ""
    {
        "timespan".to_string()
    } else if &model.selection_input.date != ""
        && &model.selection_input.timespan == ""
        && &model.selection_input.depth == ""
    {
        "date".to_string()
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
        .unique_by(|s| &s.depth)
        .map(|s| &s.depth)
        .collect_vec();
    if &suggestion_custom.len() == &(1 as usize) && &model.selection_input.depth == "" {
        let autofill = suggestion_custom[0].to_string().clone();
        orders
            .skip()
            .perform_cmd(async { Msg::SaveDepth(autofill) });
    }
    let suggestion_custom = custom_suggestion(&suggestions, model)
        .unique_by(|s| &s.date)
        .map(|s| &s.date)
        .collect_vec();
    if &suggestion_custom.len() == &(1 as usize) && &model.selection_input.date == "" {
        let autofill = suggestion_custom[0].to_string().clone();
        orders.skip().perform_cmd(async { Msg::SaveDate(autofill) });
    }
    let suggestion_custom = custom_suggestion(&suggestions, model)
        .unique_by(|s| &s.timespan)
        .map(|s| &s.timespan)
        .collect_vec();
    if &suggestion_custom.len() == &(1 as usize) && &model.selection_input.timespan == "" {
        let autofill = suggestion_custom[0].to_string().clone();
        orders
            .skip()
            .perform_cmd(async { Msg::SaveTimespan(autofill) });
    }
}

pub fn custom_suggestion<'a>(
    suggestions: &'a Vec<shared::models::HtmlSuggestion>,
    model: &'a Model,
) -> impl Iterator<Item = &'a shared::models::HtmlSuggestion> {
    //TODO filter is not working
    return suggestions.iter().filter(move |s| {
        model.selection_input.target == s.target && model.selection_input.timespan == s.timespan
    });
}
