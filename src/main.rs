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
    utils::clap_prints::{
      print_help,
      print_version,
    },
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

  let None = args.help.then(print_help) else {
    return
  };

  let None = args.version.then(print_version) else {
    return
  };

  BotLauncher::init(args.config, args.gui).await;
}
