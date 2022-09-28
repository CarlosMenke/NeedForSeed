use crate::api;
use seed::{prelude::*, *};

const DEPTH1: &str = "1";
const DEPTH2: &str = "2";
const DEPTH3: &str = "3";
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
    let (depth, timeframe) = match url.remaining_path_parts().as_slice() {
        [DEPTH1, WEEK] => (Depth::Depth1, Timeframe::Week),
        [DEPTH2, WEEK] => (Depth::Depth2, Timeframe::Week),
        [DEPTH3, WEEK] => (Depth::Depth3, Timeframe::Week),
        [DEPTH1, MONTH] => (Depth::Depth1, Timeframe::Month),
        [DEPTH2, MONTH] => (Depth::Depth2, Timeframe::Month),
        [DEPTH3, MONTH] => (Depth::Depth3, Timeframe::Month),
        [DEPTH1, YEAR] => (Depth::Depth1, Timeframe::Year),
        [DEPTH2, YEAR] => (Depth::Depth2, Timeframe::Year),
        [DEPTH3, YEAR] => (Depth::Depth3, Timeframe::Year),
        [DEPTH1, ALL] => (Depth::Depth1, Timeframe::All),
        [DEPTH2, ALL] => (Depth::Depth2, Timeframe::All),
        [DEPTH3, ALL] => (Depth::Depth3, Timeframe::All),
        [] => {
            Urls::new(&base_url).default().go_and_replace();
            (Depth::Depth1, Timeframe::Year)
        }
        _ => (Depth::Depth1, Timeframe::All),
    };
    orders.skip().perform_cmd({
        let token = ctx.clone().unwrap().token;
        let depth_str = depth.clone().str();
        let timeframte_str = timeframe.clone().str();
        async {
            Msg::FetchedMusicSummary(
                api::requests::get_music(token, depth_str, timeframte_str).await,
            )
        }
    });
    Model {
        base_url,
        ctx,
        depth,
        timeframe,
        music_summary: None,
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
    music_summary: Option<shared::models::ResponseHtml>,
}

// ------ Frequency ------

#[derive(Clone)]
enum Depth {
    Depth1,
    Depth2,
    Depth3,
}
impl Depth {
    fn str(self) -> String {
        match self {
            Depth::Depth1 => DEPTH1.to_string(),
            Depth::Depth2 => DEPTH2.to_string(),
            Depth::Depth3 => DEPTH3.to_string(),
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
        self.depth1(Timeframe::All)
    }
    fn depth1(self, time: Timeframe) -> Url {
        self.base_url()
            .add_path_part(DEPTH1)
            .add_path_part(time.str())
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
                        api::requests::get_music(token, depth_str, timeframe_str).await,
                    )
                }
            });
        }
        Msg::FetchedMusicSummary(Ok(response_data)) => {
            model.music_summary = Some(response_data);
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
    let music_summary_html = match model.music_summary.clone() {
        Some(m) => m.html,
        None => "".to_string(),
    };
    let (depth, link) = match &model.depth {
        Depth::Depth1 => (
            DEPTH1,
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
                "Switch to depth 1",
                attrs! {
                    At::Href => Urls::new(&model.base_url).depth1(model.timeframe.clone())
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
        div![format!("Hello! This is your {} report.", depth), link,],
        div![
            format!("Hello! This is your {} report.", timeframe),
            link_timeframe,
        ],
        raw![&music_summary_html]
    ]
}
