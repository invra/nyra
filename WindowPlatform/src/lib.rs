/*
  SPDX-License-Identifier: Unlicense
  Project: Nyra
  WindowPlatform/src/lib.rs
  Authors: Invra, HitenTandon
  Notes: GUI Implementation in iced-rs
*/

use {
  iced::{
    Element, Settings, Task,
    widget::{button, column, text},
  },
  std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    thread,
  },
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
  StartBot = 0,
}

#[derive(Debug, Default)]
pub struct NyraGui {
  config: Option<String>,
  start_bot: Option<unsafe extern "C" fn(*mut c_char)>,
}

impl NyraGui {
  fn new(config: Option<String>, start_bot: Option<unsafe extern "C" fn(*mut c_char)>) -> Self {
    Self {
      config,
      start_bot,
      ..Default::default()
    }
  }

  fn update(&mut self, message: Message) {
    match message {
      Message::StartBot => {
        let Some(start_bot) = self.start_bot else {
          eprintln!("Error: start_bot function not provided");
          return;
        };
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
      }
    }
  }

  fn view(&self) -> Element<'_, Message> {
    column![
      text("Nyra"),
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
  let config_str = (!config.is_null())
    .then(|| unsafe { CStr::from_ptr(config) })
    .and_then(|c| c.to_str().ok())
    .map(String::from);

  let settings = Settings::default();
  println!("\x1b[1m\x1b[36m[STDOUT/status]:\x1b[0m GUI has started.");

  if let Err(e) = iced::application("Nyra Control Panel", NyraGui::update, NyraGui::view)
    .settings(settings)
    .run_with(move || (NyraGui::new(config_str, start_bot), Task::none()))
  {
    eprintln!("Failed to run GUI: {e}");
  }
}
