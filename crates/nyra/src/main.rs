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
      KeyEvent,
      KeyModifiers,
    },
    terminal,
  },
  nyra_utils::log,
  std::sync::{
    Arc,
    atomic::{
      AtomicBool,
      Ordering,
    },
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
    _ = terminal::disable_raw_mode();
  }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
  let args = get_args();

  if arg_parser::handle_common_args(&args) {
    return Ok(())
  }

  nyra_core::BotLauncher::init_instance(args.config.clone());

  let _raw_guard = RawModeGuard::new();
  let running = Arc::new(AtomicBool::new(true));
  let r = Arc::clone(&running);

  let quit_task = task::spawn_blocking(move || {
    while r.load(Ordering::Relaxed) {
      if matches!(
        event::read(),
        Ok(Event::Key(
          KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
            ..
          } | KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
          }
        ))
      ) {
        log::info!("Gracefully exiting…");
        r.store(false, Ordering::Relaxed);
        return
      }
    }
  });

  #[cfg(feature = "only-gui")]
  {
    nyra_gui::init_gui();
    quit_task.await;
    return Ok(());
  }

  #[cfg(all(feature = "gui", not(feature = "only-gui")))]
  if args.gui {
    _ = nyra_gui::init_gui();
    quit_task.await.ok();
    return Ok(());
  }

  #[warn(unreachable_code)]
  if !args.gui {
    tokio::spawn(nyra_core::BotLauncher::start());
    quit_task.await.ok();
    log::info!("Clean exit complete");
  }

  Ok(())
}
