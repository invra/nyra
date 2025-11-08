use {
  crate::helper::{
    Context,
    Error,
    MyCommand,
  },
  poise::command,
};

/// Purges messages
#[command(
  prefix_command,
  slash_command,
  category = "Moderation",
  required_permissions = "BAN_MEMBERS"
)]
pub async fn purge(
  ctx: Context<'_>,
  #[description = "Message count to purge"] count: u8,
) -> Result<(), Error> {
  let channel_id = ctx.channel_id();

  let messages = ctx
    .http()
    .get_messages(channel_id, None, Some(count))
    .await?;

  for msg in &messages {
    let _ = ctx.http().delete_message(channel_id, msg.id, None).await;
  }

  Ok(())
}

inventory::submit! { MyCommand(purge) }
