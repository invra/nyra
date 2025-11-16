/*
 *  SPDX-License-Identifier: Unlicense
 *  Project: Nyra
 *  Crate: Nyra
 *  File: term_utils.rs
 *  Authors: Invra, Hiten-Tandon
 */

use {
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

pub struct RawModeGuard;

impl RawModeGuard {
  pub fn new() -> Self {
    _ = terminal::enable_raw_mode().expect("failed to enable raw mode");
    Self
  }
}

impl Drop for RawModeGuard {
  fn drop(&mut self) {
    _ = terminal::disable_raw_mode();
  }
}

pub fn spawn_quit_task(running: Arc<AtomicBool>) -> tokio::task::JoinHandle<()> {
  task::spawn_blocking(move || {
    while running.load(Ordering::Relaxed) {
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
        running.store(false, Ordering::Relaxed);
        return;
      }
    }
  })
}
