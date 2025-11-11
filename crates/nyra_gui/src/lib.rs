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

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum Message {
  StartBot,
  StopBot,
}

impl Nyra {
  fn update(&mut self, message: Message) {
    match message {
      Message::StopBot => {
        let new_state = !self.is_running.clone().load(Ordering::Relaxed);
        self.is_running.clone().store(new_state, Ordering::Relaxed);

        std::thread::spawn(move || {
          let fut = async move {
            BotLauncher::stop().await;
            log::bot!("Instance has stopped");
          };

          if let Ok(rt) = tokio::runtime::Runtime::new() {
            rt.block_on(fut);
          } else {
            log::error!("Failed to create runtime");
          }
        });
      }

      Message::StartBot => {
        let new_state = !self.is_running.clone().load(Ordering::Relaxed);
        self.is_running.clone().store(new_state, Ordering::Relaxed);

        std::thread::spawn(move || {
          let fut = async move {
            BotLauncher::start().await;
            log::bot!("Instance has started");
          };

          if let Ok(rt) = tokio::runtime::Runtime::new() {
            rt.block_on(fut);
          } else {
            log::error!("Failed to create runtime");
          }
        });
      }
    }
  }

  fn view(&self) -> Column<'_, Message> {
    let is_running = self.is_running.load(Ordering::Relaxed);

    column![
      button(if is_running { "Stop Bot" } else { "Start Bot" }).on_press(if is_running {
        Message::StopBot
      } else {
        Message::StartBot
      }),
      text(if is_running { "Running" } else { "Not Running" }).size(50),
    ]
    .padding(20)
    .align_x(Center)
  }
}
