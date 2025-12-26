/*
 *  SPDX-License-Identifier: Unlicense
 *  Project: Nyra
 *  Crate: nyra_core
 *  File: lib.rs
 *  Authors: Invra, Hiten-Tandon
 */

use {
  mongodb::bson::doc,
  nyra_config::Config,
  nyra_utils::{
    clear_runtime_info,
    log,
    set_runtime_info,
  },
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

pub async fn run_db_check() -> Result<(), ()> {
  log::info!("Checking connection to MongoDB server");
  let config = nyra_config::Config::global();

  let user = config
    .db
    .clone()
    .unwrap_or_default()
    .username
    .unwrap_or_default();
  let host = config
    .db
    .clone()
    .unwrap_or_default()
    .host
    .unwrap_or_default();
  let port = config
    .db
    .clone()
    .unwrap_or_default()
    .port
    .unwrap_or_default()
    .to_string();

  let uri = format!(
    "mongodb://{user}:{}@{host}:{port}/?authSource=admin",
    config
      .db
      .clone()
      .unwrap_or_default()
      .password
      .unwrap_or_default(),
  );

  let client = match mongodb::Client::with_uri_str(&uri).await {
    Ok(c) => c,
    Err(e) => {
      log::error!("Failed to parse MongoDB URI: {e}");
      return Err(());
    }
  };

  let admin_db = client.database("admin");
  let result = admin_db
    .run_command(doc! { "getParameter": 1, "featureCompatibilityVersion": 1 })
    .await;

  match result {
    Ok(_) => {
      log::success!("Server connection for {user}@{host}:{port} was successful");
      Ok(())
    }
    Err(e) => {
      log::error!("Server connection for {user}@{host}:{port} failed: {e}");
      Err(())
    }
  }
}

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

  pub fn is_running() -> bool {
    INSTANCE.get().is_some_and(|this| {
      this
        .shard_manager
        .try_read()
        .map_or(true, |lock| lock.is_some())
    })
  }

  pub async fn start() {
    use poise::serenity_prelude::{
      Client,
      GatewayIntents,
    };

    let bi = Self::instance();

    let token = bi.config.general.token.clone();
    let intents = GatewayIntents::GUILD_MESSAGES
      | GatewayIntents::DIRECT_MESSAGES
      | GatewayIntents::MESSAGE_CONTENT;

    set_runtime_info();

    log::bot!("Starting bot…");

    let framework = poise::Framework::builder()
      .options(poise::FrameworkOptions {
        prefix_options: poise::PrefixFrameworkOptions {
          prefix: Some(
            bi.config
              .general
              .prefix
              .clone()
              .unwrap_or_default()
              .to_string(),
          ),
          edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
            std::time::Duration::from_secs(3600),
          ))),
          case_insensitive_commands: true,
          ..Default::default()
        },
        commands: nyra_commands::all(),
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

          Ok(nyra_commands::helper::Data {})
        })
      })
      .build();

    let mut client = Client::builder(token, intents)
      .framework(framework)
      .await
      .expect("Error creating client");

    {
      let mut lock = bi.shard_manager.write().await;
      *lock = Some(client.shard_manager.clone());
    }

    if let Err(e) = client.start().await {
      log::error!("Client exited with error: {e}");
    }
  }

  #[allow(dead_code)]
  pub async fn stop() {
    let bi = Self::instance();
    let lock = bi.shard_manager.read().await;

    if let Some(manager) = &*lock {
      log::bot!("Stopping bot gracefully…");
      manager.shutdown_all().await;
      clear_runtime_info();
      log::success!("Bot has been stopped.");
    } else {
      log::error!("Cannot stop bot — shard manager not initialized.");
    }
  }
}
