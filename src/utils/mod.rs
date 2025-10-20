/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: utils/mod.rs
    Authors: Invra
    Notes: Utility functions for Nyra
*/

pub mod colorize;
use colorize::{
  Color,
  ColorExt,
};

pub enum LogLevel {
  Info,
  Success,
  Warning,
  Error,
  Bot,
  Debug,
}

#[allow(dead_code)]
impl LogLevel {
  const fn as_str(&self) -> &'static str {
    match self {
      Self::Info => "inf",
      Self::Success => "suc",
      Self::Warning => "wrn",
      Self::Error => "err",
      Self::Bot => "bot",
      Self::Debug => "dbg",
    }
  }

  const fn get_color(&self) -> Color {
    match self {
      Self::Info => Color::Cyan,
      Self::Success => Color::Green,
      Self::Warning => Color::Yellow,
      Self::Error => Color::Red,
      Self::Bot => Color::Magenta,
      Self::Debug => Color::Blue,
    }
  }
}

pub fn log(level: &LogLevel, message: &str) {
  let (stream, color) = match level {
    LogLevel::Error => ("STDERR", level.get_color()),
    _ => ("STDOUT", level.get_color()),
  };

  println!(
    "{} {}",
    format!("[{}/{}]:", stream, level.as_str())
      .color(color)
      .bold(),
    message
  );
}

#[allow(dead_code)]
pub fn info(message: &str) {
  log(&LogLevel::Info, message);
}

#[allow(dead_code)]
pub fn success(message: &str) {
  log(&LogLevel::Success, message);
}

#[allow(dead_code)]
pub fn warning(message: &str) {
  log(&LogLevel::Warning, message);
}

#[allow(dead_code)]
pub fn error(message: &str) {
  log(&LogLevel::Error, message);
}

#[allow(dead_code)]
pub fn bot(message: &str) {
  log(&LogLevel::Bot, message);
}

#[allow(dead_code)]
pub fn debug(message: &str) {
  log(&LogLevel::Debug, message);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_log_levels() {
    info("This is an info message");
    success("This is a success message");
    warning("This is a warning message");
    error("This is an error message");
    bot("This is a bot message");
    debug("This is a debug message");
  }
}
