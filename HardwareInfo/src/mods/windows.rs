/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: HardwareInfo/src/mods/windows.rs
    Authors: Invra
    Notes: Special windows implementation file
*/

#[cfg(target_os = "windows")]
pub fn get_caption() -> String {
  use winver::WindowsVersion;

  WindowsVersion::detect().unwrap().to_string()
}
