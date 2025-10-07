/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: HardwareInfo/src/mods/linux.rs
    Authors: Invra
    Notes: Special Linux implementation file
*/

#[cfg(target_os = "linux")]
pub fn get_distro_and_version() -> Result<Box<str>, Box<str>> {
  use std::{collections::HashMap, fs::OpenOptions, io::Read};
  let mut buf = String::new();

  _ = OpenOptions::new()
    .read(true)
    .write(false)
    .open("/etc/os-release")
    .map_err(|_| "Unknown")?
    .read_to_string(&mut buf)
    .map_err(|_| "Unknown")?;

  let os_data = buf
    .lines()
    .filter_map(|x| x.split_once("="))
    .collect::<HashMap<_, _>>();

  Ok(
    os_data
      .get("PRETTY_NAME")
      .copied()
      .map(Box::<str>::from)
      .unwrap_or_else(|| {
        format!(
          "{} {} ({})",
          os_data[&"NAME"],
          os_data[&"VERSION"],
          os_data.get("VERSION_CODENAME").copied().unwrap_or("")
        )
        .replace("()", "")
        .into()
      })
      .replace("\"", "")
      .into(),
  )
}
