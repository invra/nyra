use {
  crate::commands::{
    MyCommand,
    helper::*,
  },
  chrono::{
    DateTime,
    Utc,
  },
  poise::{
    CreateReply,
    command,
    serenity_prelude::{
      CreateEmbed,
      CreateEmbedFooter,
    },
  },
};

/// Ping command
#[command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
  let timestamp: DateTime<Utc> = chrono::offset::Utc::now();

  #[allow(clippy::useless_format)]
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
inventory::submit! { MyCommand(|| ping()) }
