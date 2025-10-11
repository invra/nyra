use {
  gpui::{
    point,
    div,
    prelude::*,
    px,
    size,
    App,
    WindowBounds,
    WindowOptions
  },
  std::{
    ffi::{
      CStr,
      CString
    },
    os::raw::c_char,
    sync::{
      Arc,
      Mutex
    },
    thread
  },
};

#[derive(Default)]
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

  fn start_bot(&self) {
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

struct NyraView {
  gui: Arc<Mutex<NyraGui>>,
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
      .bg(gpui::rgb(0x191724))
      .items_center()
      .text_color(gpui::rgb(0xe0def4))
      .child(div().child("Nyra").text_3xl())
      .child(
        div()
          .id("start-bot")
          .child("Start Bot")
          .bg(gpui::rgb(0x1f1d2e))
          .text_color(gpui::rgb(0xe0def4))
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
pub unsafe extern "C" fn init_gui(
  config: *const c_char,
  start_bot: Option<unsafe extern "C" fn(*mut c_char)>,
) {
  let config_str = (!config.is_null())
    .then(|| unsafe { CStr::from_ptr(config) })
    .and_then(|c| c.to_str().ok())
    .map(String::from);

  let gui = Arc::new(Mutex::new(NyraGui::new(config_str, start_bot)));

  gpui::Application::new().run(move |cx: &mut App| {
    let bounds = WindowBounds::Windowed(gpui::Bounds::centered(None, size(px(400.), px(200.)), cx));
    cx.open_window(
      WindowOptions {
        window_bounds: Some(bounds),
        titlebar: Some(gpui::TitlebarOptions {
          title: None,
          appears_transparent: true,
          traffic_light_position: Some(point(px(9.0), px(9.0))),
        }),
        window_min_size: Some(gpui::Size {
          width: px(360.0),
          height: px(240.0),
        }),
        ..Default::default()
      },
      move |_window, cx| cx.new(move |_| NyraView { gui: gui.clone() }),
    )
    .unwrap();
  });
}
