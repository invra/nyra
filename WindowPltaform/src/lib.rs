/*
  SPDX-License-Identifier: Unlicense
  Project: Nyra
  WindowPlatform/src/lib.rs
  Authors: Invra, HitenTandon
  Notes: GUI Implementation in iced-rs
*/

use {
  iced::{
    widget::{
      text,
      button,
      column
    },
    Element, Settings, Task
  },
  std::{
    ffi::{CStr, CString},
    ops::Not as _,
    os::raw::c_char,
    thread
  }
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
  StartBot,
}

#[derive(Debug, Default)]
pub struct NyraGui {
  config: Option<String>,
  start_bot: Option<unsafe extern "C" fn(*mut c_char)>,
}

impl NyraGui {
  fn new(config: Option<String>, start_bot: Option<unsafe extern "C" fn (*mut c_char)>) -> Self {
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
           config
            .map(CString::new)
            .transpose()
            .ok()
            .flatten()
            .map(CString::into_raw)
            .filter(|x| !x.is_null())
            .inspect(|&x| start_bot(x))
            .map(|x| CString::from_raw(x))
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
  let config_str =
    config
      .is_null()
      .not()
      .then(|| unsafe { CStr::from_ptr(config) })
      .map(CStr::to_str)
      .transpose()
      .ok()
      .flatten()
      .map(String::from);

  let settings = Settings::default();
  println!("\x1b[1m\x1b[36m[STDOUT/status]:\x1b[0m {}", "GUI has started.");

  _ = iced::application("Nyra Control Panel", NyraGui::update, NyraGui::view)
    .settings(settings)
    .run_with(move || (NyraGui::new(config_str, start_bot), Task::none()))
    .inspect_err(|e| eprintln!("Failed to run GUI: {e}"));
}
