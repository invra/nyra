/*
 *  SPDX-License-Identifier: Unlicense
 *  Project: Nyra
 *  Crate: nyra_core
 *  File: lib.rs
 *  Authors: Invra, Hiten-Tandon
 */

use {
  nyra_commands as commands,
  nyra_config::Config,
  nyra_utils::log,
  std::sync::{
    Arc,
    OnceLock,
  },
  tokio::sync::RwLock,
};

#[derive(Debug)]
pub struct BotLauncher {
  config: Arc<Config>,
  shard_manager: RwLock<Option<Arc<poise::serenity_prelude::ShardManager>>>,
}

static INSTANCE: OnceLock<Arc<BotLauncher>> = OnceLock::new();

impl BotLauncher {
  pub fn init_instance(config_arg: Option<String>) {
    if let Err(e) = Config::init_global(config_arg) {
      log::error!("{}", e);
      return;
    }

    let config = Config::global();

    INSTANCE
      .set(Arc::new(Self {
        config,
        shard_manager: RwLock::new(None),
      }))
      .expect("BotLauncher::init called more than once");
  }

  fn instance() -> Arc<Self> {
    INSTANCE
      .get()
      .expect("BotLauncher not initialized — call BotLauncher::init() first")
      .clone()
  }

  pub async fn start() {
    let this = Self::instance();
    this.start_bot().await;
  }

  #[allow(dead_code)]
  pub async fn stop() {
    let this = Self::instance();
    this.stop_bot().await;
  }

  pub fn is_running() -> bool {
    INSTANCE.get().is_some_and(|this| {
      this
        .shard_manager
        .try_read()
        .map_or(true, |lock| lock.is_some())
    })
  }

  async fn start_bot(&self) {
    use poise::serenity_prelude::{
      Client,
      GatewayIntents,
    };

    let token = self.config.general.token.clone();
    let intents = GatewayIntents::GUILD_MESSAGES
      | GatewayIntents::DIRECT_MESSAGES
      | GatewayIntents::MESSAGE_CONTENT;

    log::bot!("Starting bot…");

    let framework = poise::Framework::builder()
      .options(poise::FrameworkOptions {
        prefix_options: poise::PrefixFrameworkOptions {
          prefix: Some(self.config.general.prefix.clone()),
          edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
            std::time::Duration::from_secs(3600),
          ))),
          case_insensitive_commands: true,
          ..Default::default()
        },
        commands: commands::all(),
        ..Default::default()
      })
      .setup(|ctx, ready, framework| {
        Box::pin(async move {
          log::success!("The bot has started");
          log::bot!("Username: {}", ready.user.name);
          log::bot!("ID: {}", ready.user.id);
          poise::builtins::register_globally(ctx, &framework.options().commands).await?;
          for command in &framework.options().commands {
            let category = command
              .category
              .as_ref()
              .map_or_else(String::new, |cat| format!("from {}", cat.as_str()));
            log::bot!("Loaded command: {} {}", command.name, category);
          }

          Ok(crate::commands::helper::Data {})
        })
      })
      .build();

    let mut client = Client::builder(token, intents)
      .framework(framework)
      .await
      .expect("Error creating client");

    {
      let mut lock = self.shard_manager.write().await;
      *lock = Some(client.shard_manager.clone());
    }

    if let Err(e) = client.start().await {
      log::error!("Client exited with error: {e}");
    }
  }

  #[allow(dead_code)]
  async fn stop_bot(&self) {
    let lock = self.shard_manager.read().await;
    if let Some(manager) = &*lock {
      log::bot!("Stopping bot gracefully…");
      manager.shutdown_all().await;
      log::success!("Bot has been stopped.");
    } else {
      log::error!("Cannot stop bot — shard manager not initialized.");
    }
  }
}
