/*
 *  SPDX-License-Identifier: Unlicense
 *  Project: Nyra
 *  Crate: nyra_commands
 *  File: information/host/mod.rs
 *  Authors: Invra, Hiten-Tandon
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
  mongodb::{
    Client,
    bson::doc,
  },
  nyra_utils::log,
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

pub async fn get_mongo_ver() -> String {
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
      return "?.?".into()
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
          return ver.to_string();
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
      .field("MongoDB Version", get_mongo_ver().await, false)
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
