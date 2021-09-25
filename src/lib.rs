// @TODO: uncomment once https://github.com/rust-lang/rust/issues/54726 stable
//#![rustfmt::skip::macros(class)]

#![allow(
    clippy::used_underscore_binding,
    clippy::non_ascii_literal,
    clippy::enum_glob_use,
    clippy::must_use_candidate,
    clippy::wildcard_imports
)]

mod generated;
mod page;

use fixed_vec_deque::FixedVecDeque;
use generated::css_classes::C;
use seed::{prelude::*, *};
use Visibility::*;
use crate::page::coin_flip::{Coin, FlipResult};
use rand::rngs::ThreadRng;
use rand::thread_rng;

const TITLE_SUFFIX: &str = "Kavik.cz";
// https://mailtolink.me/
const MAIL_TO_KAVIK: &str = "mailto:martin@kavik.cz?subject=Something%20for%20Martin&body=Hi!%0A%0AI%20am%20Groot.%20I%20like%20trains.";
const MAIL_TO_HELLWEB: &str =
    "mailto:martin@hellweb.app?subject=Hellweb%20-%20pain&body=Hi!%0A%0AI%20hate";
const USER_AGENT_FOR_PRERENDERING: &str = "ReactSnap";
const STATIC_PATH: &str = "static";
const IMAGES_PATH: &str = "static/images";

const ABOUT: &str = "about";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .subscribe(Msg::UrlChanged)
        .stream(streams::window_event(Ev::Scroll, |_| Msg::Scrolled));

    Model {
        base_url: url.to_base_url(),
        page: Page::init(url),
        scroll_history: ScrollHistory::new(),
        menu_visibility: Hidden,
        in_prerendering: is_in_prerendering(),
        flip_count: 2,
        total_casts: 0,
        cast_until_termination: true,
        krark_count: 2,
        total_coins_flipped: 0,
    }
}

fn is_in_prerendering() -> bool {
    let user_agent =
        window().navigator().user_agent().expect("cannot get user agent");

    user_agent == USER_AGENT_FOR_PRERENDERING
}

// ------ ------
//     Model
// ------ ------

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Visibility {
    Visible,
    Hidden,
}

impl Visibility {
    pub fn toggle(&mut self) {
        *self = match self {
            Visible => Hidden,
            Hidden => Visible,
        }
    }
}

// We need at least 3 last values to detect scroll direction,
// because neighboring ones are sometimes equal.
type ScrollHistory = FixedVecDeque<[i32; 3]>;

pub struct Model {
    pub base_url: Url,
    pub page: Page,
    pub scroll_history: ScrollHistory,
    pub menu_visibility: Visibility,
    pub in_prerendering: bool,

    pub flip_count: usize,
    pub total_casts: usize,
    pub cast_until_termination: bool,
    pub krark_count: usize,
    pub total_coins_flipped: usize,
}

// ------ Page ------

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Page {
    Home,
    About,
    NotFound,
}

impl Page {
    pub fn init(mut url: Url) -> Self {
        let (page, title) = match url.remaining_path_parts().as_slice() {
            [] => (Self::Home, TITLE_SUFFIX.to_owned()),
            [ABOUT] => (Self::About, format!("About - {}", TITLE_SUFFIX)),
            _ => (Self::NotFound, format!("404 - {}", TITLE_SUFFIX)),
        };
        document().set_title(&title);
        page
    }
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }

    pub fn about(self) -> Url {
        self.base_url().add_path_part(ABOUT)
    }
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    UrlChanged(subs::UrlChanged),
    ScrollToTop,
    Scrolled,
    ToggleMenu,
    HideMenu,

    FlipCoins,
    SetFlipCount(usize),
    CastUntilTermination,
    CastForSetAmount,
    SetKrarkCount(usize),
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::init(url);
        },
        Msg::ScrollToTop => window().scroll_to_with_scroll_to_options(
            web_sys::ScrollToOptions::new().top(0.),
        ),
        Msg::Scrolled => {
            // Some browsers use `document.body.scrollTop`
            // and other ones `document.documentElement.scrollTop`.
            let mut position = body().scroll_top();
            if position == 0 {
                position = document()
                    .document_element()
                    .expect("get document element")
                    .scroll_top()
            }
            *model.scroll_history.push_back() = position;
        },
        Msg::ToggleMenu => model.menu_visibility.toggle(),
        Msg::HideMenu => {
            model.menu_visibility = Hidden;
        },


        Msg::FlipCoins => {
            let copy_count = get_total_spell_casts(model);

            model.total_casts = copy_count;

        },
        Msg::SetFlipCount(_0) => {
            model.flip_count = _0
        },
        Msg::CastUntilTermination => {
            model.cast_until_termination = true
        },
        Msg::CastForSetAmount => {
            model.cast_until_termination = false
        }
        Msg::SetKrarkCount(_0) => {
           model.krark_count = _0
        }
    }
}

fn get_total_spell_casts(model: &mut Model) -> usize {
    let mut rng = thread_rng();
    let mut copy_count = 0_usize;
    let mut total_coins_flipped = 0_usize;
    if model.cast_until_termination {
        let mut tails_seen = true;
        while tails_seen && copy_count != usize::MAX {
            tails_seen = false;
            for flip in Coin::flip(&mut rng, 0, model.krark_count) {
                if !tails_seen && (flip == FlipResult::Tails || flip == FlipResult::Both) {
                    tails_seen = true;
                } else {
                    copy_count = copy_count.saturating_add(1);
                }
                total_coins_flipped = total_coins_flipped.saturating_add(1);
            }
            if !tails_seen {
                copy_count = copy_count.saturating_add(1);
            }
        }


    } else {
       for _ in 0..model.flip_count {
           let mut tails_seen = false;
           for flip in Coin::flip(&mut rng, 0, model.krark_count) {
               if !tails_seen && (flip == FlipResult::Tails || flip == FlipResult::Both) {
                   tails_seen = true;
               } else {
                   copy_count = copy_count.saturating_add(1);
               }
               total_coins_flipped = total_coins_flipped.saturating_add(1);
           }
           if !tails_seen {
               copy_count = copy_count.saturating_add(1);
           }
       }
    }
    model.total_coins_flipped = total_coins_flipped;
    copy_count
}

// ------ ------
//     View
// ------ ------

// Notes:
// - \u{00A0} is the non-breaking space
//   - https://codepoints.net/U+00A0
//
// - "▶\u{fe0e}" - \u{fe0e} is the variation selector, it prevents ▶ to change to emoji in some browsers
//   - https://codepoints.net/U+FE0E

pub fn view(model: &Model) -> impl IntoNodes<Msg> {
    // div![
    //     C![
    //         IF!(not(model.in_prerendering) => C.fade_in),
    //         C.min_h_screen,
    //         C.flex,
    //         C.flex_col,
    //     ],
        match model.page {
            Page::Home => page::home::view(&model.base_url, &model),
            Page::About => page::about::view(),
            Page::NotFound => page::not_found::view(),
        }
        // page::partial::header::view(model),
        // page::partial::footer::view(),
    // ]
}

pub fn image_src(image: &str) -> String {
    format!("{}/{}", IMAGES_PATH, image)
}

pub fn asset_path(asset: &str) -> String {
    format!("{}/{}", STATIC_PATH, asset)
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn run() {
    log!("Starting app...");

    App::start("app", init, update, view);

    log!("App started.");
}
