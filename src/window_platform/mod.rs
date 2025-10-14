/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: window_platform/mod.rs
    Authors: Invra
    Notes: WindowPlatform GUI implementation
*/

mod theme;
use {
  crate::bot_launcher::BotLauncher,
  gpui::{
    App,
    KeyBinding,
    WindowBounds,
    WindowOptions,
    actions,
    div,
    point,
    prelude::*,
    px,
    size,
  },
  std::{
    sync::Arc,
    sync::atomic::{
      AtomicBool,
      Ordering,
    },
  },
  theme::{
    Colors,
    Theme,
  },
};

struct NyraView {
  bot_launcher: Arc<BotLauncher>,
  colors: Colors,
  is_running: Arc<AtomicBool>,
}

impl gpui::Render for NyraView {
  fn render(
    &mut self,
    _window: &mut gpui::Window,
    _cx: &mut gpui::Context<Self>,
  ) -> impl IntoElement {
    let is_running = self.is_running.load(Ordering::Relaxed);
    let button_text = if is_running {
      "Bot Running…"
    } else {
      "Start Bot"
    };

    div()
      .flex()
      .flex_col()
      .size_full()
      .bg(self.colors.bg)
      .child(
        div()
          .w_full()
          .window_control_area(gpui::WindowControlArea::Drag)
          .h(px(33.0))
          .border_b_1()
          .border_color(self.colors.surface)
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
            .child(button_text)
            .bg(if is_running {
              self.colors.overlay
            } else {
              self.colors.surface
            })
            .text_color(self.colors.text)
            .p(px(8.))
            .border(px(1.))
            .rounded(px(4.))
            .mt_16()
            .cursor_pointer()
            .hover(|style| {
              if !is_running {
                style.bg(self.colors.overlay)
              } else {
                style
              }
            })
            .on_click({
              let bot_launcher = self.bot_launcher.clone();
              let is_running = self.is_running.clone();
              move |_event, _cx, _| {
                if !is_running.load(Ordering::Relaxed) {
                  let bot_launcher = bot_launcher.clone();
                  let is_running = is_running.clone();
                  std::thread::spawn(move || {
                    is_running.store(true, Ordering::Relaxed);
                    crate::utils::info("Starting bot from GUI…");
                    match tokio::runtime::Runtime::new() {
                      Ok(rt) => {
                        rt.block_on(bot_launcher.start_bot());
                        crate::utils::info("Bot has stopped");
                      }
                      Err(e) => crate::utils::error(&format!("Failed to create runtime: {}", e)),
                    }
                    is_running.store(false, Ordering::Relaxed);
                  });
                }
              }
            }),
        ),
      )
  }
}

actions!(window, [Quit]);
pub fn init_gui(bot_launcher: Arc<BotLauncher>) {
  let theme_colors = Colors::from_theme(Theme::RosePine);
  let is_running = Arc::new(AtomicBool::new(false));

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
          bot_launcher: bot_launcher.clone(),
          colors: theme_colors,
          is_running: is_running.clone(),
        })
      },
    )
    .unwrap();
    cx.on_action(|_: &Quit, cx| cx.quit());
    cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
  });
}
