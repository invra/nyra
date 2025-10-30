/*
  SPDX-License-Identifier: Unlicense
  Project: Nyra
  File: commands/information/host/unknown.rs
  Authors: Invra, Hiten-Tandon
  Notes: System info calls for default non-standard systems (to return the types term for nothing)
*/

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
pub fn get_cpu_model() -> Box<str> {
  "Unknown CPU".into()
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
pub fn get_mem() -> (f64, f64) {
  (0.0, 0.0)
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
pub fn get_os_name() -> Box<str> {
  "Unknown OS".into()
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
pub fn get_cpu_count() -> usize {
  0x0
}
