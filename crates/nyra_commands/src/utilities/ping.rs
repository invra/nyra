/*
  SPDX-License-Identifier: Unlicense
  Project: Nyra
  File: commands/information/ping.rs
  Authors: Invra, Hiten-Tandon
*/

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

/// Ping command
#[command(prefix_command, slash_command, category = "Utilities")]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
  let timestamp: DateTime<Utc> = chrono::offset::Utc::now();
  let ping_time = ctx.ping().await;

  let result = if ping_time.is_zero() {
    "Unavailable".into()
  } else {
    format!("{ping_time:#.0?}")
  };

  let reply = CreateReply::default().embed(
    CreateEmbed::new()
      .title("Gateway latency")
      .field("Gateway Latency", result, false)
      .footer(CreateEmbedFooter::new(format!(
        "Test by {}",
        ctx.author().name
      )))
      .timestamp(timestamp)
      .color(Colour::PURPLE),
  );
  ctx.send(reply).await?;

  Ok(())
}

inventory::submit! { MyCommand(ping) }
