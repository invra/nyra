using Discord;
using Discord.WebSocket;
using TerryDavis.Commands;
using TerryDavis.Config;

namespace TerryDavis {
  public class Bot {
    private readonly DiscordSocketClient client;
    private readonly CommandHandler commandHandler;
    private readonly BotConfig config;


    public Bot() {
      config = new BotConfig();
      Console.WriteLine($"\x1b[1;36m[STDOUT/status]:\x1b[0m Initialising the Discord bot");
      client = new DiscordSocketClient(
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

      commandHandler = new CommandHandler(client, config);

      client.Ready += () => {
        Console.WriteLine($"\x1b[1;36m[STDOUT/status]:\x1b[0m Bot is online");
        Console.WriteLine($"\x1b[1;36m[STDOUT/status]:\x1b[0m Bot username is: {client.CurrentUser.Username}");
        Console.WriteLine($"\x1b[1;36m[STDOUT/status]:\x1b[0m Bot Id: {client.CurrentUser.Id}");
        Console.WriteLine($"\x1b[1;36m[STDOUT/status]:\x1b[0m Is a Bot: {client.CurrentUser.IsBot}");
        return Task.CompletedTask;
      };
    }

    public async Task RunAsync() {
      string token = Environment.GetEnvironmentVariable("DISCORD_TOKEN")!;
      client.Log += LogAsync;
      await client.LoginAsync(TokenType.Bot, token);
      await client.StartAsync();

      await commandHandler.InitializeAsync();

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
