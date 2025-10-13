/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: bot_launcher.rs
    Authors: Invra
    Notes: Bot launcher functionality
*/

use {
  crate::{
    config::Config,
    utils,
  },
  serenity::async_trait,
  serenity::model::channel::Message,
  serenity::prelude::*,
};

struct Handler {
  config: Config,
}

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    if msg.content.starts_with(&self.config.general.prefix) {
      if msg.content == format!("{}ping", &self.config.general.prefix) {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
          utils::error(&format!("Error sending message: {why:?}"));
        }
      }
    }
  }
}

pub struct BotLauncher {
  config: crate::config::Config,
}

impl BotLauncher {
  pub fn new(config: crate::config::Config) -> Self {
    Self { config }
  }

  pub async fn start_bot(&self) {
    let token = self.config.general.token.clone();
    let intents = GatewayIntents::GUILD_MESSAGES
      | GatewayIntents::DIRECT_MESSAGES
      | GatewayIntents::MESSAGE_CONTENT;

    utils::bot("Starting bot...");

    let handler = Handler {
      config: self.config.clone(),
    };

    let mut client = match Client::builder(&token, intents)
      .event_handler(handler)
      .await
    {
      Ok(client) => {
        utils::success("Bot client created successfully");
        client
      }
      Err(why) => {
        utils::error(&format!("Error creating client: {why:?}"));
        return;
      }
    };

    if let Err(why) = client.start().await {
      utils::error(&format!("Client error: {why:?}"));
    }
  }
}
