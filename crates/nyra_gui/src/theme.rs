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
  pub fn from_theme(theme: &Theme) -> Self {
    match theme {
      Theme::RosePine => Self {
        bg: rgb(0x19_1724),
        surface: rgb(0x1f_1d2e),
        overlay: rgb(0x26_233a),
        text: rgb(0xe0_def4),
        love: rgb(0xeb_6f92),
        gold: rgb(0xf6_c177),
        rose: rgb(0xeb_bcba),
        pine: rgb(0x31_748f),
        foam: rgb(0x9c_cfd8),
        iris: rgb(0xc4_a7e7),
      },
      Theme::RosePineMoon => Self {
        bg: rgb(0x23_2136),
        surface: rgb(0x2a_273f),
        overlay: rgb(0x39_3552),
        text: rgb(0xe0_def4),
        love: rgb(0xeb_6f92),
        gold: rgb(0xf6_c177),
        rose: rgb(0xea_9a97),
        pine: rgb(0x3e_8fb0),
        foam: rgb(0x9c_cfd8),
        iris: rgb(0xc4_a7e7),
      },
      Theme::RosePineDawn => Self {
        bg: rgb(0xfa_f4ed),
        surface: rgb(0xff_faf3),
        overlay: rgb(0xf2_e9e1),
        text: rgb(0x57_5279),
        love: rgb(0xb4_637a),
        gold: rgb(0xea_9d34),
        rose: rgb(0xd7_827e),
        pine: rgb(0x28_6983),
        foam: rgb(0x56_949f),
        iris: rgb(0x90_7aa9),
      },
    }
  }
}

pub fn rgb(hex: u32) -> Rgba {
  gpui::rgb(hex)
}
