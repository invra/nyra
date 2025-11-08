/*
  SPDX-License-Identifier: Unlicense
  Project: Nyra
  File: commands/information/ping.rs
  Authors: Invra
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
      User,
    },
  },
};

/// Kick command
#[command(
  prefix_command,
  slash_command,
  category = "Moderation",
  required_permissions = "KICK_MEMBERS"
)]
pub async fn kick(
  ctx: Context<'_>,
  #[description = "User to check"] user: User,
  #[description = "Reason"] reason: Option<String>,
) -> Result<(), Error> {
  let timestamp: DateTime<Utc> = chrono::offset::Utc::now();
  let u = user.id;
  let r = reason;
  let guild = ctx
    .guild_id()
    .ok_or("This command can only be used in a guild")?;

  let reply = CreateReply::default().embed(
    CreateEmbed::new()
      .title("Kick Command")
      .description(format!(
        "Kicked <@{}> for {}.",
        u.get(),
        r.as_deref().unwrap_or("No reason provided")
      ))
      .timestamp(timestamp)
      .color(Colour::DARK_GREEN),
  );

  if ctx.author().id == user.id {
    return Err("You cannot kick yourself.".into());
  }

  if let Err(err) = ctx.http().kick_member(guild, user.id, r.as_deref()).await {
    return Err(format!("Failed to kick user: {err:?}").into());
  }

  ctx.send(reply).await?;

  Ok(())
}

inventory::submit! { MyCommand(kick) }
