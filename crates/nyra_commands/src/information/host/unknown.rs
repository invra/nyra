/*
 *  SPDX-License-Identifier: Unlicense
 *  Project: Nyra
 *  Crate: nyra_commands
 *  File: information/host/unknown.rs
 *  Authors: Invra, Hiten-Tandon
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
