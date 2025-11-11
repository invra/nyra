/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    Crate: nyra_gui
    File: lib.rs
    Authors: Invra
    Notes: Iced-rs implementation
*/

use {
  iced::{
    Center,
    widget::{
      Column,
      button,
      column,
      text,
    },
  },
  nyra_core::BotLauncher,
  nyra_utils::log,
  std::sync::{
    Arc,
    atomic::{
      AtomicBool,
      Ordering,
    },
  },
};

pub fn init_gui() -> Result<(), ()> {
  _ = iced::run("Nyra", Nyra::update, Nyra::view);
  Ok(())
}

#[derive(Default)]
struct Nyra {
  is_running: Arc<AtomicBool>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
  ToggleBot,
  BotStopped,
  BotStarted,
}

impl Nyra {
  fn update(&mut self, message: Message) {
    match message {
      Message::ToggleBot => {
        let is_running_clone = self.is_running.clone();
        let currently_running = is_running_clone.load(Ordering::Relaxed);

        let new_state = !currently_running;
        is_running_clone.store(new_state, Ordering::Relaxed);

        std::thread::spawn(move || {
          let fut = async move {
            if new_state {
              log::info!("Starting bot...");
              BotLauncher::start().await;
              log::bot!("Instance has started");
            } else {
              log::info!("Stopping bot...");
              BotLauncher::stop().await;
              log::bot!("Instance has stopped");
            }
          };

          if let Ok(rt) = tokio::runtime::Runtime::new() {
            rt.block_on(fut);
          } else {
            log::error!("Failed to create runtime");
          }
        });
      }

      Message::BotStarted => {
        self.is_running.store(true, Ordering::Relaxed);
        log::info!("Bot has started (GUI event)");
      }

      Message::BotStopped => {
        self.is_running.store(false, Ordering::Relaxed);
        log::info!("Bot has stopped (GUI event)");
      }
    }
  }

  fn view(&self) -> Column<'_, Message> {
    let is_running = self.is_running.load(Ordering::Relaxed);

    column![
      button(if is_running { "Stop Bot" } else { "Start Bot" }).on_press(Message::ToggleBot),
      text(if is_running { "Running" } else { "Not Running" }).size(50),
    ]
    .padding(20)
    .align_x(Center)
  }
}
