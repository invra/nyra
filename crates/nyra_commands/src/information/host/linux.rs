/*
 *  SPDX-License-Identifier: Unlicense
 *  Project: Nyra
 *  Crate: nyra_commands
 *  File: information/host/linux.rs
 *  Authors: Invra, Hiten-Tandon
 */

#[cfg(target_os = "linux")]
use std::{
  collections::HashMap,
  fs::File,
  io::Read,
};

#[cfg(target_os = "linux")]
pub fn get_cpu_count() -> usize {
  let mut buf = String::new();
  if File::open("/proc/cpuinfo")
    .and_then(|mut f| f.read_to_string(&mut buf))
    .is_err()
  {
    return 0;
  }

  buf
    .lines()
    .filter(|line| line.starts_with("processor"))
    .count()
}

#[cfg(target_os = "linux")]
pub fn get_cpu_model() -> Box<str> {
  let mut buf = String::new();
  if File::open("/proc/cpuinfo")
    .and_then(|mut f| f.read_to_string(&mut buf))
    .is_err()
  {
    return "".into();
  }

  buf
    .lines()
    .filter_map(|x| x.split_once(':'))
    .map(|(a, b)| (a.trim(), b.trim()))
    .collect::<HashMap<_, _>>()
    .get("model name")
    .copied()
    .map(Into::into)
    .unwrap_or_default()
}

#[cfg(target_os = "linux")]
pub fn get_mem() -> (f64, f64) {
  let mut buf = String::new();
  if File::open("/proc/meminfo")
    .and_then(|mut f| f.read_to_string(&mut buf))
    .is_err()
  {
    return (0.0, 0.0);
  }

  let data = buf
    .lines()
    .filter_map(|x| x.split_once(':'))
    .collect::<HashMap<_, _>>();

  let total = data
    .get("MemTotal")
    .map(|s| s.trim_matches(|x: char| !x.is_ascii_digit()).parse::<f64>())
    .and_then(Result::ok)
    .map(|x| x / 2_f64.powi(20))
    .unwrap_or_default();

  let used = data
    .get("MemAvailable")
    .map(|s| s.trim_matches(|x: char| !x.is_ascii_digit()).parse::<f64>())
    .and_then(Result::ok)
    .map(|x| total - x / 2_f64.powi(20))
    .unwrap_or_default();

  (used, total)
}

#[cfg(target_os = "linux")]
pub fn get_os_name() -> Box<str> {
  let mut buf = String::new();
  if File::open("/etc/os-release")
    .and_then(|mut f| f.read_to_string(&mut buf))
    .is_err()
  {
    return "Unknown Linux".into();
  }

  let pretty = buf
    .lines()
    .filter_map(|x| x.split_once('='))
    .collect::<HashMap<_, _>>()
    .get("PRETTY_NAME")
    .map_or("Unknown Linux", |s| s.trim_matches('"'));

  pretty.into()
}
