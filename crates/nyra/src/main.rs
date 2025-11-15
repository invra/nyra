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

fn spawn_quit_task(running: Arc<AtomicBool>) -> tokio::task::JoinHandle<()> {
  let r = Arc::clone(&running);
  task::spawn_blocking(move || {
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
        return;
      }
    }
  })
}

#[cfg(feature = "only-gui")]
#[tokio::main]
async fn main() -> Result<(), ()> {
  let args = get_args();
  if arg_parser::handle_common_args(&args) {
    return Ok(());
  }

  nyra_core::BotLauncher::init_instance(args.config.clone());

  _ = nyra_gui::init_gui();
  Ok(())
}

#[cfg(all(feature = "gui", not(feature = "only-gui")))]
#[tokio::main]
async fn main() -> Result<(), ()> {
  let args = get_args();
  if arg_parser::handle_common_args(&args) {
    return Ok(());
  }

  nyra_core::BotLauncher::init_instance(args.config.clone());

  let _raw_guard = RawModeGuard::new();
  let running = Arc::new(AtomicBool::new(true));
  let quit_task = spawn_quit_task(Arc::clone(&running));

  if args.gui {
    _ = nyra_gui::init_gui();

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
#[tokio::main]
async fn main() -> Result<(), ()> {
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
