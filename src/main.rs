use {
  serde::Deserialize, serenity::async_trait, serenity::model::channel::Message,
  serenity::prelude::*, std::fs,
};

#[derive(Deserialize, Clone)]
struct Config {
  general: General,
}

#[derive(Deserialize, Clone)]
struct General {
  token: String,
  prefix: String,
}

struct Handler {
  config: Config,
}

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    if msg.content.starts_with(&self.config.general.prefix) {
      if msg.content == format!("{}ping", &self.config.general.prefix) {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
          println!("Error sending message: {why:?}");
        }
      }
    }
  }
}

async fn start_bot(config: Config) {
  let token = config.general.token.clone();
  let intents =
    GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

  let handler = Handler { config };

  let mut client = Client::builder(&token, intents)
      .event_handler(handler)
      .await
      .expect("Err creating client");

  if let Err(why) = client.start().await {
      println!("Client error: {why:?}");
  }
}

#[tokio::main]
async fn main() {
  let config_location = "/Users/invra/.config/nyra/nyra.toml";
  let fs_str = fs::read_to_string(config_location).unwrap();
  let config: Config = toml::from_str(&fs_str).unwrap();

  start_bot(config).await;
}
