/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: bot_launcher.rs
    Authors: Invra
    Notes: Bot launcher functionality
*/

use {
  crate::utils,
  poise::serenity_prelude as serenity,
  std::sync::Arc,
};

struct Data {}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn age(
  ctx: Context<'_>,
  #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
  let u = user.as_ref().unwrap_or_else(|| ctx.author());
  let response = format!("{}'s account was created at {}", u.name, u.created_at());
  ctx.say(response).await?;
  Ok(())
}

pub struct BotLauncher {
  config: crate::config::Config,
  shard_manager: tokio::sync::RwLock<Option<Arc<serenity::ShardManager>>>,
}

impl BotLauncher {
  pub fn new(config: crate::config::Config) -> Self {
    Self {
      config,
      shard_manager: tokio::sync::RwLock::new(None),
    }
  }

  pub async fn start_bot(&self) {
    let token = self.config.general.token.clone();
    let intents = serenity::GatewayIntents::GUILD_MESSAGES
      | serenity::GatewayIntents::DIRECT_MESSAGES
      | serenity::GatewayIntents::MESSAGE_CONTENT;

    utils::bot("Starting bot…");

    let framework = poise::Framework::builder()
      .options(poise::FrameworkOptions {
        prefix_options: poise::PrefixFrameworkOptions {
          prefix: Some(self.config.general.prefix.to_string()),
          edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
            std::time::Duration::from_secs(3600),
          ))),
          case_insensitive_commands: true,
          ..Default::default()
        },
        commands: vec![age()],
        ..Default::default()
      })
      .setup(|ctx, ready, framework| {
        Box::pin(async move {
          utils::success("The bot has started");
          utils::bot(&format!("Username is {}", ready.user.name));
          utils::bot(&format!("Id is {}", ready.user.id));
          utils::bot(if ready.user.bot {
            "Is a bot"
          } else {
            "Is a user"
          });
          poise::builtins::register_globally(ctx, &framework.options().commands).await?;
          Ok(Data {})
        })
      })
      .build();

    let mut client = serenity::Client::builder(token, intents)
      .framework(framework)
      .await
      .expect("Error creating client");

    {
      let mut lock = self.shard_manager.write().await;
      *lock = Some(client.shard_manager.clone());
    }

    if let Err(e) = client.start().await {
      utils::error(&format!("Client exited with error: {}", e));
    }
  }

  #[allow(dead_code)]
  pub async fn stop_bot(&self) {
    let lock = self.shard_manager.read().await;
    if let Some(manager) = &*lock {
      utils::bot("Stopping bot gracefully…");
      manager.shutdown_all().await;
      utils::success("Bot has been stopped.");
    } else {
      utils::error("Cannot stop bot — shard manager not initialized.");
    }
  }
}
