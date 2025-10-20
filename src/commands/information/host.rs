/*
  SPDX-License-Identifier: Unlicense
  Project: Nyra
  File: commands/information/ping.rs
  Authors: Invra, Hiten-Tandon
*/

use {
  crate::commands::helper::{
    Context,
    Error,
    MyCommand,
  },
  chrono::{
    DateTime,
    Utc,
  },
  poise::{
    CreateReply,
    command,
    serenity_prelude::{
      Colour,
      CreateEmbed,
      CreateEmbedFooter,
    },
  },
  sysinfo::System,
};

/// Host information command
#[command(prefix_command, slash_command, category = "Information")]
pub async fn host(ctx: Context<'_>) -> Result<(), Error> {
  let timestamp: DateTime<Utc> = chrono::offset::Utc::now();
  let mut sys = System::new_all();
  sys.refresh_all();

  let reply = CreateReply::default().embed(
    CreateEmbed::new()
      .title("Host Info")
      .field("CPU Model", get_cpu_model(&sys), false)
      .field("Processors", get_cpu_count(&sys).to_string(), false)
      .field(
        "Memory",
        format!(
          "{:.2} GB/{:.2} GB",
          get_mem_used_gb(&sys),
          get_mem_heap_gb(&sys)
        ),
        false,
      )
      .field("OS", get_os_name(), false)
      .footer(CreateEmbedFooter::new(format!(
        "Host requested by {}",
        ctx.author().name
      )))
      .timestamp(timestamp)
      .color(Colour::PURPLE),
  );

  ctx.send(reply).await?;

  Ok(())
}
inventory::submit! { MyCommand(host) }

#[cfg(not(target_os = "windows"))]
fn get_cpu_model(sys: &System) -> Box<str> {
  sys.cpus()[0].brand().into()
}

#[cfg(target_os = "windows")]
fn get_cpu_model(_: &System) -> Box<str> {
  use {
    serde::Deserialize,
    wmi::{
      COMLibrary,
      WMIConnection,
    },
  };

  #[allow(non_camel_case_types)]
  #[allow(non_snake_case)]
  #[derive(Deserialize)]
  struct Win32_Proccessor {
    Name: Option<String>,
  }

  let result = (|| -> Result<String, Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;

    let results: Vec<Win32_Proccessor> = wmi_con.raw_query("SELECT Name FROM Win32_Processor")?;

    if let Some(cpu) = results.first() {
      let cpu_model = cpu.Name.as_deref().unwrap_or("Unknown CPU");
      Ok(cpu_model.into())
    } else {
      Ok("Unknown CPU".into())
    }
  })();

  result.unwrap_or_else(|_| "Unknown CPU".into()).into()
}

fn get_cpu_count(sys: &System) -> usize {
  sys.cpus().len()
}

fn get_mem_heap_gb(sys: &System) -> f64 {
  sys.total_memory() as f64 / 1024.0_f64.powi(3)
}

fn get_mem_used_gb(sys: &System) -> f64 {
  sys.used_memory() as f64 / 1024.0_f64.powi(3)
}

#[cfg(target_os = "macos")]
fn get_os_name() -> Box<str> {
  use serde::Deserialize;

  #[derive(Deserialize)]
  #[serde(rename_all = "PascalCase")]
  struct SystemVersion {
    product_version: String,
  }

  let file_buf: SystemVersion =
    plist::from_file("/System/Library/CoreServices/SystemVersion.plist")
      .expect("Cannot read from PLIST!");

  let (major, minor): (u8, u8) = file_buf
    .product_version
    .split_once('.')
    .map(|(x, y)| (x.parse::<u8>().unwrap_or(0), y.parse::<u8>().unwrap_or(0)))
    .unwrap_or((0, 0));

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
fn get_os_name() -> Box<str> {
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

  let os_data = buf
    .lines()
    .filter_map(|x| x.split_once('='))
    .collect::<HashMap<_, _>>();

  let pretty = os_data
    .get("PRETTY_NAME")
    .map(|s| s.trim_matches('"'))
    .unwrap_or("Unknown Linux");

  pretty.into()
}

#[cfg(target_os = "windows")]
fn get_os_name() -> Box<str> {
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
    results.first()?.Caption.clone()
  })()
  .unwrap_or_else(|| "Unknown Windows".to_string());

  normalize_windows_name(&caption).into_boxed_str()
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
fn get_os_name() -> Box<str> {
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
    return "Unknown Windows".into()
  };

  result.push(version_name);

  if version_name.starts_with(|x: char| x.is_ascii_digit()) {
    return result.join(" ")
  }

  let Some(sub_version) = words.next() else {
    return result.join(" ")
  };

  if sub_version.starts_with(|x: char| x.is_ascii_digit()) {
    result.push(sub_version);
  }

  result.join(" ")
}

#[cfg(test)]
mod tests {
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
}
