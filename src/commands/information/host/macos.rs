/*
  SPDX-License-Identifier: Unlicense
  Project: Nyra
  File: commands/information/host/macos.rs
  Authors: Invra, Hiten-Tandon
  Notes: System info calls specific to macos
*/

#[cfg(target_os = "macos")]
pub fn get_cpu_model() -> Box<str> {
  use sysctl::Sysctl;
  match sysctl::Ctl::new("machdep.cpu.brand_string") {
    Ok(ctl) => match ctl.value_string() {
      Ok(s) if !s.is_empty() => s.into_boxed_str(),
      _ => "Unknown CPU".into(),
    },
    Err(_) => "Unknown CPU".into(),
  }
}

#[cfg(target_os = "macos")]
#[allow(clippy::cast_precision_loss)]
pub fn get_mem() -> (f64, f64) {
  use {
    libc::{
      _SC_PAGESIZE,
      HOST_VM_INFO64,
      host_statistics64,
      sysconf,
      vm_statistics64_data_t,
    },
    mach2::mach_init::mach_host_self,
    sysctl::Sysctl,
  };

  unsafe {
    let host = mach_host_self();
    let mut stats: vm_statistics64_data_t = std::mem::zeroed();
    let mut count = std::mem::size_of::<vm_statistics64_data_t>() / std::mem::size_of::<i32>();

    if host_statistics64(
      host,
      HOST_VM_INFO64,
      &mut stats as *mut _ as *mut i32,
      &mut count as *mut _ as *mut u32,
    ) != 0
    {
      return (0.0, 0.0);
    }

    let page_size = sysconf(_SC_PAGESIZE) as u64;
    let used_pages = stats.active_count + stats.wire_count + stats.compressor_page_count;
    let used_bytes = used_pages as u64 * page_size;

    let total_bytes = sysctl::Ctl::new("hw.memsize")
      .ok()
      .and_then(|ctl| ctl.value().ok())
      .and_then(|val| match val {
        sysctl::CtlValue::U64(u) => Some(u),
        sysctl::CtlValue::S64(i) => Some(i as u64),
        _ => None,
      })
      .unwrap_or(0);

    (
      used_bytes as f64 / 1024.0f64.powi(3),
      total_bytes as f64 / 1024.0f64.powi(3),
    )
  }
}

#[cfg(target_os = "macos")]
pub fn get_os_name() -> Box<str> {
  use std::fs;

  let plist = fs::read_to_string("/System/Library/CoreServices/SystemVersion.plist")
    .expect("Failed to read SystemVersion.plist");

  if let Some(pos) = plist.find("<key>ProductVersion</key>") {
    let after_key = &plist[pos..];

    if let Some(start) = after_key.find("<string>") {
      if let Some(end) = after_key.find("</string>") {
        let version = &after_key[start + 8..end];
        return get_pretty_macos(version);
      }
    }
  }

  get_pretty_macos("0.0.0") // Should return macOS Unknown 
}

#[allow(dead_code)]
pub fn get_pretty_macos(ver: &str) -> Box<str> {
  let (major, minor): (u8, u8) = ver.split_once('.').map_or((0, 0), |(x, y)| {
    (x.parse().unwrap_or(0), y.parse().unwrap_or(0))
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
