use crate::api;
use seed::{prelude::*, *};

const API_TARGET: &str = "timeManagment";
const DEPTH2: &str = "2";
const DEPTH3: &str = "3";
const DEPTHALL: &str = "all";
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
            Depth::DepthAll
        }
        _ => Depth::DepthAll,
    };
    let timeframe = match url.next_path_part() {
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
            Msg::FetchedMusicSummary(
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
    Week,
    Month,
    Year,
    All,
}
impl Timeframe {
    fn str(self) -> String {
        match self {
            Timeframe::Week => WEEK.to_string(),
            Timeframe::Month => MONTH.to_string(),
            Timeframe::Year => YEAR.to_string(),
            Timeframe::All => ALL.to_string(),
        }
    }
}

pub enum Msg {
    GetMusicSummary,
    FetchedMusicSummary(fetch::Result<shared::models::ResponseHtml>),
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
        self.depthall(Timeframe::All)
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
        Msg::GetMusicSummary => {
            orders.skip().perform_cmd({
                let token = model.ctx.clone().unwrap().token;
                let depth_str = model.depth.clone().str();
                let timeframe_str = model.timeframe.clone().str();
                async {
                    Msg::FetchedMusicSummary(
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
        Msg::FetchedMusicSummary(Ok(response_data)) => {
            model.finance_summary = Some(response_data);
        }
        Msg::FetchedMusicSummary(Err(fetch_error)) => {
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
                "Switch to depth 2",
                attrs! {
                    At::Href => Urls::new(&model.base_url).depth2(model.timeframe.clone())
                }
            ],
        ),
        Depth::Depth2 => (
            DEPTH2,
            a![
                "Switch to depth 3",
                attrs! {
                    At::Href => Urls::new(&model.base_url).depth3(model.timeframe.clone())
                }
            ],
        ),
        Depth::Depth3 => (
            DEPTH3,
            a![
                "Switch to depth All",
                attrs! {
                    At::Href => Urls::new(&model.base_url).depthall(model.timeframe.clone())
                }
            ],
        ),
    };
    let (timeframe, link_timeframe) = match &model.timeframe {
        Timeframe::Week => (
            WEEK,
            a![
                "Switch to timeframe month",
                attrs! {
                    At::Href => Urls::new(&model.base_url).month(model.depth.clone())
                }
            ],
        ),
        Timeframe::Month => (
            MONTH,
            a![
                "Switch to timeframe year",
                attrs! {
                    At::Href => Urls::new(&model.base_url).year(model.depth.clone())
                }
            ],
        ),
        Timeframe::Year => (
            YEAR,
            a![
                "Switch to timeframe all",
                attrs! {
                    At::Href => Urls::new(&model.base_url).all(model.depth.clone())
                }
            ],
        ),
        Timeframe::All => (
            ALL,
            a![
                "Switch to timeframe week",
                attrs! {
                    At::Href => Urls::new(&model.base_url).week(model.depth.clone())
                }
            ],
        ),
    };

    div![
        "This is the depth: ",
        depth,
        div![format!("This is your {} report.", depth), link,],
        div![
            format!("This is your {} report.", timeframe),
            link_timeframe,
        ],
        raw![&finance_summary_html]
    ]
}
