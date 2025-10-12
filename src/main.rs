/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: main.rs
    Authors: Invra
    Notes: Main entry point for Nyra
*/

mod bot_launcher;
mod utils;
mod window_platform;

use {
  crate::{
    bot_launcher::{
      BotLauncher,
      Config,
    },
    window_platform::init_gui,
  },
  clap::Parser,
  std::{
    fs,
    sync::Arc,
  },
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Start in GUI mode
  #[arg(short, long)]
  gui: bool,
}

#[tokio::main]
async fn main() {
  let args = Args::parse();
  let config_location = "/Users/invra/.config/nyra/nyra.toml";
  let fs_str = match fs::read_to_string(config_location) {
    Ok(str) => str,
    Err(e) => {
      crate::utils::error(&format!("Failed to read config file: {}", e));
      return;
    }
  };
  let config: Config = match toml::from_str(&fs_str) {
    Ok(config) => config,
    Err(e) => {
      crate::utils::error(&format!("Failed to parse config file: {}", e));
      return;
    }
  };
  crate::utils::success("Config loaded successfully");

  if args.gui {
    // Start GUI mode
    crate::utils::info("Starting in GUI mode...");
    let bot_launcher = Arc::new(BotLauncher::new(config));
    init_gui(bot_launcher);
  } else {
    // Start bot directly
    crate::utils::info("Starting in CLI mode...");
    let bot_launcher = BotLauncher::new(config);
    bot_launcher.start_bot().await;
  }
}
