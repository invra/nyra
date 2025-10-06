#[cfg(target_os = "windows")]
pub fn get_caption() -> String {
  use winver::WindowsVersion;

  WindowsVersion::detect().unwrap().to_string()
}
