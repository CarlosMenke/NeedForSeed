use seed::{prelude::*, *};

pub struct General {
    pub button: Style,
    pub button_small: Style,
    pub button_headline: Style,

    pub input: Style,
    pub label: Style,

    pub form: Style,
    pub form_fix: Style,
    pub form_time_entery: Style,

    pub body_navbar: Style,

    pub shape: Style,
    pub shape_first: Style,
    pub shape_last: Style,

    pub navbar_item: Style,
}

impl Default for General {
    fn default() -> General {
        General {
            button: style! {
                St::MarginTop => px(50),
                St::Width => "100%",
                St::BackgroundColor => "#ffffff",
                St::Color => "#080710",
                St::Padding => "15px 0",
                St::FontSize => px(18),
                St::FontWeight => 600,
                St::BorderRadius => px(5),
                St::Cursor => "Pointer",
                St::Border => "0",
            },
            button_small: style! {
                St::MarginTop => px(5),
                St::BackgroundColor => "rgba(0, 0, 0, 0.00)",
                St::Color => "#ffffff",
            },
            button_headline: style! {
                St::MarginTop => px(0),
                St::BackgroundColor => "rgba(0, 0, 0, 0.00)",
                St::Color => "#ffffff",
                St::FontSize => "1.3em",
            },

            input: style! {
                St::Display => "block",
                St::Height => px(50),
                St::Width => "100%",
                St::BackgroundColor => "rgba(255, 255, 255, 0.007)",
                St::Color => "rgba(255, 255, 255, 1.00)",
                St::BorderRadius => px(3),
                St::Padding => "0 10px",
                St::MarginTop => px(8);
                St::FontSize => px(14),
                St::FontWeight => 300,
                St::Border => "0",
            },
            label: style! {
                St::Display => "block",
                St::MarginTop => px(20);
                St::FontSize => px(16),
                St::FontWeight => 500,
            },

            form: style! {
                St::BackgroundColor => "rgba(255, 255, 255, 0.13)",
                St::BorderRadius =>  px(10),
                St::BackdropFilter =>  "blur(10px)",
                St::Border => "2px solid rgba(255,255,255,0.1)",
                St::BorderRadius => px(10),
                St::FontFamily => "'Poppins', sans-serif",
                St::Color => "#ffffff",
                St::LetterSpacing => px(0.5),
                St::Outline => "none",
                St::Border => "none",
                St::FontSize => px(32),
                St::FontWeight => "500",
                St::LineHeight => px(42),
                St::TextAlign => "center",
            },
            form_fix: style! {
                St::Height => px(530),
                St::Width => px(400),
                St::Position => "absolute",
                St::Transform => "translate(-50%,-50%)",
                St::Top => "50%",
                St::Left => "50%",
                St::Padding => "50px 35px",
            },
            form_time_entery: style! {
                St::Top => "20%";
                St::Width => px(460),
                St::Height => px(540),
            },

            body_navbar: style! {
                //St::BackgroundColor => "#080710",
                St::Height => px(300),
                St::Display => "flex",
                St::FlexDirection => "row",
                St::JustifyContent => "space-evenly",
                St::FlexBasis => "120%",
                St::FlexWrap => "wrap"
            },

            shape: style! {
                St::Height => px(200),
                St::Width => px(200),
                St::Position => "absolute",
                St::BorderRadius => "50%",
            },
            shape_first: style! {
                St::Background => "linear-gradient(#1845ad,#23a2f6)",
                St::Top => px(-75),
                St::Left => px(-65),
            },
            shape_last: style! {
                St::Background => "linear-gradient(to right, #ff512f,#f09819)",
                St::Right => px(-55),
                St::Bottom => px(-65),
            },

            navbar_item: style! {
                St::TextDecoration => "none",
                St::TextAlign => "center",
                St::Color => "rgba(255, 255, 255, 0.96)",
                St::Padding => "0px auto 0px auto",
                St::MarginTop => px(8),
                St::FontFamily => "'Poppins', sans-serif",
                St::LetterSpacing => px(0.5),
                St::FontSize => px(16),
                St::FontWeight => "500",
                St::Width => "100%",
            },
        }
    }
}
