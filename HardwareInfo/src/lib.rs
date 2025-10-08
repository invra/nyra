/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: HardwareInfo/src/lib.rs
    Authors: Invra
    Notes: Entrypoint file for HardwareInfo
*/

mod mods;

use {
  std::ffi::CString,
  std::os::raw::c_char,
  sysinfo::{CpuExt, System, SystemExt},
};

#[cfg(target_os = "windows")]
use mods::windows::get_caption;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_cpu_model() -> *mut c_char {
  let mut sys = System::new_all();

  sys.refresh_cpu();

  let cpu_brand = sys.global_cpu_info().brand();
  let cpu_brand_str = cpu_brand.to_string();

  let cpu_brand_str = if cpu_brand_str.is_empty() {
    "Unknown"
  } else {
    &cpu_brand_str
  };

  CString::new(cpu_brand_str)
    .unwrap_or_else(|_| CString::new("Unknown").unwrap())
    .into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_cpu_core_count() -> usize {
  let mut sys = System::new_all();

  sys.refresh_cpu();

  sys.cpus().len()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_mem_heap_usize() -> usize {
  let mut sys = System::new_all();

  sys.refresh_memory();

  sys.total_memory() as usize
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_mem_used_usize() -> usize {
  let mut sys = System::new_all();

  sys.refresh_memory();

  sys.used_memory() as usize
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_host_os_string() -> *mut c_char {
  use std::ffi::CString;

  #[cfg(target_os = "macos")]
  {
    let info = os_info::get();
    use crate::mods::macos::get_version_name;
    let version = info.version().to_string();
    let mut parts = version.split('.');
    let major = parts
      .next()
      .and_then(|s| s.parse::<u32>().ok())
      .unwrap_or(0);
    let minor = parts
      .next()
      .and_then(|s| s.parse::<u32>().ok())
      .unwrap_or(0);

    CString::new(format!("macOS {}", get_version_name(major, minor)).into_bytes())
      .unwrap_or_default()
      .into_raw()
  }

  #[cfg(target_os = "windows")]
  {
    CString::new(format!("Windows {}", get_caption()).into_bytes())
      .unwrap_or_default()
      .into_raw()
  }

  #[cfg(target_os = "linux")]
  {
    CString::new(
      crate::mods::linux::get_distro_and_version()
        .unwrap_or("Unknown".into())
        .as_bytes()
        .to_vec(),
    )
    .unwrap_or_default()
    .into_raw()
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_string(s: *mut c_char) {
  if s.is_null() { return };

  unsafe {
    let _ = CString::from_raw(s);
  }
}
