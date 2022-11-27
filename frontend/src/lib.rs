#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

mod api;
mod design;
mod page;

use api::requests::*;
use design::General;
use shared::*;

const STORAGE_KEY_CTX: &str = "ctx";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    log!("Base URL {:?}", url);
    orders.subscribe(Msg::UrlChanged);
    orders
        .subscribe(Msg::UrlChanged)
        .notify(subs::UrlChanged(url.clone()));
    let ctx = LocalStorage::get(STORAGE_KEY_CTX).ok();
    Model {
        base_url: url.to_base_url(),
        page: Page::init(url, orders, &None),
        ctx,
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
const CALNEDER: &str = "Calender";
const TIMEMANAGMENTCREATE: &str = "TimeManagmentCreate";
const FINANCEMANAGMENTCREATE: &str = "FinanceManagmentCreate";
pub enum Page {
    Home,
    LedgerSummary(page::ledger_summary::Model),
    TimeManagmentCreate(page::time_managment_create::Model),
    FinanceManagmentCreate(page::finance_managment_create::Model),
    NotFound,
}
impl Page {
    fn init(
        mut url: Url,
        orders: &mut impl Orders<Msg>,
        ctx: &Option<shared::auth::UserLoginResponse>,
    ) -> Self {
        match url.next_path_part() {
            Some(MUSIC) => Self::LedgerSummary(page::ledger_summary::init(
                url,
                &mut orders.proxy(Msg::LedgerSummaryMsg),
                ctx.clone(),
                "music".to_string().clone(),
            )),
            Some(FINANCE) => Self::LedgerSummary(page::ledger_summary::init(
                url,
                &mut orders.proxy(Msg::LedgerSummaryMsg),
                ctx.clone(),
                "finance".to_string().clone(),
            )),
            Some(TIMEMANAGMENT) => Self::LedgerSummary(page::ledger_summary::init(
                url,
                &mut orders.proxy(Msg::LedgerSummaryMsg),
                ctx.clone(),
                "timeManagment".to_string().clone(),
            )),
            Some(CALNEDER) => Self::LedgerSummary(page::ledger_summary::init(
                url,
                &mut orders.proxy(Msg::LedgerSummaryMsg),
                ctx.clone(),
                "calender".to_string().clone(),
            )),
            Some(TIMEMANAGMENTCREATE) => {
                Self::TimeManagmentCreate(page::time_managment_create::init(
                    url,
                    &mut orders.proxy(Msg::TimeManagmentCreateMsg),
                    ctx.clone(),
                ))
            }
            Some(FINANCEMANAGMENTCREATE) => {
                Self::FinanceManagmentCreate(page::finance_managment_create::init(
                    url,
                    &mut orders.proxy(Msg::FinanceManagmentCreateMsg),
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
    fn music(self) -> page::ledger_summary::Urls<'a> {
        page::ledger_summary::Urls::new(self.base_url().add_path_part(MUSIC))
    }
    fn finance(self) -> page::ledger_summary::Urls<'a> {
        page::ledger_summary::Urls::new(self.base_url().add_path_part(FINANCE))
    }
    fn calender(self) -> page::ledger_summary::Urls<'a> {
        page::ledger_summary::Urls::new(self.base_url().add_path_part(CALNEDER))
    }
    fn ledger_summary(self) -> page::ledger_summary::Urls<'a> {
        page::ledger_summary::Urls::new(self.base_url().add_path_part(TIMEMANAGMENT))
    }
    fn time_managment_create(self) -> Url {
        self.base_url().add_path_part(TIMEMANAGMENTCREATE)
    }
    fn finance_managment_create(self) -> Url {
        self.base_url().add_path_part(FINANCEMANAGMENTCREATE)
    }
    fn home(self) -> Url {
        self.base_url()
    }
}

pub enum Msg {
    UrlChanged(subs::UrlChanged),
    GoToUrl(Url),
    // ----- Page Msg
    LedgerSummaryMsg(page::ledger_summary::Msg),
    TimeManagmentCreateMsg(page::time_managment_create::Msg),
    FinanceManagmentCreateMsg(page::finance_managment_create::Msg),

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
            LocalStorage::insert(STORAGE_KEY_CTX, &response_data)
                .expect("Failed to insert CTX to Local Storage.");
            model.ctx = Some(response_data);
        }

        Msg::FetchedLogin(Err(fetch_error)) => {
            log!("Example_A error:", fetch_error);
            orders.skip();
        }
        // ------- Page -------
        Msg::LedgerSummaryMsg(msg) => {
            if let Page::LedgerSummary(model) = &mut model.page {
                page::ledger_summary::update(msg, model, &mut orders.proxy(Msg::LedgerSummaryMsg))
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
        Msg::FinanceManagmentCreateMsg(msg) => {
            if let Page::FinanceManagmentCreate(model) = &mut model.page {
                page::finance_managment_create::update(
                    msg,
                    model,
                    &mut orders.proxy(Msg::FinanceManagmentCreateMsg),
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
            Page::LedgerSummary(model) =>
                page::ledger_summary::view(&model).map_msg(Msg::LedgerSummaryMsg),
            Page::TimeManagmentCreate(model) =>
                page::time_managment_create::view(&model).map_msg(Msg::TimeManagmentCreateMsg),
            Page::FinanceManagmentCreate(model) =>
                page::finance_managment_create::view(&model).map_msg(Msg::FinanceManagmentCreateMsg),
            Page::NotFound => page::not_found::view(),
        }
    ]
}

fn header(base_url: &Url) -> Node<Msg> {
    let general = General::default();
    ul![
        C!["navbar"],
        "Test Navbar",
        &general.body_navbar,
        &general.navbar,
        style! {St::Display => "flex", St::FlexDirection => "row", St::JustifyContent => "center", St::Width => "100%", St::FlexWrap => "wrap"},
        a![
            attrs! { At::Href => Urls::new(base_url).home() },
            "Home",
            &general.navbar_item,
        ],
        a![
            attrs! { At::Href => Urls::new(base_url).calender().default() },
            "Calender",
            &general.navbar_item,
        ],
        a![
            attrs! { At::Href => Urls::new(base_url).music().default() },
            "Music",
            &general.navbar_item,
        ],
        a![
            attrs! { At::Href => Urls::new(base_url).finance().default() },
            "Finance",
            &general.navbar_item,
        ],
        a![
            attrs! { At::Href => Urls::new(base_url).ledger_summary().default() },
            "Time Tracking",
            &general.navbar_item,
        ],
        a![
            attrs! { At::Href => Urls::new(base_url).time_managment_create() },
            "Time Tracking",
            &general.navbar_item,
        ],
        a![
            attrs! { At::Href => Urls::new(base_url).finance_managment_create() },
            "Finance Tracking",
            &general.navbar_item,
        ],
    ]
}

fn view_login(login_data: &auth::UserLogin) -> Node<Msg> {
    let general = General::default();
    div![
        general.body,
        div![
            C!["background"],
            general.background,
            div![C!["shape"], &general.shape, &general.shape_first],
            div![C!["shape"], &general.shape, &general.shape_last],
        ],
        div![
            C!["form"],
            general.form,
            general.form_fix,
            h3!["Need for Seed by Carlos"],
            label!["Username", &general.label],
            input![
                C!["login-name"],
                input_ev(Ev::Input, Msg::SaveLoginUsername),
                attrs! {
                    At::Placeholder => "Name",
                    At::AutoFocus => AtValue::None,
                    At::Value => login_data.username,
                },
                &general.input,
            ],
            label!["Password", &general.label],
            input![
                C!["login-password"],
                input_ev(Ev::Input, Msg::SaveLoginPassword),
                attrs! {
                    At::Placeholder => "Password",
                    At::AutoFocus => AtValue::None,
                    At::Value => login_data.password,
                    At::Type => "Password",
                },
                &general.input,
            ],
            button![
                ev(Ev::Click, |_| Msg::GetLoginRequest),
                &general.button,
                "Get Login message"
            ],
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
