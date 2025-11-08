/*
  SPDX-License-Identifier: Unlicense
  Project: Nyra
  File: commands/information/host/tests.rs
  Authors: Invra, Hiten-Tandon
  Notes: Tests for system call name prettier parsers
*/

#[cfg(test)]
mod tests {
  use crate::information::host::{
    macos::get_pretty_macos,
    windows::normalize_windows_name,
  };
  #[test]
  fn windows_unknown() {
    let input = "Microsoft Garry 420";
    assert_eq!(normalize_windows_name(input), "Unknown Windows");
  }

  #[test]
  fn windows_word_and_number() {
    let input = "Microsoft Windows Server 2022 Datacenter";
    assert_eq!(normalize_windows_name(input), "Windows Server 2022");
  }

  #[test]
  fn windows_word() {
    let input = "Microsoft Windows XP Professional SP2";
    assert_eq!(normalize_windows_name(input), "Windows XP");
  }

  #[test]
  fn windows_number() {
    let input = "Microsoft Windows 10 Pro";
    assert_eq!(normalize_windows_name(input), "Windows 10");
  }

  #[test]
  fn macos_before_rehaul() {
    let input = "10.15";
    assert_eq!(get_pretty_macos(input), "macOS Catalina".into());
  }

  #[test]
  fn macos_after_rehaul() {
    let input = "11.0";
    assert_eq!(get_pretty_macos(input), "macOS Big Sur".into());
  }

  #[test]
  fn macos_tahoe_beta() {
    let input = "16.0";
    assert_eq!(get_pretty_macos(input), "macOS Tahoe".into());
  }

  #[test]
  fn macos_tahoe_release() {
    let input = "26.0";
    assert_eq!(get_pretty_macos(input), "macOS Tahoe".into());
  }
}
