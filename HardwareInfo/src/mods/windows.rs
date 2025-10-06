#[cfg(target_os = "windows")]
pub fn get_windows_caption() -> String {
  use winver::WindowsVersion;

  WindowsVersion::detect().unwrap().to_string()
}
