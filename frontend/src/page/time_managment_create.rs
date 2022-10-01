use crate::api;
use seed::{prelude::*, *};
use std::collections::HashMap;

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
    suggestions: Option<shared::models::ResponseHashMap>,
}

// ------ Frequency ------

pub enum Msg {
    GetSummary,
    FetchedSuggestion(fetch::Result<shared::models::ResponseHashMap>),
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
        None => HashMap::new(),
    };

    div!["Create new time Tracking Entery",]
}
