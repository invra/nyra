/*
  SPDX-License-Identifier: Unlicense
  Project: Nyra
  WindowPlatform/src/lib.rs
  Authors: Invra, HitenTandon
  Notes: GUI Implementation in gpui
*/

use {
  gpui::{
    App, Application, Bounds, Context, SharedString, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
  },
  std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    sync::{Arc, Mutex},
    thread,
  },
};

#[derive(Debug, Default)]
pub struct NyraGui {
  config: Option<String>,
  start_bot: Option<unsafe extern "C" fn(*mut c_char)>,
}

impl NyraGui {
  fn new(config: Option<String>, start_bot: Option<unsafe extern "C" fn(*mut c_char)>) -> Self {
    Self { config, start_bot }
  }

  fn start_bot(&self) {
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

// GPUI Application implementation
struct NyraApp {
  gui: Arc<Mutex<NyraGui>>,
}

impl Application for NyraApp {
  fn draw(&mut self, ctx: &mut Context) {
    let gui = self.gui.lock().unwrap();

    // Root container
    div()
      .size(size(400.0, 200.0))
      .background(rgb(0.1, 0.1, 0.1))
      .child(
        div()
          .size(size(400.0, 200.0))
          .child(gpui::text("Nyra").color(rgb(1.0, 1.0, 1.0)).size(px(24.0)))
          .child(gpui::button("Start Bot").size(size(150.0, 50.0)).on_click({
            let gui = gui.clone();
            move |_| {
              gui.lock().unwrap().start_bot();
            }
          })),
      )
      .render(ctx);
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

  let gui = Arc::new(Mutex::new(NyraGui::new(config_str, start_bot)));

  let mut window = Window::new(
    "Nyra Control Panel",
    WindowBounds::new((800.0, 600.0)),
    WindowOptions::default(),
  );

  let mut app = NyraApp { gui };

  println!("\x1b[1m\x1b[36m[STDOUT/status]:\x1b[0m GUI has started.");

  window.run(&mut app);
}
