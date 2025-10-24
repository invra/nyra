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
  let args = get_args();
  if utils::clap::handle_common_args(&args) {
    return;
  }

  BotLauncher::init(args.config, args.gui).await;
}
