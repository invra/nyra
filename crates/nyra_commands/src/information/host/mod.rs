/*
  SPDX-License-Identifier: Unlicense
  Project: Nyra
  File: commands/information/host/mod.rs
  Authors: Invra, Hiten-Tandon
  Notes: The api calls for the host command
*/

mod host_helper;
mod tests;

mod linux;
mod macos;
mod unknown;
mod windows;

use {
  crate::helper::{
    Context,
    Error,
    MyCommand,
  },
  chrono::{
    DateTime,
    Utc,
  },
  host_helper::{
    get_cpu_count,
    get_cpu_model,
    get_mem,
    get_os_name,
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
};

/// Host information command
#[command(prefix_command, slash_command, category = "Information")]
pub async fn host(ctx: Context<'_>) -> Result<(), Error> {
  let timestamp: DateTime<Utc> = chrono::offset::Utc::now();
  let (used, total) = get_mem();

  let reply = CreateReply::default().embed(
    CreateEmbed::new()
      .title("Host Info")
      .field("CPU Model", get_cpu_model(), false)
      .field("Processors", get_cpu_count().to_string(), false)
      .field("Memory", format!("{used:.2} GB/{total:.2} GB"), false)
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
