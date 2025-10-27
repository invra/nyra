#[cfg(target_os = "linux")]
pub fn get_cpu_model() -> Box<str> {
  use std::{
    collections::HashMap,
    fs::File,
    io::Read,
  };

  let mut buf = String::new();
  if File::open("/proc/cpuinfo")
    .and_then(|mut f| f.read_to_string(&mut buf))
    .is_err()
  {
    return "".into();
  }

  buf
    .lines()
    .filter_map(|x| x.split_once(':'))
    .map(|(a, b)| (a.trim(), b.trim()))
    .collect::<HashMap<_, _>>()
    .get("model name")
    .copied()
    .map(Into::into)
    .unwrap_or_default()
}

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

#[cfg(target_os = "windows")]
pub fn get_cpu_model() -> Box<str> {
  use {
    serde::Deserialize,
    wmi::{
      COMLibrary,
      WMIConnection,
    },
  };

  #[derive(Deserialize)]
  #[serde(rename_all = "PascalCase")]
  struct Win32Proccessor {
    name: Option<String>,
  }

  let result = (|| -> Result<String, Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;

    let results: Vec<Win32Proccessor> = wmi_con.raw_query("SELECT Name FROM Win32_Processor")?;

    if let Some(cpu) = results.first() {
      let cpu_model = cpu.name.as_deref().unwrap_or("Unknown CPU");
      Ok(cpu_model.into())
    } else {
      Ok("Unknown CPU".into())
    }
  })();

  result.unwrap_or_else(|_| "Unknown CPU".into()).into()
}

pub fn get_cpu_count() -> usize {
  num_cpus::get()
}

#[cfg(target_os = "linux")]
pub fn get_mem() -> (f64, f64) {
  use std::{
    collections::HashMap,
    fs::File,
    io::Read,
  };

  let mut buf = String::new();
  if File::open("/proc/meminfo")
    .and_then(|mut f| f.read_to_string(&mut buf))
    .is_err()
  {
    return (0.0, 0.0);
  }

  let data = buf
    .lines()
    .filter_map(|x| x.split_once(':'))
    .collect::<HashMap<_, _>>();

  let total = data
    .get("MemTotal")
    .map(|s| s.trim_matches(|x: char| !x.is_ascii_digit()).parse::<f64>())
    .and_then(Result::ok)
    .map(|x| x / 2_f64.powi(20))
    .unwrap_or_default();

  let used = data
    .get("MemAvailable")
    .map(|s| s.trim_matches(|x: char| !x.is_ascii_digit()).parse::<f64>())
    .and_then(Result::ok)
    .map(|x| total - x / 2_f64.powi(20))
    .unwrap_or_default();

  (used, total)
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

    let total_bytes = match sysctl::Ctl::new("hw.memsize") {
      Ok(ctl) => match ctl.value() {
        Ok(val) => match val {
          sysctl::CtlValue::Int(i) => i as u64,
          sysctl::CtlValue::Uint(u) => u as u64,
          sysctl::CtlValue::S64(i) => i as u64,
          sysctl::CtlValue::U64(u) => u,
          _ => 0,
        },
        Err(_) => 0,
      },
      Err(_) => 0,
    };

    (
      used_bytes as f64 / 1024.0f64.powi(3),
      total_bytes as f64 / 1024.0f64.powi(3),
    )
  }
}

#[cfg(target_os = "windows")]
pub fn get_mem() -> (f64, f64) {
  use {
    serde::Deserialize,
    wmi::{
      COMLibrary,
      WMIConnection,
    },
  };

  #[derive(Deserialize)]
  #[serde(rename_all = "PascalCase")]
  struct Win32ComputerSystem {
    total_physical_memory: Option<u64>,
  }

  #[derive(Deserialize)]
  #[serde(rename_all = "PascalCase")]
  struct Win32PerfFormattedDataPerfOSMemory {
    available_bytes: Option<u64>,
  }

  let com_con = COMLibrary::new().ok();
  let wmi_con = com_con.and_then(|c| WMIConnection::new(c.into()).ok());

  if let Some(wmi_con) = wmi_con {
    let total: u64 = wmi_con
      .raw_query::<Win32ComputerSystem>("SELECT TotalPhysicalMemory FROM Win32_ComputerSystem")
      .ok()
      .and_then(|v| v.first().and_then(|x| x.total_physical_memory))
      .unwrap_or(0);

    let free: u64 = wmi_con
      .raw_query::<Win32PerfFormattedDataPerfOSMemory>(
        "SELECT AvailableBytes FROM Win32_PerfFormattedData_PerfOS_Memory",
      )
      .ok()
      .and_then(|v| v.first().and_then(|x| x.available_bytes))
      .unwrap_or(0);

    let used = total.saturating_sub(free);

    return (
      used as f64 / 1024.0_f64.powi(3),
      total as f64 / 1024.0_f64.powi(3),
    );
  }

  (0.0, 0.0)
}

#[cfg(target_os = "macos")]
pub fn get_os_name() -> Box<str> {
  use serde::Deserialize;

  #[derive(Deserialize)]
  #[serde(rename_all = "PascalCase")]
  struct SystemVersion {
    product_version: String,
  }

  let file_buf: SystemVersion =
    plist::from_file("/System/Library/CoreServices/SystemVersion.plist")
      .expect("Cannot read from PLIST!");

  get_pretty_macos(&file_buf.product_version)
}

#[allow(dead_code)]
fn get_pretty_macos(ver: &str) -> Box<str> {
  let (major, minor): (u8, u8) = ver.split_once('.').map_or((0, 0), |(x, y)| {
    (x.parse::<u8>().unwrap_or(0), y.parse::<u8>().unwrap_or(0))
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

#[cfg(target_os = "linux")]
pub fn get_os_name() -> Box<str> {
  use std::{
    collections::HashMap,
    fs::File,
    io::Read,
  };

  let mut buf = String::new();
  if File::open("/etc/os-release")
    .and_then(|mut f| f.read_to_string(&mut buf))
    .is_err()
  {
    return "Unknown Linux".into();
  }

  let pretty = buf
    .lines()
    .filter_map(|x| x.split_once('='))
    .collect::<HashMap<_, _>>()
    .get("PRETTY_NAME")
    .map(|s| s.trim_matches('"'))
    .unwrap_or("Unknown Linux");

  pretty.into()
}

#[cfg(target_os = "windows")]
pub fn get_os_name() -> Box<str> {
  use {
    serde::Deserialize,
    wmi::{
      COMLibrary,
      WMIConnection,
    },
  };

  #[derive(Deserialize)]
  #[serde(rename_all = "PascalCase")]
  struct Win32OperatingSystem {
    caption: Option<String>,
  }

  let caption = (|| -> Option<String> {
    let com_con = COMLibrary::new().ok()?;
    let wmi_con = WMIConnection::new(com_con.into()).ok()?;
    let results: Vec<Win32OperatingSystem> = wmi_con
      .raw_query("SELECT Caption FROM Win32_OperatingSystem")
      .ok()?;
    results.first()?.caption.clone()
  })()
  .unwrap_or_else(|| "Unknown Windows".to_string());

  normalize_windows_name(&caption).into_boxed_str()
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
pub fn get_os_name() -> Box<str> {
  "Unknown OS".into()
}

#[allow(dead_code)]
fn normalize_windows_name(caption: &str) -> String {
  let mut words = caption
    .split_whitespace()
    .skip_while(|&w| w != "Windows")
    .skip(1);

  let mut result = vec!["Windows"];

  let Some(version_name) = words.next() else {
    return "Unknown Windows".into();
  };

  result.push(version_name);

  if version_name.starts_with(|x: char| x.is_ascii_digit()) {
    return result.join(" ");
  }

  let Some(sub_version) = words.next() else {
    return result.join(" ");
  };

  if sub_version.starts_with(|x: char| x.is_ascii_digit()) {
    result.push(sub_version);
  }

  result.join(" ")
}

#[cfg(test)]
mod tests {
  use crate::commands::information::host::host_helper::get_pretty_macos;

  use super::normalize_windows_name;

  #[test]
  fn windows_unknown() {
    let input = "Microsoft Garry 420";
    let output = normalize_windows_name(input);
    assert_eq!(output, "Unknown Windows");
  }

  #[test]
  fn windows_word_and_number() {
    let input = "Microsoft Windows Server 2022 Datacenter";
    let output = normalize_windows_name(input);
    assert_eq!(output, "Windows Server 2022");
  }

  #[test]
  fn windows_word() {
    let input = "Microsoft Windows XP Professional SP2";
    let output = normalize_windows_name(input);
    assert_eq!(output, "Windows XP");
  }

  #[test]
  fn windows_number() {
    let input = "Microsoft Windows 10 Pro";
    let output = normalize_windows_name(input);
    assert_eq!(output, "Windows 10");
  }

  #[test]
  fn macos_before_rehaul() {
    let input = "10.15";
    let output = get_pretty_macos(input);
    assert_eq!(output, "macOS Catalina".into());
  }

  #[test]
  fn macos_after_rehaul() {
    let input = "11.0";
    let output = get_pretty_macos(input);
    assert_eq!(output, "macOS Big Sur".into());
  }

  #[test]
  fn macos_tahoe_beta() {
    let input = "16.0";
    let output = get_pretty_macos(input);
    assert_eq!(output, "macOS Tahoe".into());
  }

  #[test]
  fn macos_tahoe_main() {
    let input = "26.0";
    let output = get_pretty_macos(input);
    assert_eq!(output, "macOS Tahoe".into());
  }
}
