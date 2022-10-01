use crate::api;
use seed::{prelude::*, *};
use std::collections::BTreeMap;

// ------ ------
//     Init
// ------ ------

pub fn init(
    mut url: Url,
    orders: &mut impl Orders<Msg>,
    ctx: Option<shared::auth::UserLoginResponse>,
) -> Model {
    let base_url = url.to_base_url();
    orders.skip().perform_cmd({
        let token = ctx.clone().unwrap().token;
        async { Msg::FetchedSuggestion(api::requests::get_time_suggestion(token).await) }
    });
    Model {
        base_url,
        ctx,
        suggestions: None,
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    base_url: Url,
    ctx: Option<shared::auth::UserLoginResponse>,
    suggestions: Option<shared::models::ResponseBTreeMap>,
}

// ------ Frequency ------

pub enum Msg {
    GetSummary,
    FetchedSuggestion(fetch::Result<shared::models::ResponseBTreeMap>),
}
// ------ ------
//     Urls
// ------ ------

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::GetSummary => {
            orders.skip().perform_cmd({
                let token = model.ctx.clone().unwrap().token;
                async { Msg::FetchedSuggestion(api::requests::get_time_suggestion(token).await) }
            });
        }
        Msg::FetchedSuggestion(Ok(response_data)) => {
            log!(response_data);
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
        Some(m) => m.hash_map,
        None => BTreeMap::new(),
    };

    div![
        "Create new time Tracking Entery",
        div![
            input![
                C!["input-content"],
                //input_ev(Ev::Input, Msg::SaveNewEntery),
                attrs! {
                    At::Placeholder => "Name",
                    At::AutoFocus => AtValue::None,
                    //At::Value => new_entery.content,
                    At::List => "suggestions",
                }
            ],
            datalist![
                id!["suggestions"],
                suggestions
                    .iter()
                    .map(|(content, _headline)| { option![content] })
            ]
        ],
    ]
}
