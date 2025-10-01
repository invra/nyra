/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: Utils/CommandHandler.cs
    Authors: Invra
    Notes: Simple command handler to manage all deeper commands
*/

global using System.Text;
global using System.Text.RegularExpressions;
global using Discord;
global using Discord.Commands;
global using Discord.Rest;
global using Discord.WebSocket;
using System.Reflection;
using Microsoft.Extensions.DependencyInjection;
using Nyra.Config;

namespace Nyra.Commands {
  [AttributeUsage(AttributeTargets.Class, AllowMultiple = false)]
  public class CategoryAttribute : Attribute {
    public string Name { get; }

    public CategoryAttribute(string name) {
      Name = name;
    }
  }

  public class CommandHandler {
    private readonly DiscordSocketClient client;
    private readonly CommandService commands;
    private readonly BotConfig config;
    private readonly IServiceProvider services;

    public CommandHandler(DiscordSocketClient client, BotConfig config) {
      this.client = client;
      commands = new CommandService();
      this.config = config;

      var collection = new ServiceCollection();
      collection.AddSingleton(this.client);
      collection.AddSingleton(this.config);
      services = collection.BuildServiceProvider();
    }

    public async Task InitializeAsync() {
      await commands.AddModulesAsync(Assembly.GetEntryAssembly(), services);

      client.MessageReceived += HandleCommandAsync;
    }

    private async Task HandleCommandAsync(SocketMessage rawMessage) {
      if (rawMessage is not SocketUserMessage message)
        return;
      if (message.Source != MessageSource.User)
        return;

      int argPos = 0;
      if (!message.HasStringPrefix(config.Prefix, ref argPos))
        return;

      var context = new SocketCommandContext(client, message);
      var result = await commands.ExecuteAsync(context, argPos, services);

      if (!result.IsSuccess) {
        if (result.Error == CommandError.UnknownCommand)
          await context.Channel.SendMessageAsync("This command doesn't exist!");
        else
          Console.WriteLine($"[Command Error] {result.ErrorReason}");
      }
    }
  }
}
