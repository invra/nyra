/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: main.rs
    Authors: Invra
    Notes: Main entry point for Nyra
*/

mod bot_launcher;
mod commands;
mod config;
mod utils;
mod window_platform;

use crate::{
  bot_launcher::BotLauncher,
  utils::clap::get_args,
};

#[tokio::main]
async fn main() {
  if !utils::clap::handle_common_args(&get_args()) {
    BotLauncher::init(&get_args()).await;
  }
}
