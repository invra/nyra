/*
  SPDX-License-Identifier: Unlicense
  Project: Nyra
  File: commands/information/host/host_helper.rs
  Authors: Invra, Hiten-Tandon
  Notes: System info calls
*/

#[cfg(target_os = "linux")]
pub use crate::information::host::linux::*;
#[cfg(target_os = "macos")]
pub use crate::information::host::macos::*;
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
pub use crate::information::host::unknown::*;
#[cfg(target_os = "windows")]
pub use crate::information::host::windows::*;
