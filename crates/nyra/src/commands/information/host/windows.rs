/*
  SPDX-License-Identifier: Unlicense
  Project: Nyra
  File: commands/information/host/windows.rs
  Authors: Invra, Hiten-Tandon
  Notes: System info calls for windows
*/

#[cfg(target_os = "windows")]
pub fn get_cpu_count() -> usize {
  use serde::Deserialize;
  use wmi::{
    COMLibrary,
    WMIConnection,
  };

  #[derive(Deserialize)]
  #[serde(rename_all = "PascalCase")]
  struct Win32Processor {
    number_of_cores: Option<u32>,
  }

  (|| -> Result<usize, Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;
    let results: Vec<Win32Processor> =
      wmi_con.raw_query("SELECT NumberOfCores FROM Win32_Processor")?;

    Ok(
      results
        .into_iter()
        .filter_map(|p| p.number_of_cores)
        .map(|n| n as usize)
        .sum(),
    )
  })()
  .unwrap_or_default()
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
  struct Win32Processor {
    name: Option<String>,
  }

  let result = (|| -> Result<String, Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;
    let results: Vec<Win32Processor> = wmi_con.raw_query("SELECT Name FROM Win32_Processor")?;
    Ok(
      results
        .first()
        .and_then(|c| c.name.clone())
        .unwrap_or("Unknown CPU".into()),
    )
  })();

  result.unwrap_or_else(|_| "Unknown CPU".into()).into()
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
    let total = wmi_con
      .raw_query::<Win32ComputerSystem>("SELECT TotalPhysicalMemory FROM Win32_ComputerSystem")
      .ok()
      .and_then(|v| v.first().and_then(|x| x.total_physical_memory))
      .unwrap_or(0);

    let free = wmi_con
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

#[allow(dead_code)]
pub fn normalize_windows_name(caption: &str) -> String {
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
