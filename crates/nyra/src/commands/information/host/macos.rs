/*
  SPDX-License-Identifier: Unlicense
  Project: Nyra
  File: commands/information/host/macos.rs
  Authors: Invra, Hiten-Tandon
  Notes: System info calls specific to macos, dependency-free version
*/

#[cfg(target_os = "macos")]
pub fn get_cpu_count() -> usize {
  use std::process::Command;

  Command::new("sysctl")
    .arg("-n")
    .arg("hw.physicalcpu")
    .output()
    .ok()
    .and_then(|out| String::from_utf8(out.stdout).ok())
    .and_then(|s| s.trim().parse().ok())
    .unwrap_or_default()
}

#[cfg(target_os = "macos")]
pub fn get_cpu_model() -> Box<str> {
  use std::process::Command;
  let output = Command::new("sysctl")
    .arg("-n")
    .arg("machdep.cpu.brand_string")
    .output();

  match output {
    Ok(out) if out.status.success() => {
      let s = String::from_utf8_lossy(&out.stdout).trim().to_owned();
      if s.is_empty() {
        "Unknown CPU".into()
      } else {
        s.into_boxed_str()
      }
    }
    _ => "Unknown CPU".into(),
  }
}

#[cfg(target_os = "macos")]
pub fn get_mem() -> (f64, f64) {
  use std::process::Command;

  let total_bytes = Command::new("sysctl")
    .arg("-n")
    .arg("hw.memsize")
    .output()
    .ok()
    .and_then(|out| String::from_utf8(out.stdout).ok())
    .and_then(|s| s.trim().parse::<u64>().ok())
    .unwrap_or_default();

  let mut page_size = 4096u64;
  let mut active_pages = 0u64;
  let mut wired_pages = 0u64;
  let mut compressed_pages = 0u64;

  if let Ok(out) = Command::new("vm_stat").output() {
    let text = String::from_utf8_lossy(&out.stdout);
    for line in text.lines() {
      if line.contains("page size of") {
        if let Some(size_str) = line.split("page size of ").nth(1)
          && let Some(size_str) = size_str.split(' ').next()
        {
          page_size = size_str.parse().unwrap_or(page_size);
        }
      } else if let Some((key, val_str)) = line.split_once(':')
        && let Ok(val) = val_str
          .trim()
          .trim_end_matches('.')
          .replace('.', "")
          .parse::<u64>()
      {
        match key.trim() {
          "Pages active" => active_pages = val,
          "Pages wired down" => wired_pages = val,
          "Pages occupied by compressor" => compressed_pages = val,
          _ => {}
        }
      }
    }
  }

  (
    ((active_pages + wired_pages + compressed_pages) * page_size) as f64 / 1024.0f64.powi(3),
    total_bytes as f64 / 1024.0f64.powi(3),
  )
}

#[cfg(target_os = "macos")]
pub fn get_os_name() -> Box<str> {
  use std::fs;

  let plist =
    fs::read_to_string("/System/Library/CoreServices/SystemVersion.plist").unwrap_or_default();

  if let Some(pos) = plist.find("<key>ProductVersion</key>") {
    let after_key = &plist[pos..];
    if let Some(start) = after_key.find("<string>")
      && let Some(end) = after_key.find("</string>")
    {
      let version = &after_key[start + 8..end];
      return get_pretty_macos(version);
    }
  }

  get_pretty_macos("0.0.0")
}

#[allow(dead_code)]
pub fn get_pretty_macos(ver: &str) -> Box<str> {
  let (major, minor): (u8, u8) = ver.split_once('.').map_or((0, 0), |(x, y)| {
    (x.parse().unwrap_or_default(), y.parse().unwrap_or(0))
  });

  format!(
    "macOS {}",
    match major {
      10 => match minor {
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
  )
  .into()
}
