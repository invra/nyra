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
    Element,
    Length,
    Subscription,
    keyboard,
    widget::{
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

pub fn init_gui() -> iced::Result {
  iced::application("Nyra", Nyra::update, Nyra::view)
    .subscription(Nyra::subscription)
    .run()
}

#[derive(Default)]
struct Nyra {
  is_running: Arc<AtomicBool>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
  StartBot,
  StopBot,
}

impl Nyra {
  fn subscription(&self) -> Subscription<Message> {
    keyboard::on_key_press(|key, _modifiers| match key {
      keyboard::Key::Named(keyboard::key::Named::F2) => Some(Message::StartBot),
      _ => None,
    })
  }

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

  fn view(&self) -> Element<'_, Message> {
    let is_running = self.is_running.load(Ordering::Relaxed);

    column![
      text("Nyra").size(30),
      button(text(format!(
        "{} (F2)",
        if is_running { "Stop Bot" } else { "Start Bot" }
      )))
      .on_press(if is_running {
        Message::StopBot
      } else {
        Message::StartBot
      }),
    ]
    .width(Length::Fill)
    .padding(20)
    .align_x(Center)
    .into()
  }
}
