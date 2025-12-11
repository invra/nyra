/*
 *  SPDX-License-Identifier: Unlicense
 *  Project: Nyra
 *  Crate: nyra_utils
 *  File: lib.rs
 *  Authors: Invra, Hiten-Tandon
 */

pub mod colorize;
pub mod log;

/// Some other stuff
use std::{
  sync::RwLock,
  time::{
    Duration,
    SystemTime,
  },
};

static RUNTIME_INFO: RwLock<Option<SystemTime>> = RwLock::new(None);

pub fn set_runtime_info() {
  let mut guard = RUNTIME_INFO.write().unwrap();
  *guard = Some(SystemTime::now());
}

pub fn clear_runtime_info() {
  let mut guard = RUNTIME_INFO.write().unwrap();
  *guard = None;
}

pub fn runtime_duration() -> Option<Duration> {
  let guard = RUNTIME_INFO.read().unwrap();
  guard.map(|time| time.elapsed().expect("SystemTime before UNIX_EPOCH"))
}
