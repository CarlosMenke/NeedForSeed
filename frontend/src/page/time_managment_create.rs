use crate::api;
use enclose::enc;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use itertools::Itertools;
use seed::{prelude::*, *};
use std::collections::BTreeMap;
use web_sys::HtmlInputElement;

const ENTER_KEY: u32 = 13;
const ESC_KEY: u32 = 27;

type RunningEnteryId = String;

#[derive(Clone, Debug)]
pub struct EditingNewTimeEntery {
    pub id: RunningEnteryId,
    pub offset: i32,
}

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
        suggestion_filter: "".to_string(),
        running_entery: None,
        editing_offset: None,
        refs: Refs::default(),
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    _base_url: Url,
    ctx: Option<shared::auth::UserLoginResponse>,
    suggestions: Option<shared::models::HeadlineSuggestion>,
    start_entery: shared::models::StartTimeEntery,
    suggestion_filter: String,
    running_entery: Option<shared::models::ResponseRunningLedgerTimeEntery>,
    editing_offset: Option<EditingNewTimeEntery>,
    refs: Refs,
}

#[derive(Default)]
struct Refs {
    editing_running_entery_input: ElRef<HtmlInputElement>,
}

// ------ Frequency ------

pub enum Msg {
    GetSuggestion,

    FetchedSuggestion(fetch::Result<shared::models::HeadlineSuggestion>),
    FetchedRunningEntery(fetch::Result<shared::models::ResponseRunningLedgerTimeEntery>),
    FetchedStartTimeEntery(fetch::Result<shared::models::ResponseStatus>),
    FetchedKillTimeEntery(fetch::Result<shared::models::ResponseStatus>),

    SaveNewEnteryHeadline(String),
    SaveNewEnteryTarget(String),
    SaveNewEnteryDuration(String),
    SaveNewEnteryDate(String),
    SaveNewEnteryOffset(String),

    StartOffsetEdit(RunningEnteryId),
    EditingRunningEnteryOffsetChanged(String),
    SaveEditingRunningEnteryOffset,
    CancelRunningEnteryOffsetEdit,

    StartTimeEntery,
    StopTimeEntery(RunningEnteryId),
    KillTimeEntery(RunningEnteryId),
}
// ------ ------
//     Urls
// ------ ------

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let data = &mut model.running_entery;
    match msg {
        Msg::SaveNewEnteryHeadline(content) => {
            model.start_entery.headline = content;
            update_suggestion_filter(model);
            autofill(orders, model);
        }
        Msg::SaveNewEnteryTarget(content) => {
            model.start_entery.account_target = content;
            update_suggestion_filter(model);
            autofill(orders, model);
        }
        Msg::SaveNewEnteryDuration(content) => {
            model.start_entery.duration = match content.parse::<u32>() {
                Ok(0) => None,
                Ok(n) => Some(n),
                Err(_) => None,
            };
        }
        Msg::SaveNewEnteryDate(content) => {
            model.start_entery.date = if content == "".to_string() {
                None
            } else {
                Some(content)
            };
            log!(model.start_entery.date);
        }
        Msg::SaveNewEnteryOffset(content) => {
            model.start_entery.offset = match content.parse::<i32>() {
                Ok(0) => None,
                Ok(n) => Some(n),
                Err(_) => None,
            };
        }

        Msg::StartOffsetEdit(running_entery_id) => {
            if let Some(running_entery) = model
                .running_entery
                .as_ref()
                .unwrap()
                .running_entery
                .get(&running_entery_id)
            {
                model.editing_offset = Some({
                    EditingNewTimeEntery {
                        id: running_entery_id,
                        offset: running_entery.offset.unwrap_or(0),
                    }
                });
            }

            let input = model.refs.editing_running_entery_input.clone();
            orders.after_next_render(move |_| {
                input
                    .get()
                    .expect("get `editing_running_entery_input`")
                    .select();
            });
        }
        Msg::EditingRunningEnteryOffsetChanged(offset) => {
            if let Some(ref mut editing_running_entery) = model.editing_offset {
                editing_running_entery.offset = offset.parse::<i32>().unwrap_or(0);
            }
        }
        Msg::SaveEditingRunningEnteryOffset => {
            if let Some(editing_offset) = model.editing_offset.take() {
                let offset = editing_offset.offset;
                if offset == 0 {
                } else if let Some(entery) = match data {
                    Some(e) => e.running_entery.get_mut(&editing_offset.id),
                    None => None,
                } {
                    entery.offset = Some(offset.to_owned());
                }
            }
            log!("{:#?}", &model.running_entery);
        }

        Msg::CancelRunningEnteryOffsetEdit => {
            model.editing_offset = None;
        }
        Msg::GetSuggestion => {
            orders.skip().perform_cmd({
                let token = model.ctx.clone().unwrap().token;
                async { Msg::FetchedSuggestion(api::requests::get_time_suggestion(token).await) }
            });
        }
        Msg::StartTimeEntery => {
            if &model.start_entery.account_target == "" {
                return;
            }
            orders.skip().perform_cmd({
                let token = model.ctx.clone().unwrap().token;
                let mut start_entery = model.start_entery.clone();
                start_entery.date = match start_entery.date {
                    Some(e) => Some(e.replace("-", "/")),
                    None => None,
                };
                async {
                    Msg::FetchedStartTimeEntery(
                        api::requests::start_time_entery(token, start_entery).await,
                    )
                }
            });
        }
        Msg::StopTimeEntery(remove_line) => {
            //save offset if input is present
            if let Some(editing_offset) = model.editing_offset.take() {
                let offset = editing_offset.offset;
                if offset == 0 {
                } else if let Some(entery) = match data {
                    Some(e) => e.running_entery.get_mut(&editing_offset.id),
                    None => None,
                } {
                    entery.offset = Some(offset.to_owned());
                }
            }
            log!("{:#?}", &model.running_entery);
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
        Msg::KillTimeEntery(remove_line) => {
            orders.skip().perform_cmd({
                let token = model.ctx.clone().unwrap().token;
                let stop_entery = shared::models::StopLedgerTimeEntery {
                    remove_line,
                    new_entery: shared::models::NewTimeEntery::default(),
                };
                log!(stop_entery);
                async {
                    Msg::FetchedKillTimeEntery(
                        api::requests::kill_time_entery(token, stop_entery).await,
                    )
                }
            });
        }
        Msg::FetchedStartTimeEntery(Ok(_response_data))
        | Msg::FetchedKillTimeEntery(Ok(_response_data)) => {
            model.start_entery = shared::models::StartTimeEntery::default();
            orders.skip().perform_cmd({
                let token = model.ctx.clone().unwrap().token;
                async {
                    Msg::FetchedRunningEntery(api::requests::get_time_running_entery(token).await)
                }
            });
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
        | Msg::FetchedStartTimeEntery(Err(fetch_error))
        | Msg::FetchedKillTimeEntery(Err(fetch_error)) => {
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
    let running_entery = match model.running_entery.clone() {
        Some(m) => m.running_entery,
        None => BTreeMap::new(),
    };
    let empty = if &model.suggestion_filter == "" {
        true
    } else {
        false
    };
    div![
        "Create new time Tracking Entery",
        div![
            input![
                C!["input-content-headline"],
                input_ev(Ev::Input, Msg::SaveNewEnteryHeadline),
                attrs! {
                    At::Placeholder => "Headline",
                    At::AutoFocus => true.as_at_value();
                    At::Value => &model.start_entery.headline,
                    At::List => "suggestions-headline",
                },
            ],
            datalist![
                id!["suggestions-headline"],
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
                    At::Value => &model.start_entery.account_target,
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
                C!["input-content_duration"],
                input_ev(Ev::Input, Msg::SaveNewEnteryDuration),
                attrs! {
                    At::Placeholder => "Duration",
                    At::AutoFocus => true.as_at_value();
                    At::Value => &model.start_entery.duration.clone().unwrap_or(0),
                }
            ],
            input![
                C!["input-content_date"],
                input_ev(Ev::Input, Msg::SaveNewEnteryDate),
                attrs! {
                    At::Placeholder => "Date",
                    At::AutoFocus => true.as_at_value();
                    At::Type => "date",
                    At::Value => &model.start_entery.date.clone().unwrap_or("".to_string()),
                }
            ],
            input![
                C!["input-content_offset"],
                input_ev(Ev::Input, Msg::SaveNewEnteryOffset),
                attrs! {
                    At::Placeholder => "Offset",
                    At::AutoFocus => true.as_at_value();
                    At::Value => &model.start_entery.offset.clone().unwrap_or(0),
                }
            ],
            button![ev(Ev::Click, |_| Msg::StartTimeEntery), "Start Entery"],
            ul![running_entery.iter().filter_map(|(remove_line, entery)| {
                Some(view_runing_enteries(
                    remove_line.to_string(),
                    entery,
                    &model.editing_offset,
                    &model.refs.editing_running_entery_input,
                ))
            })]
        ],
    ]
}

//TODO add change name / headline of running entery
fn view_runing_enteries(
    id: RunningEnteryId,
    entery: &shared::models::NewTimeEntery,
    editing_running_entery: &Option<EditingNewTimeEntery>,
    editing_running_entery_input: &ElRef<HtmlInputElement>,
) -> Node<Msg> {
    //TODO use entery for button name
    div![
        style! {St::Display => "flex", St::FlexDirection => "column", St::JustifyContent => "flex-start" , St::MaxWidth => px(500), St::Margin => "auto", St::MarginTop => px(30)},
        p![entery.headline.clone()],
        p![entery.account_target.clone()],
        p![format!("Duration: {}", entery.duration)],
        div![
            style! {St::Display => "flex", St::FlexDirection => "row", St::JustifyContent => "flex-start", St::MarginBottom => px(10)},
            label![
                C!["input-running_entery_offset"],
                ev(Ev::DblClick, enc!((id) move |_| Msg::StartOffsetEdit(id))),
                "Offset: ",
            ],
            match editing_running_entery {
                Some(editing_running_entery) if editing_running_entery.id == id => {
                    input![
                        el_ref(editing_running_entery_input),
                        C!["input"],
                        attrs! {At::Value => editing_running_entery.offset},
                        input_ev(Ev::Input, Msg::EditingRunningEnteryOffsetChanged),
                        keyboard_ev(Ev::KeyDown, |keyboard_event| {
                            match keyboard_event.key_code() {
                                ENTER_KEY => Some(Msg::SaveEditingRunningEnteryOffset),
                                ESC_KEY => Some(Msg::CancelRunningEnteryOffsetEdit),
                                _ => None,
                            }
                        }),
                    ]
                }
                _ => label![entery.offset.clone()],
            },
        ],
        button![
            "kill",
            ev(Ev::Click, enc!((id) move |_| Msg::KillTimeEntery(id)))
        ],
        button![
            "Stop",
            ev(Ev::Click, enc!((id) move |_| Msg::StopTimeEntery(id)))
        ]
    ]
}

fn update_suggestion_filter(model: &mut Model) {
    model.suggestion_filter =
        if &model.start_entery.account_target == "" && &model.start_entery.headline != "" {
            "headline".to_string()
        } else if &model.start_entery.account_target != "" && &model.start_entery.headline == "" {
            "account_target".to_string()
        } else {
            model.suggestion_filter.clone()
        };
}

pub fn custom_suggestion<'a>(
    suggestions: &'a Vec<shared::models::TimeEnterySuggestion>,
    model: &'a Model,
) -> impl Iterator<Item = &'a shared::models::TimeEnterySuggestion> {
    let matcher = SkimMatcherV2::default();
    let threshhold: i64 = model
        .start_entery
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
                        &model.start_entery.account_target.replace(" ", ""),
                    )
                    .unwrap_or(0)
                    > threshhold)
                || (&model.suggestion_filter == "headline"
                    && matcher
                        .fuzzy_match(&s.headline, &&model.start_entery.headline.replace(" ", ""))
                        .unwrap_or(0)
                        > threshhold)
        })
        .rev();
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
    if &suggestion_custom.len() == &(1 as usize) && &model.start_entery.headline == "" {
        let autofill = suggestion_custom[0].to_string().clone();
        orders
            .skip()
            .perform_cmd(async { Msg::SaveNewEnteryHeadline(autofill) });
    }

    let suggestion_custom = custom_suggestion(&suggestions, model)
        .unique_by(|s| &s.account_target)
        .map(|s| &s.account_target)
        .collect_vec();
    if &suggestion_custom.len() == &(1 as usize) && &model.start_entery.account_target == "" {
        let autofill = suggestion_custom[0].to_string().clone();
        orders
            .skip()
            .perform_cmd(async { Msg::SaveNewEnteryTarget(autofill) });
    }
}
