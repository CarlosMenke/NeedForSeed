use crate::api;
use seed::{prelude::*, *};

// TODO change it from depth to depth for thime (this and last)
const API_TARGET: &str = "timeManagment";
const DEPTH2: &str = "2";
const DEPTH3: &str = "3";
const DEPTHALL: &str = "all";
const DAY: &str = "day";
const WEEK: &str = "week";
const MONTH: &str = "month";
const YEAR: &str = "year";
const ALL: &str = "all";

// ------ ------
//     Init
// ------ ------

pub fn init(
    mut url: Url,
    orders: &mut impl Orders<Msg>,
    ctx: Option<shared::auth::UserLoginResponse>,
) -> Model {
    let base_url = url.to_base_url();
    let depth = match url.next_path_part() {
        Some(DEPTH2) => Depth::Depth2,
        Some(DEPTH3) => Depth::Depth3,
        Some(DEPTHALL) => Depth::DepthAll,
        None => {
            Urls::new(&base_url).default().go_and_replace();
            Depth::Depth3
        }
        _ => Depth::Depth3,
    };
    let timeframe = match url.next_path_part() {
        Some(DAY) => Timeframe::Day,
        Some(WEEK) => Timeframe::Week,
        Some(MONTH) => Timeframe::Month,
        Some(YEAR) => Timeframe::Year,
        Some(ALL) => Timeframe::All,
        None => {
            Urls::new(&base_url).default().go_and_replace();
            Timeframe::Year
        }
        _ => Timeframe::All,
    };
    orders.skip().perform_cmd({
        let token = ctx.clone().unwrap().token;
        let depth_str = depth.clone().str();
        let timeframte_str = timeframe.clone().str();
        async {
            Msg::FetchedSummary(
                api::requests::get_html(token, API_TARGET.to_string(), depth_str, timeframte_str)
                    .await,
            )
        }
    });
    Model {
        base_url,
        ctx,
        depth,
        timeframe,
        finance_summary: None,
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    base_url: Url,
    depth: Depth,
    timeframe: Timeframe,
    ctx: Option<shared::auth::UserLoginResponse>,
    finance_summary: Option<shared::models::ResponseHtml>,
}

// ------ Frequency ------

#[derive(Clone)]
enum Depth {
    Depth2,
    Depth3,
    DepthAll,
}
impl Depth {
    fn str(self) -> String {
        match self {
            Depth::Depth2 => DEPTH2.to_string(),
            Depth::Depth3 => DEPTH3.to_string(),
            Depth::DepthAll => DEPTHALL.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
enum Timeframe {
    Day,
    Week,
    Month,
    Year,
    All,
}
impl Timeframe {
    fn str(self) -> String {
        match self {
            Timeframe::Day => DAY.to_string(),
            Timeframe::Week => WEEK.to_string(),
            Timeframe::Month => MONTH.to_string(),
            Timeframe::Year => YEAR.to_string(),
            Timeframe::All => ALL.to_string(),
        }
    }
}

pub enum Msg {
    GetSummary,
    FetchedSummary(fetch::Result<shared::models::ResponseHtml>),
}
// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn _root(self) -> Url {
        self.base_url()
    }
    pub fn default(self) -> Url {
        self.depth3(Timeframe::All)
    }
    fn depth2(self, time: Timeframe) -> Url {
        self.base_url()
            .add_path_part(DEPTH2)
            .add_path_part(time.str())
    }
    fn depth3(self, time: Timeframe) -> Url {
        self.base_url()
            .add_path_part(DEPTH3)
            .add_path_part(time.str())
    }
    fn depthall(self, time: Timeframe) -> Url {
        self.base_url()
            .add_path_part(DEPTHALL)
            .add_path_part(time.str())
    }
    fn day(self, depth: Depth) -> Url {
        self.base_url()
            .add_path_part(depth.str())
            .add_path_part(DAY)
    }
    fn week(self, depth: Depth) -> Url {
        self.base_url()
            .add_path_part(depth.str())
            .add_path_part(WEEK)
    }
    fn month(self, depth: Depth) -> Url {
        self.base_url()
            .add_path_part(depth.str())
            .add_path_part(MONTH)
    }
    fn year(self, depth: Depth) -> Url {
        self.base_url()
            .add_path_part(depth.str())
            .add_path_part(YEAR)
    }
    fn all(self, depth: Depth) -> Url {
        self.base_url()
            .add_path_part(depth.str())
            .add_path_part(ALL)
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::GetSummary => {
            orders.skip().perform_cmd({
                let token = model.ctx.clone().unwrap().token;
                let depth_str = model.depth.clone().str();
                let timeframe_str = model.timeframe.clone().str();
                async {
                    Msg::FetchedSummary(
                        api::requests::get_html(
                            token,
                            API_TARGET.to_string(),
                            depth_str,
                            timeframe_str,
                        )
                        .await,
                    )
                }
            });
        }
        Msg::FetchedSummary(Ok(response_data)) => {
            model.finance_summary = Some(response_data);
        }
        Msg::FetchedSummary(Err(fetch_error)) => {
            log!("Fetch error:", fetch_error);
            orders.skip();
        }
    }
}
// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    let finance_summary_html = match model.finance_summary.clone() {
        Some(m) => m.html,
        None => "".to_string(),
    };
    let (depth, link) = match &model.depth {
        Depth::DepthAll => (
            DEPTHALL,
            a![
                "Switch to 2",
                attrs! {
                    At::Href => Urls::new(&model.base_url).depth2(model.timeframe.clone())
                }
            ],
        ),
        Depth::Depth2 => (
            DEPTH2,
            a![
                "Switch to 3",
                attrs! {
                    At::Href => Urls::new(&model.base_url).depth3(model.timeframe.clone())
                }
            ],
        ),
        Depth::Depth3 => (
            DEPTH3,
            a![
                "Switch to All",
                attrs! {
                    At::Href => Urls::new(&model.base_url).depthall(model.timeframe.clone())
                }
            ],
        ),
    };
    let (timeframe, link_timeframe) = match &model.timeframe {
        Timeframe::Day => (
            DAY,
            a![
                "Switch to Week",
                attrs! {
                    At::Href => Urls::new(&model.base_url).week(model.depth.clone())
                }
            ],
        ),
        Timeframe::Week => (
            WEEK,
            a![
                "Switch to month",
                attrs! {
                    At::Href => Urls::new(&model.base_url).month(model.depth.clone())
                }
            ],
        ),
        Timeframe::Month => (
            MONTH,
            a![
                "Switch to year",
                attrs! {
                    At::Href => Urls::new(&model.base_url).year(model.depth.clone())
                }
            ],
        ),
        Timeframe::Year => (
            YEAR,
            a![
                "Switch to all",
                attrs! {
                    At::Href => Urls::new(&model.base_url).all(model.depth.clone())
                }
            ],
        ),
        Timeframe::All => (
            ALL,
            a![
                "Switch to Day",
                attrs! {
                    At::Href => Urls::new(&model.base_url).day(model.depth.clone())
                }
            ],
        ),
    };

    div![
        div![format!("Depth:  {}    ", depth), link,],
        div![format!("Timeframe:  {}    ", timeframe), link_timeframe,],
        raw![&finance_summary_html]
    ]
}
