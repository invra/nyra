/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: Bot.cs
    Authors: Invra
    Notes: Bot constructor
*/

using Nyra.Colourise;
using Nyra.Commands;
using Nyra.Config;
using Nyra.Stdout;

namespace Nyra {
  public class Bot {
    private readonly DiscordSocketClient client;
    private readonly CommandHandler commandHandler;
    private static readonly Lazy<Bot> instance = new Lazy<Bot>(() => new Bot());
    public static Bot Nyra { get { return instance.Value; } }

    private Bot() {
      ConsoleCalls.PrintStatus("Initialising the Discord bot");
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
        ConsoleCalls.PrintStatus("Bot is online");
        ConsoleCalls.PrintStatus($"Bot username is: {client.CurrentUser.Username}");
        ConsoleCalls.PrintStatus($"Bot Id: {client.CurrentUser.Id}");
        ConsoleCalls.PrintStatus($"Is a Bot: {client.CurrentUser.IsBot}");
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
        ConsoleCalls.PrintCustom(message.Message, message.Severity.ToString());
      }
      return Task.CompletedTask;
    }
  }
}
