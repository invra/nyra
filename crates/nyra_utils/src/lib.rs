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
    SystemTime,
    UNIX_EPOCH,
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

pub fn runtime_epoch() -> Option<u64> {
  let guard = RUNTIME_INFO.read().unwrap();
  guard.map(|time| {
    time
      .duration_since(UNIX_EPOCH)
      .expect("SystemTime before UNIX_EPOCH")
      .as_secs()
  })
}
