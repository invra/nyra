/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: main.rs
    Authors: Invra
    Notes: Main entry point for Nyra
*/

mod arg_parser;
use {
  arg_parser::get_args,
  crossterm::{
    event::{
      self,
      Event,
      KeyCode,
      KeyModifiers,
    },
    terminal,
  },
  nyra_utils::log,
  std::time::Duration,
  tokio::task,
};

#[tokio::main]
async fn main() {
  let args = get_args();

  if !arg_parser::handle_common_args(&get_args()) {
    nyra_core::BotLauncher::init_instance(args.config.clone());
    terminal::enable_raw_mode().expect("failed to enable raw mode");

    let quit_task = task::spawn_blocking(|| {
      loop {
        if event::poll(Duration::from_millis(100)).unwrap_or_default()
          && let Ok(Event::Key(key_event)) = event::read()
        {
          match key_event.code {
            KeyCode::Char('q') => {
              terminal::disable_raw_mode().ok();
              log::info!("Gracefully exiting…");
              terminal::disable_raw_mode().ok();
              std::process::exit(0);
            }
            KeyCode::Char('c') => {
              if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                terminal::disable_raw_mode().ok();
                log::info!("Gracefully exiting…");
                terminal::disable_raw_mode().ok();
                std::process::exit(0);
              }
            }
            _ => return,
          }
        }
      }
    });

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

    nyra_core::BotLauncher::start().await;
    let _ = quit_task.await;
    terminal::disable_raw_mode().ok();
  }
}
