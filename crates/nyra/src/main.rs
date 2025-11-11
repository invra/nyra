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
  std::{
    sync::{
      Arc,
      atomic::{
        AtomicBool,
        Ordering,
      },
    },
    time::Duration,
  },
  tokio::task,
};

struct RawModeGuard;

impl RawModeGuard {
  fn new() -> Self {
    terminal::enable_raw_mode().expect("failed to enable raw mode");
    Self
  }
}

impl Drop for RawModeGuard {
  fn drop(&mut self) {
    let _ = terminal::disable_raw_mode();
  }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
  let args = get_args();

  if !arg_parser::handle_common_args(&args) {
    nyra_core::BotLauncher::init_instance(args.config.clone());

    let _raw_guard = RawModeGuard::new();
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    let quit_task = task::spawn_blocking(move || {
      while r.load(Ordering::Relaxed) {
        if event::poll(Duration::from_millis(100)).unwrap_or(false)
          && let Ok(Event::Key(key_event)) = event::read()
        {
          let quit = match key_event.code {
            KeyCode::Char('q') => true,
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => true,
            _ => false,
          };
          if quit {
            log::info!("Gracefully exiting…");
            r.store(false, Ordering::Relaxed);
            break;
          }
        }
      }
    });

    #[cfg(feature = "only-gui")]
    {
      _ = nyra_gui::init_gui();
      running.store(false, Ordering::Relaxed);

      quit_task.await.ok();
      return Ok(());
    }

    #[cfg(all(feature = "gui", not(feature = "only-gui")))]
    if args.gui {
      _ = nyra_gui::init_gui();
      running.store(false, Ordering::Relaxed);

      quit_task.await.ok();
      return Ok(());
    }

    tokio::select! {
      _ = nyra_core::BotLauncher::start() => {},
      _ = async {
        while running.load(Ordering::Relaxed) {
          tokio::time::sleep(Duration::from_millis(100)).await;
        }
      } => {}
    }

    quit_task.await.ok();
    log::info!("Clean exit complete");
  }

  Ok(())
}
