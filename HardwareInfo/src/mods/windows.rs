#[cfg(target_os = "windows")]
pub(crate) fn get_caption() -> String {
    use windows::Win32::System::Registry::{
        RegOpenKeyExA, RegQueryValueExA, HKEY_LOCAL_MACHINE, KEY_READ,
    };

    use std::ptr::null_mut;

    unsafe {
        let mut key = Default::default();
        if RegOpenKeyExA(
            HKEY_LOCAL_MACHINE,
            "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\0",
            0,
            KEY_READ,
            &mut key,
        ).is_ok() {
            let mut buf = [0u8; 256];
            let mut buf_size = buf.len() as u32;
            if RegQueryValueExA(
                key,
                "ProductName\0",
                None,
                None,
                buf.as_mut_ptr(),
                &mut buf_size,
            ).is_ok() {
                return String::from_utf8_lossy(&buf[..buf_size as usize])
                    .trim_end_matches('\0')
                    .to_string();
            }
        }
    }

    "Unknown Windows".to_string()
}
