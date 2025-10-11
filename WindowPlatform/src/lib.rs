use gpui::{App, Hsla, WindowBounds, WindowOptions, div, prelude::*, px, size};
use std::{
  os::raw::c_char,
  sync::{Arc, Mutex},
  thread,
};

#[derive(Default)]
pub struct NyraGui {
  start_bot: Option<unsafe extern "C" fn(*mut c_char)>,
}

impl NyraGui {
  fn start_bot(&self) {
    if let Some(start_bot) = self.start_bot {
      thread::spawn(move || unsafe {
        start_bot(std::ptr::null_mut());
      });
    }
  }
}

struct NyraView {
  gui: Arc<Mutex<NyraGui>>,
}

impl gpui::Render for NyraView {
  fn render(
    &mut self,
    _window: &mut gpui::Window,
    _cx: &mut gpui::Context<Self>,
  ) -> impl IntoElement {
    div().child("Nyra").text_color(Hsla::white()).child(
      div()
        .id("start-bot")
        .child("Start Bot")
        .bg(Hsla::blue())
        .text_color(Hsla::white())
        .p(px(8.))
        .border(px(1.))
        .rounded(px(4.))
        .on_click({
          let gui = self.gui.clone();
          move |_event, _cx, _| gui.lock().unwrap().start_bot()
        }),
    )
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_gui(start_bot: Option<unsafe extern "C" fn(*mut c_char)>) {
  let gui = Arc::new(Mutex::new(NyraGui { start_bot }));

  gpui::Application::new().run(move |cx: &mut App| {
    let bounds = WindowBounds::Windowed(gpui::Bounds::centered(None, size(px(400.), px(200.)), cx));
    cx.open_window(
      WindowOptions {
        window_bounds: Some(bounds),
        ..Default::default()
      },
      move |_window, cx| cx.new(move |_| NyraView { gui: gui.clone() }),
    )
    .unwrap();
  });
}
