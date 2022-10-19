use crate::api;
use enclose::enc;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
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
    suggestions: Option<shared::models::ResponseBTreeMap>,
    start_entery: shared::models::StartTimeEntery,
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

    FetchedSuggestion(fetch::Result<shared::models::ResponseBTreeMap>),
    FetchedRunningEntery(fetch::Result<shared::models::ResponseRunningLedgerTimeEntery>),
    FetchedStartTimeEntery(fetch::Result<shared::models::ResponseStatus>),

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
}
// ------ ------
//     Urls
// ------ ------

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let data = &mut model.running_entery;
    match msg {
        Msg::SaveNewEnteryHeadline(content) => {
            model.start_entery.headline = content;
        }
        Msg::SaveNewEnteryTarget(content) => {
            model.start_entery.account_target = content;
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
                        offset: running_entery.offset.clone(),
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
                    entery.offset = offset.to_owned();
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
                    At::AutoFocus => true.as_at_value();
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
            "Stop",
            ev(Ev::Click, enc!((id) move |_| Msg::StopTimeEntery(id)))
        ]
    ]
}
