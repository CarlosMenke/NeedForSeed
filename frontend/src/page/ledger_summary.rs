use crate::api;
use seed::{prelude::*, *};

// TODO change it from depth to depth for thime (this and last)
const DEPTH2: &str = "2";
const DEPTH3: &str = "3";
const DEPTH4: &str = "4";
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
    api_target: String,
) -> Model {
    let base_url = url.to_base_url();
    let depth = match url.next_path_part() {
        Some(DEPTH2) => Depth::Depth2,
        Some(DEPTH3) => Depth::Depth3,
        Some(DEPTH4) => Depth::Depth4,
        Some(DEPTHALL) => Depth::DepthAll,
        None => {
            Urls::new(&base_url).default().go_and_replace();
            Depth::DepthAll
        }
        _ => Depth::DepthAll,
    };
    let timeframe = match url.next_path_part() {
        Some(DAY) => Timeframe::Day,
        Some(WEEK) => Timeframe::Week,
        Some(MONTH) => Timeframe::Month,
        Some(YEAR) => Timeframe::Year,
        Some(ALL) => Timeframe::All,
        None => {
            Urls::new(&base_url).default().go_and_replace();
            Timeframe::Day
        }
        _ => Timeframe::Day,
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
            Timepoint::Now
        }
        _ => Timepoint::Now,
    };
    orders.skip().perform_cmd({
        let token = ctx.clone().unwrap().token;
        let api_target_clone = api_target.clone();
        let depth_str = depth.clone().str();
        let timeframte_str = timeframe.clone().str();
        let timepoint_str = timepoint.clone().str();
        async {
            Msg::FetchedSummary(
                api::requests::get_html(
                    token,
                    api_target_clone,
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
        summary: None,
        api_target,
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
    summary: Option<shared::models::ResponseHtml>,
    api_target: String,
}

// ------ Frequency ------

#[derive(Clone)]
enum Depth {
    Depth2,
    Depth3,
    Depth4,
    DepthAll,
}
impl Depth {
    fn str(self) -> String {
        match self {
            Depth::Depth2 => DEPTH2.to_string(),
            Depth::Depth3 => DEPTH3.to_string(),
            Depth::Depth4 => DEPTH4.to_string(),
            Depth::DepthAll => DEPTHALL.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
        self.depthall(Timeframe::Day, Timepoint::Now)
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
    fn depth4(self, time: Timeframe, point: Timepoint) -> Url {
        self.base_url()
            .add_path_part(DEPTH4)
            .add_path_part(time.str())
            .add_path_part(point.str())
    }
    fn depthall(self, time: Timeframe, point: Timepoint) -> Url {
        self.base_url()
            .add_path_part(DEPTHALL)
            .add_path_part(time.str())
            .add_path_part(point.str())
    }
    fn day(self, depth: Depth, _point: Timepoint) -> Url {
        self.base_url()
            .add_path_part(depth.str())
            .add_path_part(DAY)
            .add_path_part("0".to_string())
    }
    fn week(self, depth: Depth, _point: Timepoint) -> Url {
        self.base_url()
            .add_path_part(depth.str())
            .add_path_part(WEEK)
            .add_path_part("0".to_string())
    }
    fn month(self, depth: Depth, _point: Timepoint) -> Url {
        self.base_url()
            .add_path_part(depth.str())
            .add_path_part(MONTH)
            .add_path_part("0".to_string())
    }
    fn year(self, depth: Depth, _point: Timepoint) -> Url {
        self.base_url()
            .add_path_part(depth.str())
            .add_path_part(YEAR)
            .add_path_part("0".to_string())
    }
    fn all(self, depth: Depth, _point: Timepoint) -> Url {
        self.base_url()
            .add_path_part(depth.str())
            .add_path_part(ALL)
            .add_path_part("1".to_string())
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
                let api_target_clone = model.api_target.clone();
                let depth_str = model.depth.clone().str();
                let timeframe_str = model.timeframe.clone().str();
                let timepoint_str = model.timepoint.clone().str();
                async {
                    Msg::FetchedSummary(
                        api::requests::get_html(
                            token,
                            api_target_clone,
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
            model.summary = Some(response_data);
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
    let summary_html = match model.summary.clone() {
        Some(m) => m.html,
        None => "".to_string(),
    };
    let (depth, link) = match &model.depth {
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
                "Switch to 4",
                attrs! {
                    At::Href => Urls::new(&model.base_url).depth4(model.timeframe.clone(), model.timepoint.clone())
                }
            ],
        ),
        Depth::Depth4 => (
            DEPTH4,
            a![
                "Switch to all",
                attrs! {
                    At::Href => Urls::new(&model.base_url).depthall(model.timeframe.clone(), model.timepoint.clone())
                }
            ],
        ),
        Depth::DepthAll => (
            DEPTHALL,
            a![
                "Switch to 2",
                attrs! {
                    At::Href => Urls::new(&model.base_url).depth2(model.timeframe.clone(), model.timepoint.clone())
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
        style! {
            //St::Height => px(10000);
        },
        div![format!("Depth:  {}    ", depth), link,],
        div![format!("Timeframe:  {}    ", timeframe), link_timeframe,],
        div![format!("Timepoint:  {}    ", timepoint), link_timepoint,],
        div![
            raw![&summary_html],
            style! { St::Margin => "40px 40px 40px 40px"},
        ],
    ]
}
