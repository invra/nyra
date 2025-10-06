/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: HardwareInfo/src/mods/macos.rs
    Authors: Invra
    Notes: Special macos implementation file
*/

pub(crate) fn get_version_name(major: u32, minor: u32) -> &'static str {
  match major {
    10 => match minor {
      0 => "Cheetah",
      1 => "Puma",
      2 => "Jaguar",
      3 => "Panther",
      4 => "Tiger",
      5 => "Leopard",
      6 => "Snow Leopard",
      7 => "Lion",
      8 => "Mountain Lion",
      9 => "Mavericks",
      10 => "Yosemite",
      11 => "El Capitan",
      12 => "Sierra",
      13 => "High Sierra",
      14 => "Mojave",
      15 => "Catalina",
      _ => "Unknown",
    },
    11 => "Big Sur",
    12 => "Monterey",
    13 => "Ventura",
    14 => "Sonoma",
    15 => "Sequoia",
    16 | 26 => "Tahoe",
    _ => "Unknown",
  }
}
