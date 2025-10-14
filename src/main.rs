/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: main.rs
    Authors: Invra
    Notes: Main entry point for Nyra
*/
mod bot_launcher;
mod config;
mod utils;
mod window_platform;

use {
  crate::{bot_launcher::BotLauncher, config::Config, window_platform::init_gui},
  clap::Parser,
  std::sync::Arc,
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

  let Ok(config) = args
    .config
    .as_ref()
    .map(Config::load_from_path)
    .unwrap_or_else(Config::load)
    .inspect_err(|e| crate::utils::error(&e.to_string()))
  else {
    return
  };

  crate::utils::success("Config loaded successfully");
  let bot_launcher = Arc::new(BotLauncher::new(config));

  if args.gui {
    crate::utils::info("Starting in GUI mode…");
    init_gui(bot_launcher);
  } else {
    crate::utils::info("Starting in CLI mode…");
    bot_launcher.start_bot().await;
  }
}
