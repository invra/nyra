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
  crate::{
    bot_launcher::BotLauncher,
    config::Config,
    window_platform::init_gui,
  },
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

  let config = match &args.config {
    Some(path) => match Config::load_from_path(path) {
      Ok(config) => {
        crate::utils::success(&format!("Config loaded successfully from: {}", path));
        config
      }
      Err(e) => {
        crate::utils::error(&e.to_string());
        return;
      }
    },
    None => match Config::load() {
      Ok(config) => {
        crate::utils::success("Config loaded successfully");
        config
      }
      Err(e) => {
        crate::utils::error(&e.to_string());
        return;
      }
    },
  };

  if args.gui {
    // Start GUI mode which won't start the bot directly
    crate::utils::info("Starting in GUI mode…");
    let bot_launcher = Arc::new(BotLauncher::new(config));
    init_gui(bot_launcher);
  } else {
    // Start bot directly
    crate::utils::info("Starting in CLI mode…");
    let bot_launcher = BotLauncher::new(config);
    bot_launcher.start_bot().await;
  }
}
