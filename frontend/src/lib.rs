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

const MUSIC: &str = "Music";
const FINANCE: &str = "Finance";
const TIMEMANAGMENT: &str = "TimeManagment";
const TIMEMANAGMENTCREATE: &str = "TimeManagmentCreate";
pub enum Page {
    Home,
    Music(page::music::Model),
    Finance(page::finance::Model),
    TimeManagment(page::time_managment::Model),
    TimeManagmentCreate(page::time_managment_create::Model),
    NotFound,
}
impl Page {
    fn init(
        mut url: Url,
        orders: &mut impl Orders<Msg>,
        ctx: &Option<shared::auth::UserLoginResponse>,
    ) -> Self {
        match url.next_path_part() {
            Some(MUSIC) => Self::Music(page::music::init(
                url,
                &mut orders.proxy(Msg::MusicMsg),
                ctx.clone(),
            )),
            Some(FINANCE) => Self::Finance(page::finance::init(
                url,
                &mut orders.proxy(Msg::FinanceMsg),
                ctx.clone(),
            )),
            Some(TIMEMANAGMENT) => Self::TimeManagment(page::time_managment::init(
                url,
                &mut orders.proxy(Msg::TimeManagmentMsg),
                ctx.clone(),
            )),
            Some(TIMEMANAGMENTCREATE) => {
                Self::TimeManagmentCreate(page::time_managment_create::init(
                    url,
                    &mut orders.proxy(Msg::TimeManagmentCreateMsg),
                    ctx.clone(),
                ))
            }
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
    fn music(self) -> page::music::Urls<'a> {
        page::music::Urls::new(self.base_url().add_path_part(MUSIC))
    }
    fn finance(self) -> page::finance::Urls<'a> {
        page::finance::Urls::new(self.base_url().add_path_part(FINANCE))
    }
    fn time_managment(self) -> page::time_managment::Urls<'a> {
        page::time_managment::Urls::new(self.base_url().add_path_part(TIMEMANAGMENT))
    }
    fn time_managment_create(self) -> Url {
        self.base_url().add_path_part(TIMEMANAGMENTCREATE)
    }
    fn home(self) -> Url {
        self.base_url()
    }
}

pub enum Msg {
    UrlChanged(subs::UrlChanged),
    GoToUrl(Url),
    // ----- Page Msg
    MusicMsg(page::music::Msg),
    FinanceMsg(page::finance::Msg),
    TimeManagmentMsg(page::time_managment::Msg),
    TimeManagmentCreateMsg(page::time_managment_create::Msg),

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
        }
        // ------- Page -------
        Msg::MusicMsg(msg) => {
            if let Page::Music(model) = &mut model.page {
                page::music::update(msg, model, &mut orders.proxy(Msg::MusicMsg))
            }
        }
        Msg::FinanceMsg(msg) => {
            if let Page::Finance(model) = &mut model.page {
                page::finance::update(msg, model, &mut orders.proxy(Msg::FinanceMsg))
            }
        }
        Msg::TimeManagmentMsg(msg) => {
            if let Page::TimeManagment(model) = &mut model.page {
                page::time_managment::update(msg, model, &mut orders.proxy(Msg::TimeManagmentMsg))
            }
        }
        Msg::TimeManagmentCreateMsg(msg) => {
            if let Page::TimeManagmentCreate(model) = &mut model.page {
                page::time_managment_create::update(
                    msg,
                    model,
                    &mut orders.proxy(Msg::TimeManagmentCreateMsg),
                )
            }
        }
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        IF!( ! &model.ctx.is_none() => header(&model.base_url)),
        IF!( model.ctx.is_none() => view_login(&model.login_data)),
        match &model.page {
            Page::Home => page::home::view(),
            Page::Music(model) => page::music::view(&model).map_msg(Msg::MusicMsg),
            Page::Finance(model) => page::finance::view(&model).map_msg(Msg::FinanceMsg),
            Page::TimeManagment(model) =>
                page::time_managment::view(&model).map_msg(Msg::TimeManagmentMsg),
            Page::TimeManagmentCreate(model) =>
                page::time_managment_create::view(&model).map_msg(Msg::TimeManagmentCreateMsg),
            Page::NotFound => page::not_found::view(),
        }
    ]
}

fn header(base_url: &Url) -> Node<Msg> {
    div![
        C!["navbar"],
        "Test Navbar",
        li![a![
            attrs! { At::Href => Urls::new(base_url).home() },
            "Home",
        ]],
        li![a![
            attrs! { At::Href => Urls::new(base_url).music().default() },
            "Music",
        ]],
        li![a![
            attrs! { At::Href => Urls::new(base_url).finance().default() },
            "Finance",
        ]],
        li![a![
            attrs! { At::Href => Urls::new(base_url).time_managment().default() },
            "Time Tracking",
        ]],
        li![a![
            attrs! { At::Href => Urls::new(base_url).time_managment_create() },
            "Time Tracking Create Entery",
        ]],
    ]
}

fn view_login(login_data: &auth::UserLogin) -> Node<Msg> {
    let names: Vec<&str> = Vec::from([
        "Carlos",
        "Andrea",
        "Mira",
        "Pascal",
        "Education:Uni",
        "Education:Programming",
    ]);
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
                At::Type => "Password",
            }
        ],
        button![ev(Ev::Click, |_| Msg::GetLoginRequest), "Get Login message"],
        C!["login"],
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
