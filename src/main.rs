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

use crate::utils::arg_parser::get_args;

#[tokio::main]
async fn main() {
  if !utils::arg_parser::handle_common_args(&get_args()) {
    bot_launcher::BotLauncher::init(&get_args()).await;
  }
}
