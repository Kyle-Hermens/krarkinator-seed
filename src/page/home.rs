use crate::{generated::css_classes::C, image_src, Msg, Urls, Model};
use seed::{prelude::*, *};
use std::ops::Not;

#[allow(clippy::too_many_lines)]
pub fn view(base_url: &Url, model: &Model) -> Node<Msg> {
    main![
            C![
                C.wrapper
            ],
            h1![
                C![
                 C.text_center
                ],
                "Krarkinator"
            ],
            div![
            label![
                attrs!{
                    At::For => "krark-count"
                },
                "Krark Count: "
            ],
            input![
                     C![
                    C.flip_count
                ],
               attrs!{
                    At::Type => "number",
                    At::Name => "krark-count",
                    At::Value => model.krark_count,
                },
                 input_ev(Ev::Input, |s| Msg::SetKrarkCount(s.parse().unwrap_or(0)))
            ]
        ],
            div![
                   input![
                attrs!{
                    At::Id => "forever",
                    At::Type => "radio",
                    At::Name => "flip-amount"
                    At::Checked => model.cast_until_termination.as_at_value()
                },
                input_ev(Ev::Input, |_| Msg::CastUntilTermination)
            ],
            label![
                attrs!{
                    At::For => "forever"
                },
                "Until Sequence Terminates"
            ],

            ],
            div![
                   input![
            attrs!{
                   At::Id => "fixed-radio",
                   At::Type => "radio",
                   At::Name => "flip-amount"
                    At::Checked => model.cast_until_termination.not().as_at_value()
                },
                input_ev(Ev::Input, |_| Msg::CastForSetAmount)
            ],
                label![
                attrs!{
                    At::For => "set-amount",
                },
                "For "
            ],
                input![
                C![
                    C.flip_count
                ],
            attrs!{
                   At::Id => "set-amount",
                   At::Type => "number",
                    At::Value => model.flip_count
                },
                input_ev(Ev::Input, |s| Msg::SetFlipCount(s.parse().unwrap_or(0)))
            ],
            label![
                attrs!{
                    At::For => "fixed-radio",
                },
                " Iterations"
            ],

            ],
            button![
                C![
                C.bg_blue_5,
                C.bg_blue_7,
                C.text_gray_1
            ],
                "Flip Coins",
            ev(Ev::Click, |_| Msg::FlipCoins)
            ],
            div![
            "Total Casts: ",
            model.total_casts,
            "Total Coins Flipped: ",
            model.total_coins_flipped,
        ]
        ]
}
