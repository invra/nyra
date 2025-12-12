/*
 *  SPDX-License-Identifier: Unlicense
 *  Project: Nyra
 *  Crate: nyra_commands
 *  File: information/bot.rs
 *  Authors: Invra, Hiten-Tandon
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
  mongodb::{
    Client,
    bson::doc,
  },
  nyra_utils::{
    log,
    runtime_duration,
  },
  poise::{
    CreateReply,
    command,
    serenity_prelude::{
      Colour,
      CreateActionRow,
      CreateButton,
      CreateEmbed,
      CreateEmbedFooter,
    },
  },
};

const CRATE_VER: &str = env!("CARGO_PKG_VERSION");
const COMPILER_NAME: &str = compile_time::rustc_version_str!();
const COMPILE_TIME_ISO: &str = compile_time::datetime_str!();

fn discord_timestamp_from_iso(iso: &str) -> Box<str> {
  let dt: DateTime<Utc> = iso.parse().unwrap_or_default();
  let epoch = dt.timestamp();

  format!("<t:{}:f>", epoch).into()
}

pub async fn get_mongo_ver() -> Box<str> {
  let config = nyra_config::Config::global();

  let uri = format!(
    "mongodb://{}:{}@{}:{}/?authSource=admin",
    config.db.username.clone().unwrap_or("mongodb".into()),
    config.db.password.clone().unwrap_or("mongodb".into()),
    config.db.host.clone().unwrap_or("127.0.0.1".into()),
    config.db.port.clone().unwrap_or(27017).to_string(),
  );
  let client = match Client::with_uri_str(uri).await {
    Ok(c) => c,
    Err(e) => {
      log::error!("MongoDB client error: {e:?}");
      return "?.?".into();
    }
  };

  let admin = client.database("admin");

  let res = admin
    .run_command(doc! { "getParameter": 1, "featureCompatibilityVersion": 1 })
    .await;

  match res {
    Ok(doc) => {
      if let Some(fcv) = doc.get_document("featureCompatibilityVersion").ok() {
        if let Some(ver) = fcv.get_str("version").ok() {
          return ver.into();
        }
      }
      "unknown".into()
    }
    Err(e) => {
      log::error!("MONGO: getParameter error: {e:?}");
      "?.?".into()
    }
  }
}

/// Bot information command
#[command(prefix_command, slash_command, category = "Information")]
pub async fn bot(ctx: Context<'_>) -> Result<(), Error> {
  let timestamp: DateTime<Utc> = chrono::offset::Utc::now();

  let reply = CreateReply::default()
    .embed(
      CreateEmbed::new()
        .title("Bot Info")
        .field("Written with", "Rust", true)
        .field("MongoDB", format!("v{}", get_mongo_ver().await), true)
        .field("Commands Crate", format!("v{CRATE_VER}"), true)
        .field(
          "Compiled on",
          discord_timestamp_from_iso(COMPILE_TIME_ISO),
          true,
        )
        .field("Compiled by", COMPILER_NAME, true)
        .field(
          "Bot Uptime",
          format!(
            "`{}`",
            pretty_duration::pretty_duration(
              &runtime_duration().unwrap_or_default(),
              Some(pretty_duration::PrettyDurationOptions {
                output_format: Some(pretty_duration::PrettyDurationOutputFormat::Compact),
                singular_labels: None,
                plural_labels: None
              })
            )
          ),
          true,
        )
        .footer(CreateEmbedFooter::new(format!(
          "Host requested by {}",
          ctx.author().name
        )))
        .timestamp(timestamp)
        .color(Colour::PURPLE),
    )
    .components(vec![CreateActionRow::Buttons(vec![
      CreateButton::new_link(env!("CARGO_PKG_REPOSITORY")).label("Git Repo"),
    ])]);

  ctx.send(reply).await?;

  Ok(())
}
inventory::submit! { MyCommand(bot) }
