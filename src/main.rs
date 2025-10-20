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
    config::Config,
    window_platform::init_gui,
  },
  clap::Parser,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  gui: bool,

  #[arg(short, long)]
  config: Option<String>,
}

#[tokio::main]
async fn main() {
  let args = Args::parse();

  let config = match Config::load(args.config) {
    Ok(cfg) => {
      crate::utils::success("Config loaded successfully");
      cfg
    }
    Err(e) => {
      crate::utils::error(&e.to_string());
      return;
    }
  };

  BotLauncher::init(config);

  if args.gui {
    crate::utils::info("Starting in GUI mode…");
    init_gui();
  } else {
    crate::utils::info("Starting in CLI mode…");
    BotLauncher::start().await;
  }
}
