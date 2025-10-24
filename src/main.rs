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

use {
  crate::{
    bot_launcher::BotLauncher,
    window_platform::init_gui,
  },
  clap::Parser,
};

#[derive(Parser, Debug)]
#[command(
  author,
  version,
  about = "A Discord bot written in Rust.",
  long_about = "Nyra is a Discord bot which is written with Rust."
)]
struct Args {
  #[arg(short, long)]
  gui: bool,

  #[arg(short, long)]
  config: Option<String>,
}

#[tokio::main]
async fn main() {
  let args = Args::parse();
  BotLauncher::init(args.config);

  if args.gui {
    crate::utils::info("Starting in GUI mode…");
    init_gui();
  } else {
    crate::utils::info("Starting in CLI mode…");
    BotLauncher::start().await;
  }
}
