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
    utils::clap_prints,
    window_platform::init_gui,
  },
  clap::Parser,
};

#[derive(Parser, Debug)]
#[command(author, version, disable_help_flag = true, disable_version_flag = true)]
struct Args {
  #[arg(short, long)]
  gui: bool,
  #[arg(short, long)]
  help: bool,
  #[arg(short, long)]
  version: bool,
  #[arg(short, long)]
  config: Option<String>,
}

#[tokio::main]
async fn main() {
  let args = Args::parse();

  if args.help {
    clap_prints::print_help();
    return;
  }

  if args.version {
    clap_prints::print_version();
    return;
  }

  BotLauncher::init(args.config);

  if args.gui {
    init_gui();
  } else {
    BotLauncher::start().await;
  }
}
