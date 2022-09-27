use crate::api;
use seed::{prelude::*, *};

const DEPTH1: &str = "1";
const DEPTH2: &str = "2";
const DEPTH3: &str = "3";

// ------ ------
//     Init
// ------ ------

pub fn init(
    mut url: Url,
    orders: &mut impl Orders<Msg>,
    ctx: Option<shared::auth::UserLoginResponse>,
) -> Model {
    let base_url = url.to_base_url();
    let depth = match url.remaining_path_parts().as_slice() {
        [DEPTH1] => Depth::Depth1,
        [DEPTH2] => Depth::Depth2,
        [DEPTH3] => Depth::Depth3,
        [] => {
            Urls::new(&base_url).default().go_and_replace();
            Depth::Depth1
        }
        _ => Depth::Depth1,
    };
    orders.skip().perform_cmd({
        let token = ctx.clone().unwrap().token;
        let depth_str = depth.clone().str();
        async { Msg::FetchedMusicSummary(api::requests::get_music(token, depth_str).await) }
    });
    Model {
        base_url,
        ctx,
        depth,
        music_summary: None,
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    base_url: Url,
    depth: Depth,
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
        self.depth1()
    }
    pub fn depth1(self) -> Url {
        self.base_url().add_path_part(DEPTH1)
    }
    pub fn depth2(self) -> Url {
        self.base_url().add_path_part(DEPTH2)
    }
    pub fn depth3(self) -> Url {
        self.base_url().add_path_part(DEPTH3)
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::GetMusicSummary => {
            orders.skip().perform_cmd({
                let token = model.ctx.clone().unwrap().token;
                let depth_str = model.depth.clone().str();
                async { Msg::FetchedMusicSummary(api::requests::get_music(token, depth_str).await) }
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
                    At::Href => Urls::new(&model.base_url).depth2()
                }
            ],
        ),
        Depth::Depth2 => (
            DEPTH2,
            a![
                "Switch to depth 3",
                attrs! {
                    At::Href => Urls::new(&model.base_url).depth3()
                }
            ],
        ),
        Depth::Depth3 => (
            DEPTH3,
            a![
                "Switch to depth 1",
                attrs! {
                    At::Href => Urls::new(&model.base_url).depth1()
                }
            ],
        ),
    };

    div![
        "This is the depth: ",
        depth,
        div![format!("Hello! This is your {} report.", depth,), link,],
        raw![&music_summary_html]
    ]
}
