/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: commands/mod.rs
    Authors: Invra
    Notes: Crate for the commands!!!!
*/

use {
  chrono::{
    DateTime,
    Utc,
  },
  poise::{
    Command,
    CreateReply,
    command,
    serenity_prelude::{
      CreateMessage,
      Timestamp,
      User,
      builder::{
        CreateEmbed,
        CreateEmbedAuthor,
        CreateEmbedFooter,
      },
    },
  },
};

#[derive(Debug)]
pub struct Data;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub struct MyCommand(fn() -> Command<Data, Error>);

inventory::collect!(MyCommand);

/// Ping command
#[command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
  let timestamp: DateTime<Utc> = chrono::offset::Utc::now();

  let reply = CreateReply::default().embed(
    CreateEmbed::new()
      .title("Gateway latency")
      .field("Gateway Latency", format!("implement NOWWW",), false)
      .footer(CreateEmbedFooter::new(format!(
        "Test by {}",
        ctx.author().name
      )))
      .timestamp(timestamp)
      .color(0x3498db),
  );

  ctx.send(reply).await?;

  Ok(())
}
inventory::submit! { MyCommand(ping) }

/// Gets user join date
#[command(prefix_command, slash_command)]
pub async fn age(
  ctx: Context<'_>,
  #[description = "User to check"] user: Option<User>,
) -> Result<(), Error> {
  let u = user.as_ref().unwrap_or_else(|| ctx.author());
  let response = format!("{}'s account was created at {}", u.name, u.created_at());

  ctx.say(response).await?;

  Ok(())
}
inventory::submit! { MyCommand(age) }

/// Show this menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
  ctx: Context<'_>,
  #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
  let config = poise::builtins::HelpConfiguration {
    ..Default::default()
  };
  poise::builtins::help(ctx, command.as_deref(), config)
    .await
    .map_err(Into::into)
}
inventory::submit! { MyCommand(help) }

#[inline(always)]
pub fn all() -> Vec<Command<Data, Error>> {
  inventory::iter::<MyCommand>().map(|x| (x.0)()).collect()
}
