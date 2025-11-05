use crate::utils::colorize::{
  Color,
  ColorExt,
};
use std::fmt;

#[allow(dead_code)]
pub enum LogLevel {
  Info,
  Success,
  Warning,
  Error,
  Bot,
  Debug,
}

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

pub(crate) fn log_internal(level: LogLevel, args: fmt::Arguments<'_>) {
  let (stream, color) = match level {
    LogLevel::Error => ("STDERR", level.get_color()),
    _ => ("STDOUT", level.get_color()),
  };

  println!(
    "{} {}",
    format!("[{}/{}]:", stream, level.as_str())
      .color(color)
      .bold(),
    args
  );
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::utils::log::log_internal($crate::utils::log::LogLevel::Info, format_args!($($arg)*));
    };
}
#[allow(unused_imports)]
pub(crate) use info;

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        $crate::utils::log::log_internal($crate::utils::log::LogLevel::Success, format_args!($($arg)*));
    };
}
#[allow(unused_imports)]
pub(crate) use success;

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {
        $crate::utils::log::log_internal($crate::utils::log::LogLevel::Warning, format_args!($($arg)*));
    };
}
#[allow(unused_imports)]
pub(crate) use warning;

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::utils::log::log_internal($crate::utils::log::LogLevel::Error, format_args!($($arg)*));
    };
}
#[allow(unused_imports)]
pub(crate) use error;

#[macro_export]
macro_rules! bot {
    ($($arg:tt)*) => {
        $crate::utils::log::log_internal($crate::utils::log::LogLevel::Bot, format_args!($($arg)*));
    };
}
#[allow(unused_imports)]
pub(crate) use bot;

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::utils::log::log_internal($crate::utils::log::LogLevel::Debug, format_args!($($arg)*));
    };
}
#[allow(unused_imports)]
pub(crate) use debug;
