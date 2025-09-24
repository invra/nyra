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
      _config = new BotConfig();
      Console.WriteLine($"\x1b[1;36m[STDOUT/status]:\x1b[0m Initialising the Discord bot");
      _client = new DiscordSocketClient(
        new DiscordSocketConfig {
          AlwaysDownloadUsers = true,
          MessageCacheSize = 100,
          GatewayIntents = GatewayIntents.Guilds
            | GatewayIntents.GuildMembers
            | GatewayIntents.GuildMessageReactions
            | GatewayIntents.GuildMessages
            | GatewayIntents.MessageContent
            | GatewayIntents.GuildVoiceStates,
        }
      );

      _commandHandler = new CommandHandler(_client, _config);

      _client.Ready += () => {
        Console.WriteLine($"\x1b[1;36m[STDOUT/status]:\x1b[0m Bot is online");
        return Task.CompletedTask;
      };
    }

    public async Task RunAsync() {
      string token = Environment.GetEnvironmentVariable("DISCORD_TOKEN")!;
      _client.Log += LogAsync;
      await _client.LoginAsync(TokenType.Bot, token);
      await _client.StartAsync();

      await _commandHandler.InitializeAsync();

      await Task.Delay(-1);
    }

    private Task LogAsync(LogMessage message) {
      if (message.Severity != LogSeverity.Info) {
        Console.WriteLine($"\x1b[1;93m[STDOUT/{message.Severity}]:\x1b[0;33m {message.Message}\x1b[0m");
      }
      return Task.CompletedTask;
    }
  }
}
