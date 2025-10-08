/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    WindowPlatform/src/lib.rs
    Authors: Invra
    Notes: GUI Implementation in iced-rs
*/

use iced::widget::{button, column, text};
use iced::{Element, Settings, Task};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::thread;

#[derive(Debug, Clone, Copy)]
pub enum Message {
  Increment,
  Decrement,
  StartBot,
}

#[derive(Debug)]
pub struct Counter {
  val: i64,
  config: Option<String>,
  start_bot: Option<unsafe extern "C" fn(*mut c_char)>,
}

impl Default for Counter {
  fn default() -> Self {
    Self {
      val: 0,
      config: None,
      start_bot: None,
    }
  }
}

impl Counter {
  fn new(config: Option<String>, start_bot: Option<unsafe extern "C" fn(*mut c_char)>) -> Self {
    Self {
      val: 0,
      config,
      start_bot,
    }
  }

  fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::Increment => self.val += 1,
      Message::Decrement => self.val -= 1,
      Message::StartBot => {
        if let Some(start_bot) = self.start_bot {
          let config = self.config.clone();

          thread::spawn(move || unsafe {
            let config_ptr = match config {
              Some(cfg) => CString::new(cfg)
                .map(|c| c.into_raw())
                .unwrap_or(std::ptr::null_mut()),
              None => std::ptr::null_mut(),
            };

            start_bot(config_ptr);

            if !config_ptr.is_null() {
              let _ = CString::from_raw(config_ptr);
            }
          });
        } else {
          eprintln!("Error: start_bot function not provided");
        }
      }
    }
    Task::none()
  }

  fn view(&self) -> Element<'_, Message> {
    column![
      button("+").on_press(Message::Increment),
      text(self.val).size(50),
      button("-").on_press(Message::Decrement),
      button("Start Bot").on_press(Message::StartBot),
    ]
    .into()
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn init_gui(
  config: *const c_char,
  start_bot: Option<unsafe extern "C" fn(*mut c_char)>,
) {
  let config_str = if config.is_null() {
    None
  } else {
    unsafe { CStr::from_ptr(config) }
      .to_str()
      .ok()
      .map(|s| s.to_string())
  };

  let settings = Settings::default();

  if let Err(e) = iced::application("Nyra Control Panel", Counter::update, Counter::view)
    .settings(settings)
    .run_with(move || (Counter::new(config_str, start_bot), Task::none()))
  {
    eprintln!("Failed to run GUI: {}", e);
  }
}
