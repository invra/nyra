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
}

impl BotLauncher {
  pub fn new(config: crate::config::Config) -> Self {
    Self { config }
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
          prefix: Some(self.config.general.prefix.to_string().into()),
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
          if ready.user.id.to_string().len() > 0 {
            utils::success("The bot has started");
          }
          utils::bot(&format!("Username is {}", ready.user.name));
          utils::bot(&format!("Id is {}", ready.user.id));
          utils::bot(&format!(
            "{}",
            if ready.user.bot {
              "Is a bot"
            } else {
              "Is a user"
            }
          ));
          poise::builtins::register_globally(ctx, &framework.options().commands).await?;
          Ok(Data {})
        })
      })
      .build();

    let client = serenity::ClientBuilder::new(token, intents)
      .framework(framework)
      .await;
    client.unwrap().start().await.unwrap();
  }
}
