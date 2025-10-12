/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: WindowPlatform/src/lib.rs
    Authors: Invra
    Notes: Entrypoint file for NyraGui
*/

mod theme;
use {
  gpui::{App, KeyBinding, WindowBounds, WindowOptions, actions, div, point, prelude::*, px, size},
  std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    sync::{Arc, Mutex},
    thread,
  },
  theme::{Colors, Theme},
};

#[derive(Default)]
pub struct NyraGui {
  config: String,
  start_bot: Option<unsafe extern "C" fn(*mut c_char)>,
}

impl NyraGui {
  fn new(config: String, start_bot: Option<unsafe extern "C" fn(*mut c_char)>) -> Self {
    Self {
      config,
      start_bot,
      ..Default::default()
    }
  }

  fn start_bot(&self) {
    let Some(start_bot) = self.start_bot else {
      eprintln!("Error: start_bot function not provided");
      return;
    };

    let config = self.config.clone();

    thread::spawn(move || unsafe {
      _ = CString::new(config)
        .map(CString::into_raw)
        .ok()
        .map(|x| {
          if x.is_null() {
            CString::default().into_raw()
          } else {
            x
          }
        })
        .inspect(|&x| start_bot(x))
        .map(|x| CString::from_raw(x))
    });
  }
}

struct NyraView {
  gui: Arc<Mutex<NyraGui>>,
  colors: Colors,
}

impl gpui::Render for NyraView {
  fn render(
    &mut self,
    _window: &mut gpui::Window,
    _cx: &mut gpui::Context<Self>,
  ) -> impl IntoElement {
    div()
      .flex()
      .flex_col()
      .size_full()
      .bg(self.colors.bg)
      .child(
        div()
          .w_full()
          .window_control_area(gpui::WindowControlArea::Drag)
          .h(px(35.0))
          .border_b_1()
          .border_color(self.colors.overlay)
          .flex()
          .items_center()
          .justify_center()
          .text_color(self.colors.text)
          .child(div().child("Nyra").font_weight(gpui::FontWeight::SEMIBOLD)),
      )
      .child(
        div().flex().flex_col().items_center().child(
          div()
            .id("start-bot")
            .child("Start Bot")
            .bg(self.colors.surface)
            .text_color(self.colors.text)
            .p(px(8.))
            .border(px(1.))
            .rounded(px(4.))
            .mt_16()
            .cursor_pointer()
            .hover(|style| style.bg(self.colors.overlay))
            .on_click({
              let gui = self.gui.clone();
              move |_event, _cx, _| gui.lock().unwrap().start_bot()
            }),
        ),
      )
  }
}

actions!(window, [Quit]);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_gui(
  config: *const c_char,
  start_bot: Option<unsafe extern "C" fn(*mut c_char)>,
) {
  let gui = Arc::new(Mutex::new(NyraGui::new(
    (!config.is_null())
      .then(|| unsafe { CStr::from_ptr(config) })
      .map(CStr::to_str)
      .and_then(Result::ok)
      .map(String::from)
      .unwrap_or_default(),
    start_bot,
  )));
  let theme_colors = Colors::from_theme(Theme::RosePine);

  gpui::Application::new().run(move |cx: &mut App| {
    let bounds = WindowBounds::Windowed(gpui::Bounds::centered(None, size(px(400.), px(200.)), cx));

    cx.open_window(
      WindowOptions {
        window_bounds: Some(bounds),
        titlebar: Some(gpui::TitlebarOptions {
          title: Some("Nyra".into()),
          appears_transparent: true,
          traffic_light_position: Some(point(px(12.0), px(9.0))),
        }),
        window_min_size: Some(gpui::Size {
          width: px(360.0),
          height: px(240.0),
        }),
        ..Default::default()
      },
      move |_window, cx| {
        cx.new(move |_| NyraView {
          gui: gui.clone(),
          colors: theme_colors,
        })
      },
    )
    .unwrap();
    cx.on_action(|_: &Quit, cx| cx.quit());
    cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
  });
}
