using Nyra.Colourise;
using Nyra.Commands;
using Nyra.Config;

namespace Nyra {
  public class Bot {
    private readonly DiscordSocketClient client;
    private readonly CommandHandler commandHandler;
    private static readonly Lazy<Bot> instance = new Lazy<Bot>(() => new Bot());
    public static Bot Nyra { get { return instance.Value; } }

    private Bot() {
      Console.WriteLine($"{"[STDOUT/status]:".Cyan().Bold()} Initialising the Discord bot");
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

      commandHandler = new CommandHandler(client, BotConfig.Config);

      client.Ready += () => {
        Console.WriteLine($"{"[STDOUT/status]:".Cyan().Bold()} Bot is online");
        Console.WriteLine($"{"[STDOUT/status]:".Cyan().Bold()} Bot username is: {client.CurrentUser.Username}");
        Console.WriteLine($"{"[STDOUT/status]:".Cyan().Bold()} Bot Id: {client.CurrentUser.Id}");
        Console.WriteLine($"{"[STDOUT/status]:".Cyan().Bold()} Is a Bot: {client.CurrentUser.IsBot}");
        return Task.CompletedTask;
      };
    }

    public async Task RunAsync() {
      client.Log += LogAsync;
      await client.LoginAsync(TokenType.Bot, BotConfig.Config.Token);
      await client.StartAsync();

      await commandHandler.InitializeAsync();

      await Task.Delay(-1);
    }

    private Task LogAsync(LogMessage message) {
      if (message.Severity != LogSeverity.Info) {
        Console.WriteLine($"{$"[STDOUT/{message.Severity}]:".Yellow().Bold()} {message.Message}");
      }
      return Task.CompletedTask;
    }
  }
}
