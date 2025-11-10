/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: main.rs
    Authors: Invra
    Notes: Main entry point for Nyra
*/

use crossterm::{
  event::{
    self,
    Event,
    KeyCode,
  },
  terminal,
};
use nyra_utils::arg_parser::get_args;
use std::time::Duration;
use tokio::task;

#[tokio::main]
async fn main() {
  let args = get_args();

  if !nyra_utils::arg_parser::handle_common_args(&get_args()) {
    nyra_core::BotLauncher::init_instance(args.config.clone());

    #[cfg(feature = "only-gui")]
    {
      nyra_gui::init_gui();
      return;
    }

    #[cfg(all(feature = "gui", not(feature = "only-gui")))]
    if args.gui {
      nyra_gui::init_gui();
      return;
    }

    terminal::enable_raw_mode().expect("failed to enable raw mode");

    let quit_task = task::spawn_blocking(|| {
      loop {
        if event::poll(Duration::from_millis(100)).unwrap_or(false) {
          if let Ok(Event::Key(key_event)) = event::read() {
            if key_event.code == KeyCode::Char('q') {
              terminal::disable_raw_mode().ok();
              println!("\r\nGracefully exiting...");
              std::process::exit(0);
            }
          }
        }
      }
    });

    nyra_core::BotLauncher::start().await;
    let _ = quit_task.await;
    terminal::disable_raw_mode().ok();
  }
}
