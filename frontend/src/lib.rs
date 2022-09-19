#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

mod api;
mod page;

use api::requests::*;
use shared::{auth::UserLogin, *};

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
        login_data: shared::auth::UserLogin::default(),
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    pub base_url: Url,
    pub page: Page,
    pub ctx: Option<shared::auth::UserLoginResponse>,
    pub login_data: shared::auth::UserLogin,
}

pub enum Page {
    Home,
    NotFound,
}
impl Page {
    fn init(
        mut url: Url,
        _orders: &mut impl Orders<Msg>,
        _ctx: &Option<shared::auth::UserLoginResponse>,
    ) -> Self {
        match url.next_path_part() {
            None => Self::Home,
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

    SaveLoginUsername(String),
    SaveLoginPassword(String),

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

        Msg::SaveLoginUsername(name) => {
            model.login_data.username = name;
        }
        Msg::SaveLoginPassword(pwd) => {
            model.login_data.password = pwd;
        }
        Msg::GetLoginRequest => {
            let name = String::from(&model.login_data.username);
            let pwd = String::from(&model.login_data.password);
            orders
                .skip()
                .perform_cmd(async { Msg::FetchedLogin(get_login(name, pwd).await) });
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
        header(&model.base_url, &model.login_data),
        match &model.page {
            Page::Home => page::home::view(),
            Page::NotFound => page::not_found::view(),
        }
    ]
}

fn header(base_url: &Url, login_data: &UserLogin) -> Node<Msg> {
    div![
        C!["navbar"],
        "Test Navbar",
        li![a![
            attrs! { At::Href => Urls::new(base_url).home() },
            "Home",
        ]],
        div![
            C!["login"],
            input![
                C!["login-name"],
                input_ev(Ev::Input, Msg::SaveLoginUsername),
                attrs! {
                    At::Placeholder => "Name",
                    At::AutoFocus => AtValue::None,
                    At::Value => login_data.username,
                }
            ],
            input![
                C!["login-password"],
                input_ev(Ev::Input, Msg::SaveLoginPassword),
                attrs! {
                    At::Placeholder => "Password",
                    At::AutoFocus => AtValue::None,
                    At::Value => login_data.password,
                }
            ],
            button![ev(Ev::Click, |_| Msg::GetLoginRequest), "Get Login message"],
        ]
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
