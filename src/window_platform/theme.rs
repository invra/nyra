/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: window_platform/theme.rs
    Authors: Invra
    Notes: Theme system pretty much I guess
*/

#![allow(dead_code)]

use gpui::Rgba;

#[allow(clippy::enum_variant_names)]
pub enum Theme {
  RosePine,
  RosePineMoon,
  RosePineDawn,
}

pub struct Colors {
  pub bg: Rgba,
  pub surface: Rgba,
  pub overlay: Rgba,
  pub text: Rgba,
  pub love: Rgba,
  pub gold: Rgba,
  pub rose: Rgba,
  pub pine: Rgba,
  pub foam: Rgba,
  pub iris: Rgba,
}

impl Colors {
  pub fn from_theme(theme: Theme) -> Self {
    match theme {
      Theme::RosePine => Self {
        bg: rgb(0x191724),
        surface: rgb(0x1f1d2e),
        overlay: rgb(0x26233a),
        text: rgb(0xe0def4),
        love: rgb(0xeb6f92),
        gold: rgb(0xf6c177),
        rose: rgb(0xebbcba),
        pine: rgb(0x31748f),
        foam: rgb(0x9ccfd8),
        iris: rgb(0xc4a7e7),
      },
      Theme::RosePineMoon => Self {
        bg: rgb(0x232136),
        surface: rgb(0x2a273f),
        overlay: rgb(0x393552),
        text: rgb(0xe0def4),
        love: rgb(0xeb6f92),
        gold: rgb(0xf6c177),
        rose: rgb(0xea9a97),
        pine: rgb(0x3e8fb0),
        foam: rgb(0x9ccfd8),
        iris: rgb(0xc4a7e7),
      },
      Theme::RosePineDawn => Self {
        bg: rgb(0xfaf4ed),
        surface: rgb(0xfffaf3),
        overlay: rgb(0xf2e9e1),
        text: rgb(0x575279),
        love: rgb(0xb4637a),
        gold: rgb(0xea9d34),
        rose: rgb(0xd7827e),
        pine: rgb(0x286983),
        foam: rgb(0x56949f),
        iris: rgb(0x907aa9),
      },
    }
  }
}

pub fn rgb(hex: u32) -> Rgba {
  gpui::rgb(hex)
}
