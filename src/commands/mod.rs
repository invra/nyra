/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: commands/mod.rs
    Authors: Invra
    Notes: Crate for the commands!!!!
*/

use poise::{Command, command, serenity_prelude::User};

#[derive(Debug)]
pub struct Data;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
  ctx.say("Pong! 🏓").await?;

  Ok(())
}

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

pub fn all() -> Vec<Command<Data, Error>> {
  vec![ping(), age()]
}
