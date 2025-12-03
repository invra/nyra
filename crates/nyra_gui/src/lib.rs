/*
 *  SPDX-License-Identifier: Unlicense
 *  Project: Nyra
 *  Crate: nyra_gui
 *  File: lib.rs
 *  Authors: Invra, Hiten-Tandon
 */

use {
  iced::{
    Center,
    Element,
    Length,
    Task,
    keyboard,
    theme::Theme,
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
  tokio::sync::oneshot,
};

pub fn init(rx: oneshot::Receiver<()>) -> iced::Result {
  iced::application("Nyra", Nyra::update, Nyra::view)
    .theme(Nyra::theme)
    .subscription(Nyra::subscription)
    .run_with(|| Nyra::new(rx))
}

#[derive(Default)]
struct Nyra {
  theme: Theme,
  is_running: Arc<AtomicBool>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
  ToggleBot,
  ExitProgram,
}

impl Nyra {
  fn theme(&self) -> Theme {
    self.theme.clone()
  }

  fn subscription(&self) -> iced::Subscription<Message> {
    keyboard::on_key_press(|key, _| {
      matches!(key, keyboard::Key::Named(keyboard::key::Named::F2)).then_some(Message::ToggleBot)
    })
  }

  fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::ExitProgram => iced::exit(),
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
        Task::none()
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

  fn new(rx: oneshot::Receiver<()>) -> (Self, Task<Message>) {
    let instance = Nyra {
      theme: Theme::TokyoNight,
      is_running: Arc::new(AtomicBool::new(false)),
    };

    (instance, Task::perform(rx, |_| Message::ExitProgram))
  }
}
