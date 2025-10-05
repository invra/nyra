/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: HardwareInfo/src/lib.rs
    Authors: Invra
    Notes: Entrypoint file for HardwareInfo
*/

use {
    std::ffi::CString,
    std::os::raw::c_char,
    std::ptr,
    sysinfo::{CpuExt, System, SystemExt},
};

#[unsafe(no_mangle)]
pub extern "C" fn get_cpu_model() -> *mut c_char {
    let mut sys = System::new_all();

    sys.refresh_cpu();

    let cpu_brand = sys.global_cpu_info().brand().to_string();

    match CString::new(cpu_brand) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_cpu_core_count() -> usize {
    let mut sys = System::new_all();

    sys.refresh_cpu();

    sys.cpus().len()
}

#[unsafe(no_mangle)]
pub extern "C" fn free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }

    unsafe {
        let _ = CString::from_raw(s);
    }
}
