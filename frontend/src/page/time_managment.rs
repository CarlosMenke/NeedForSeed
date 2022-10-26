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
const NOW: &str = "0";
const ONE: &str = "1";
const TWO: &str = "2";
const THREE: &str = "3";
const FOUR: &str = "4";
const FIVE: &str = "5";
const SIX: &str = "6";
const SEVEN: &str = "7";

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
    let timepoint = match url.next_path_part() {
        Some(NOW) => Timepoint::Now,
        Some(ONE) => Timepoint::One,
        Some(TWO) => Timepoint::Two,
        Some(THREE) => Timepoint::Three,
        Some(FOUR) => Timepoint::Four,
        Some(FIVE) => Timepoint::Five,
        Some(SIX) => Timepoint::Six,
        Some(SEVEN) => Timepoint::Seven,
        None => {
            Urls::new(&base_url).default().go_and_replace();
            Timepoint::One
        }
        _ => Timepoint::One,
    };
    orders.skip().perform_cmd({
        let token = ctx.clone().unwrap().token;
        let depth_str = depth.clone().str();
        let timeframte_str = timeframe.clone().str();
        let timepoint_str = timepoint.clone().str();
        async {
            Msg::FetchedSummary(
                api::requests::get_html_timepoint(
                    token,
                    API_TARGET.to_string(),
                    depth_str,
                    timeframte_str,
                    timepoint_str,
                )
                .await,
            )
        }
    });
    Model {
        base_url,
        ctx,
        depth,
        timeframe,
        timepoint,
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
    timepoint: Timepoint,
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

#[derive(Debug, Clone)]
enum Timepoint {
    Now,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}
impl Timepoint {
    fn str(self) -> String {
        match self {
            Timepoint::Now => NOW.to_string(),
            Timepoint::One => ONE.to_string(),
            Timepoint::Two => TWO.to_string(),
            Timepoint::Three => THREE.to_string(),
            Timepoint::Four => FOUR.to_string(),
            Timepoint::Five => FIVE.to_string(),
            Timepoint::Six => SIX.to_string(),
            Timepoint::Seven => SEVEN.to_string(),
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
        self.depth3(Timeframe::All, Timepoint::One)
    }
    fn depth2(self, time: Timeframe, point: Timepoint) -> Url {
        self.base_url()
            .add_path_part(DEPTH2)
            .add_path_part(time.str())
            .add_path_part(point.str())
    }
    fn depth3(self, time: Timeframe, point: Timepoint) -> Url {
        self.base_url()
            .add_path_part(DEPTH3)
            .add_path_part(time.str())
            .add_path_part(point.str())
    }
    fn depthall(self, time: Timeframe, point: Timepoint) -> Url {
        self.base_url()
            .add_path_part(DEPTHALL)
            .add_path_part(time.str())
            .add_path_part(point.str())
    }
    fn day(self, depth: Depth, point: Timepoint) -> Url {
        self.base_url()
            .add_path_part(depth.str())
            .add_path_part(DAY)
            .add_path_part(point.str())
    }
    fn week(self, depth: Depth, point: Timepoint) -> Url {
        self.base_url()
            .add_path_part(depth.str())
            .add_path_part(WEEK)
            .add_path_part(point.str())
    }
    fn month(self, depth: Depth, point: Timepoint) -> Url {
        self.base_url()
            .add_path_part(depth.str())
            .add_path_part(MONTH)
            .add_path_part(point.str())
    }
    fn year(self, depth: Depth, point: Timepoint) -> Url {
        self.base_url()
            .add_path_part(depth.str())
            .add_path_part(YEAR)
            .add_path_part(point.str())
    }
    fn all(self, depth: Depth, point: Timepoint) -> Url {
        self.base_url()
            .add_path_part(depth.str())
            .add_path_part(ALL)
            .add_path_part(point.str())
    }
    fn now(self, depth: Depth, timeframe: Timeframe) -> Url {
        self.base_url()
            .add_path_part(depth.str())
            .add_path_part(timeframe.str())
            .add_path_part(NOW)
    }
    fn one(self, depth: Depth, timeframe: Timeframe) -> Url {
        self.base_url()
            .add_path_part(depth.str())
            .add_path_part(timeframe.str())
            .add_path_part(ONE)
    }
    fn two(self, depth: Depth, timeframe: Timeframe) -> Url {
        self.base_url()
            .add_path_part(depth.str())
            .add_path_part(timeframe.str())
            .add_path_part(TWO)
    }
    fn three(self, depth: Depth, timeframe: Timeframe) -> Url {
        self.base_url()
            .add_path_part(depth.str())
            .add_path_part(timeframe.str())
            .add_path_part(THREE)
    }
    fn four(self, depth: Depth, timeframe: Timeframe) -> Url {
        self.base_url()
            .add_path_part(depth.str())
            .add_path_part(timeframe.str())
            .add_path_part(FOUR)
    }
    fn five(self, depth: Depth, timeframe: Timeframe) -> Url {
        self.base_url()
            .add_path_part(depth.str())
            .add_path_part(timeframe.str())
            .add_path_part(FIVE)
    }
    fn six(self, depth: Depth, timeframe: Timeframe) -> Url {
        self.base_url()
            .add_path_part(depth.str())
            .add_path_part(timeframe.str())
            .add_path_part(SIX)
    }
    fn seven(self, depth: Depth, timeframe: Timeframe) -> Url {
        self.base_url()
            .add_path_part(depth.str())
            .add_path_part(timeframe.str())
            .add_path_part(SEVEN)
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::GetSummary => {
            orders.skip().perform_cmd({
                let token = model.ctx.clone().unwrap().token;
                let depth_str = model.depth.clone().str();
                let timeframe_str = model.timeframe.clone().str();
                let timepoint_str = model.timepoint.clone().str();
                async {
                    Msg::FetchedSummary(
                        api::requests::get_html_timepoint(
                            token,
                            API_TARGET.to_string(),
                            depth_str,
                            timeframe_str,
                            timepoint_str,
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
                    At::Href => Urls::new(&model.base_url).depth2(model.timeframe.clone(), model.timepoint.clone())
                }
            ],
        ),
        Depth::Depth2 => (
            DEPTH2,
            a![
                "Switch to 3",
                attrs! {
                    At::Href => Urls::new(&model.base_url).depth3(model.timeframe.clone(), model.timepoint.clone())
                }
            ],
        ),
        Depth::Depth3 => (
            DEPTH3,
            a![
                "Switch to All",
                attrs! {
                    At::Href => Urls::new(&model.base_url).depthall(model.timeframe.clone(), model.timepoint.clone())
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
                    At::Href => Urls::new(&model.base_url).week(model.depth.clone(), model.timepoint.clone())
                }
            ],
        ),
        Timeframe::Week => (
            WEEK,
            a![
                "Switch to month",
                attrs! {
                    At::Href => Urls::new(&model.base_url).month(model.depth.clone(), model.timepoint.clone())
                }
            ],
        ),
        Timeframe::Month => (
            MONTH,
            a![
                "Switch to year",
                attrs! {
                    At::Href => Urls::new(&model.base_url).year(model.depth.clone(), model.timepoint.clone())
                }
            ],
        ),
        Timeframe::Year => (
            YEAR,
            a![
                "Switch to all",
                attrs! {
                    At::Href => Urls::new(&model.base_url).all(model.depth.clone(), model.timepoint.clone())
                }
            ],
        ),
        Timeframe::All => (
            ALL,
            a![
                "Switch to Day",
                attrs! {
                    At::Href => Urls::new(&model.base_url).day(model.depth.clone(), model.timepoint.clone())
                }
            ],
        ),
    };
    let (timepoint, link_timepoint) = match &model.timepoint {
        Timepoint::Now => (
            NOW,
            a![
                "Switch to One",
                attrs! {
                    At::Href => Urls::new(&model.base_url).one(model.depth.clone(), model.timeframe.clone())
                }
            ],
        ),
        Timepoint::One => (
            ONE,
            a![
                "Switch to Two",
                attrs! {
                    At::Href => Urls::new(&model.base_url).two(model.depth.clone(), model.timeframe.clone())
                }
            ],
        ),
        Timepoint::Two => (
            TWO,
            a![
                "Switch to Three",
                attrs! {
                    At::Href => Urls::new(&model.base_url).three(model.depth.clone(), model.timeframe.clone())
                }
            ],
        ),
        Timepoint::Three => (
            THREE,
            a![
                "Switch to Four",
                attrs! {
                    At::Href => Urls::new(&model.base_url).four(model.depth.clone(), model.timeframe.clone())
                }
            ],
        ),
        Timepoint::Four => (
            FOUR,
            a![
                "Switch to Five",
                attrs! {
                    At::Href => Urls::new(&model.base_url).five(model.depth.clone(), model.timeframe.clone())
                }
            ],
        ),
        Timepoint::Five => (
            FIVE,
            a![
                "Switch to Six",
                attrs! {
                    At::Href => Urls::new(&model.base_url).six(model.depth.clone(), model.timeframe.clone())
                }
            ],
        ),
        Timepoint::Six => (
            SIX,
            a![
                "Switch to Seven",
                attrs! {
                    At::Href => Urls::new(&model.base_url).seven(model.depth.clone(), model.timeframe.clone())
                }
            ],
        ),
        Timepoint::Seven => (
            SEVEN,
            a![
                "Switch to Now",
                attrs! {
                    At::Href => Urls::new(&model.base_url).now(model.depth.clone(), model.timeframe.clone())
                }
            ],
        ),
    };

    div![
        div![format!("Depth:  {}    ", depth), link,],
        div![format!("Timeframe:  {}    ", timeframe), link_timeframe,],
        div![format!("Timepoint:  {}    ", timepoint), link_timepoint,],
        raw![&finance_summary_html]
    ]
}
