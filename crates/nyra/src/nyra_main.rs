/*
 *  SPDX-License-Identifier: Unlicense
 *  Project: Nyra
 *  Crate: Nyra
 *  File: nyra_main.rs
 *  Authors: Invra, Hiten-Tandon
 */

#[cfg(any(feature = "gui", feature = "only-gui"))]
use std::sync::atomic::Ordering;
use {
  crate::{
    arg_parser::{
      self,
      get_args,
    },
    term_utils::{
      RawModeGuard,
      spawn_quit_task,
    },
  },
  nyra_utils::log,
  std::sync::{
    Arc,
    atomic::AtomicBool,
  },
};

#[cfg(feature = "only-gui")]
pub(crate) async fn main() -> Result<(), ()> {
  let args = get_args();
  if arg_parser::handle_common_args(&args) {
    return Ok(());
  }

  nyra_core::BotLauncher::init_instance(args.config.clone());

  let _raw_guard = RawModeGuard::new();
  let running = Arc::new(AtomicBool::new(true));
  let quit_task = spawn_quit_task(Arc::clone(&running));

  _ = nyra_gui::init_gui();

  running.store(false, Ordering::Relaxed);
  quit_task.await.ok();
  log::info!("Clean exit complete");
  Ok(())
}

#[cfg(all(feature = "gui", not(feature = "only-gui")))]
pub(crate) async fn main() -> Result<(), ()> {
  let args = get_args();
  if arg_parser::handle_common_args(&args) {
    return Ok(());
  }

  nyra_core::BotLauncher::init_instance(args.config.clone());

  let _raw_guard = RawModeGuard::new();
  let running = Arc::new(AtomicBool::new(true));
  let quit_task = spawn_quit_task(Arc::clone(&running));

  if args.gui {
    let sender = nyra_gui::init_gui().unwrap();
    _ = sender.send(());

    running.store(false, Ordering::Relaxed);
    quit_task.await.ok();
    log::info!("Clean exit complete");
    return Ok(());
  }

  tokio::spawn(nyra_core::BotLauncher::start());
  quit_task.await.ok();
  log::info!("Clean exit complete");
  Ok(())
}

#[cfg(not(any(feature = "gui", feature = "only-gui")))]
pub(crate) async fn main() -> Result<(), ()> {
  let args = get_args();
  if arg_parser::handle_common_args(&args) {
    return Ok(());
  }

  nyra_core::BotLauncher::init_instance(args.config.clone());

  let _raw_guard = RawModeGuard::new();
  let running = Arc::new(AtomicBool::new(true));
  let quit_task = spawn_quit_task(Arc::clone(&running));

  tokio::spawn(nyra_core::BotLauncher::start());
  quit_task.await.ok();
  log::info!("Clean exit complete");
  Ok(())
}
