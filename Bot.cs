using Discord;
using Discord.WebSocket;
using TerryDavis.Commands;
using TerryDavis.Config;

namespace TerryDavis {
  public class Bot {
    private readonly DiscordSocketClient _client;
    private readonly CommandHandler _commandHandler;
    private readonly BotConfig _config;

    public Bot() {
      _client = new DiscordSocketClient(
          new DiscordSocketConfig {
            AlwaysDownloadUsers = true,
            MessageCacheSize = 100,
            GatewayIntents =
                  GatewayIntents.Guilds
                  | GatewayIntents.GuildMembers
                  | GatewayIntents.GuildMessageReactions
                  | GatewayIntents.GuildMessages
                  | GatewayIntents.MessageContent
                  | GatewayIntents.GuildVoiceStates,
          }
      );

      _config = new BotConfig();
      _commandHandler = new CommandHandler(_client, _config);
    }

    public async Task RunAsync() {
      string token =
          Environment.GetEnvironmentVariable("DISCORD_TOKEN")
          ?? throw new InvalidOperationException("Missing DISCORD_TOKEN in .env");

      _client.Log += LogAsync;
      await _client.LoginAsync(TokenType.Bot, token);
      await _client.StartAsync();

      await _commandHandler.InitializeAsync();

      await Task.Delay(-1);
    }

    private Task LogAsync(LogMessage msg) {
      Console.WriteLine(msg);
      return Task.CompletedTask;
    }
  }
}
