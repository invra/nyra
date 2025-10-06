#[cfg(target_os = "windows")]
pub(crate) fn get_caption() -> String {
    use windows::Win32::System::Registry::*;
    use windows::core::PCSTR;
    use std::ptr::null_mut;

    unsafe {
        let mut key = HKEY::default();
        let subkey = b"SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\0";
        if RegOpenKeyExA(HKEY_LOCAL_MACHINE, PCSTR(subkey.as_ptr()), 0, KEY_READ, &mut key).is_err() {
            return "Windows (Unknown)".to_string();
        }

        let value_name = b"ProductName\0";
        let mut buf = [0u8; 256];
        let mut buf_size: u32 = buf.len() as u32;

        if RegQueryValueExA(
            key,
            PCSTR(value_name.as_ptr()),
            None,
            None,
            Some(buf.as_mut_ptr()),
            Some(&mut buf_size),
        ).is_err() {
            return "Windows (Unknown)".to_string();
        }

        let s = String::from_utf8_lossy(&buf[..buf_size as usize]).to_string();
        s
    }
}
