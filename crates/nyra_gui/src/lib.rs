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
  ToggleBot,
}

impl Nyra {
  fn subscription(&self) -> Subscription<Message> {
    keyboard::on_key_press(|key, _| {
      matches!(key, keyboard::Key::Named(keyboard::key::Named::F2)).then_some(Message::ToggleBot)
    })
  }

  fn update(&mut self, message: Message) {
    match message {
      Message::ToggleBot => {
        let is_running = self.is_running.clone().load(Ordering::Relaxed);
        self.is_running.store(!is_running, Ordering::Relaxed);

        std::thread::spawn(move || {
          let Ok(rt) = tokio::runtime::Runtime::new() else {
            log::error!("Failed to create runtime");
            return;
          };

          rt.block_on(async move {
            if is_running {
              BotLauncher::stop().await;
              log::bot!("Instance has stopped");
            } else {
              BotLauncher::start().await;
              log::bot!("Instance has started");
            }
          });
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
      .on_press(Message::ToggleBot),
    ]
    .width(Length::Fill)
    .padding(20)
    .align_x(Center)
    .into()
  }
}
