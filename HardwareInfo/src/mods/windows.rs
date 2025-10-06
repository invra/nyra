use winver::WindowsVersion;

pub fn get_windows_caption() -> Option<String> {
  WindowsVersion::detect().unwrap()
}
