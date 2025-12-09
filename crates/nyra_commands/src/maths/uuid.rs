/*
 *  SPDX-License-Identifier: Unlicense
 *  Project: Nyra
 *  Crate: nyra_commands
 *  File: maths/uuid.rs
 *  Authors: Invra, Hiten-Tandon
 */

use {
  crate::helper::{
    Context,
    Error,
    MyCommand,
  },
  poise::command,
  uuid::Uuid,
};

/// Random uuid gen
#[command(prefix_command, slash_command, category = "Maths")]
pub async fn uuid(ctx: Context<'_>) -> Result<(), Error> {
  let id = Uuid::new_v4();

  ctx.say(format!("Generated UUID: `{id}`")).await?;
  Ok(())
}

inventory::submit! { MyCommand(uuid) }
