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
#[command(prefix_command, slash_command, category = "information")]
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

fn get_cpu_model(sys: &System) -> &str {
  sys.cpus()[0].brand()
}

fn get_cpu_count(sys: &System) -> usize {
  sys.cpus().len()
}

fn get_mem_heap_gb(sys: &System) -> f64 {
  sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0
}

fn get_mem_used_gb(sys: &System) -> f64 {
  sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0
}

fn get_os_name() -> String {
  #[cfg(target_os = "macos")]
  {
    let info = os_info::get();
    let version = info.version().to_string();
    let mut parts = version.split('.');
    let major = parts
      .next()
      .and_then(|s| s.parse::<u32>().ok())
      .unwrap_or(0);
    let minor = parts
      .next()
      .and_then(|s| s.parse::<u32>().ok())
      .unwrap_or(0);

    let codename = match major {
      10 => match minor {
        0 => "Cheetah",
        1 => "Puma",
        2 => "Jaguar",
        3 => "Panther",
        4 => "Tiger",
        5 => "Leopard",
        6 => "Snow Leopard",
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
    };

    format!("macOS {}", codename)
  }

  #[cfg(target_os = "linux")]
  {
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
      return "Unknown Linux".to_string();
    }

    let os_data = buf
      .lines()
      .filter_map(|x| x.split_once('='))
      .collect::<HashMap<_, _>>();

    let pretty = os_data
      .get("PRETTY_NAME")
      .map(|s| s.trim_matches('"'))
      .unwrap_or("Unknown Linux");

    pretty.to_string()
  }

  #[cfg(target_os = "windows")]
  {
    use winver::WindowsVersion;

    WindowsVersion::detect().unwrap().to_string()
  }
}
