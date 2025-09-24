using System.Reflection;
using Discord;
using Discord.Commands;
using Discord.WebSocket;
using Microsoft.Extensions.DependencyInjection;
using TerryDavis.Config;

namespace TerryDavis.Commands {
  public class CommandHandler {
    private readonly DiscordSocketClient _client;
    private readonly CommandService _commands;
    private readonly BotConfig _config;
    private readonly IServiceProvider _services;

    public CommandHandler(DiscordSocketClient client, BotConfig config) {
      _client = client;
      _commands = new CommandService();
      _config = config;

      var collection = new ServiceCollection();
      collection.AddSingleton(_client);
      collection.AddSingleton(_config);
      _services = collection.BuildServiceProvider();
    }

    public async Task InitializeAsync() {
      await _commands.AddModulesAsync(Assembly.GetEntryAssembly(), _services);

      _client.MessageReceived += HandleCommandAsync;
    }

    private async Task HandleCommandAsync(SocketMessage rawMessage) {
      if (rawMessage is not SocketUserMessage message)
        return;
      if (message.Source != MessageSource.User)
        return;

      int argPos = 0;
      if (!message.HasStringPrefix(_config.Prefix, ref argPos))
        return;

      var context = new SocketCommandContext(_client, message);
      var result = await _commands.ExecuteAsync(context, argPos, _services);

      if (!result.IsSuccess) {
        if (result.Error == CommandError.UnknownCommand)
          await context.Channel.SendMessageAsync("This command doesn't exist!");
        else
          Console.WriteLine($"[Command Error] {result.ErrorReason}");
      }
    }
  }
}
