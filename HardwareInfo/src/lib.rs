/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: HardwareInfo/src/lib.rs
    Authors: Invra
    Notes: Entrypoint file for HardwareInfo
*/

mod macos;

use {
    std::ffi::CString,
    std::os::raw::c_char,
    sysinfo::{CpuExt, System, SystemExt},
    macos::get_version_name
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_cpu_model() -> *mut c_char {
  let mut sys = System::new_all();

  sys.refresh_cpu();

  let cpu_brand = sys.global_cpu_info().brand();
  let cpu_brand_str = cpu_brand.to_string();

  let cpu_brand_str = if cpu_brand_str.is_empty() {
    "Unknown CPU"
  } else {
    &cpu_brand_str
  };

  match CString::new(cpu_brand_str) {
    Ok(c_string) => c_string.into_raw(),
    Err(_) => CString::new("Unknown CPU").unwrap().into_raw(),
  }
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

  let info = os_info::get();

  let os_string = match info.os_type() {
    os_info::Type::Macos => {
      let version = info.version().to_string();
      let mut parts = version.split('.');
      let major = parts.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
      let minor = parts.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);

      format!("macOS {}", get_version_name(major, minor))
    }

    os_info::Type::Windows => {
      format!("Windows {}", info.version())  
    }
    
    other => other.to_string(),
  };

  match CString::new(os_string) {
    Ok(c_string) => c_string.into_raw(),
    Err(_) => CString::new("").unwrap().into_raw(),
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_string(s: *mut c_char) {
  if s.is_null() {
    return;
  }

  unsafe {
    let _ = CString::from_raw(s);
  }
}
