use {
  crate::commands::{
    MyCommand,
    helper::*,
  },
  poise::{
    command,
    serenity_prelude::User,
  },
};

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
inventory::submit! { MyCommand(|| age()) }
