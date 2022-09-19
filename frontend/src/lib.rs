#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

mod api;
mod page;

use api::requests::*;
use shared::*;

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    log!("Base URL {:?}", url);
    orders.subscribe(Msg::UrlChanged);
    orders
        .subscribe(Msg::UrlChanged)
        .notify(subs::UrlChanged(url.clone()));
    Model {
        base_url: url.to_base_url(),
        page: Page::init(url, orders, &None),
        ctx: None,
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    pub base_url: Url,
    pub page: Page,
    pub ctx: Option<shared::auth::UserLoginResponse>,
}

pub enum Page {
    NotFound,
}
impl Page {
    fn init(
        mut url: Url,
        orders: &mut impl Orders<Msg>,
        ctx: &Option<shared::auth::UserLoginResponse>,
    ) -> Self {
        match url.next_path_part() {
            _ => Self::NotFound,
        }
    }
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    fn home(self) -> Url {
        self.base_url()
    }
}

pub enum Msg {
    UrlChanged(subs::UrlChanged),
    GoToUrl(Url),

    GetLoginRequest,
    FetchedLogin(fetch::Result<auth::UserLoginResponse>),
}

// ------ ------
//    Update
// ------ ------

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => model.page = Page::init(url, orders, &model.ctx),
        //TODO check if needed
        Msg::GoToUrl(url) => {
            orders.request_url(url);
        }

        Msg::GetLoginRequest => {
            orders.skip().perform_cmd({
                let name = "Carlos".to_string();
                let pwd = "jkl".to_string();
                async { Msg::FetchedLogin(get_login(name, pwd).await) }
            });
        }
        Msg::FetchedLogin(Ok(response_data)) => {
            log!("fetched data: {:?}", &response_data);
            model.ctx = Some(response_data);
        }

        Msg::FetchedLogin(Err(fetch_error)) => {
            log!("Example_A error:", fetch_error);
            orders.skip();
        } // ------- Page
    };
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        header(&model.base_url),
        match &model.page {
            Page::NotFound => page::not_found::view(),
        }
    ]
}

fn header(base_url: &Url) -> Node<Msg> {
    div![
        "Test Navbar",
        li![a![
            attrs! { At::Href => Urls::new(base_url).home() },
            "Home",
        ]],
        button![ev(Ev::Click, |_| Msg::GetLoginRequest), "Get Login message"],
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
