/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: Utils/Information/HelpCommand.cs
    Authors: Invra
    Notes: A help command
*/

namespace Nyra.Commands {
  [Category("Information")]
  public class HelpCommand : ModuleBase<SocketCommandContext> {
    private readonly CommandService commands;
    private readonly DiscordSocketClient client;

    public HelpCommand(CommandService commands, DiscordSocketClient client) {
      this.commands = commands;
      this.client = client;
    }

    [Command("help")]
    [Summary("Shows a list of available commands.")]
    public async Task HelpAsync() {
      var categorizedCommands = GetCategorizedCommands();
      var embed = BuildOverviewEmbed(categorizedCommands);

      var menuBuilder = new SelectMenuBuilder()
        .WithPlaceholder("Select a category")
        .WithCustomId("help_category_select")
        .WithMinValues(1)
        .WithMaxValues(1);

      menuBuilder.AddOption("All Commands", "all", "View all available commands", new Emoji("📖"));
      foreach (var category in categorizedCommands.Keys.OrderBy(c => c)) {
        var emoji = GetCategoryEmoji(category);
        menuBuilder.AddOption(category, category.ToLower(), $"View {category} commands", emoji);
      }

      var components = new ComponentBuilder()
        .WithSelectMenu(menuBuilder)
        .Build();

      var message = await ReplyAsync(embed: embed, components: components);
      var interactionTask = Task.Run(async () => {
        var timeout = TimeSpan.FromMinutes(5);
        var cts = new CancellationTokenSource(timeout);

        try {
          while (!cts.Token.IsCancellationRequested) {
            var interaction = await InteractionUtility.WaitForInteractionAsync(
              client,
              timeout,
              x => x is SocketMessageComponent component &&
                   component.Data.CustomId == "help_category_select" &&
                   component.Message.Id == message.Id
            );

            if (interaction is SocketMessageComponent component) {
              var selectedCategory = component.Data.Values.First();

              Embed newEmbed;
              if (selectedCategory == "all") {
                newEmbed = BuildOverviewEmbed(categorizedCommands);
              } else {
                newEmbed = BuildCategoryEmbed(selectedCategory, categorizedCommands);
              }

              await component.UpdateAsync(x => {
                x.Embed = newEmbed;
                x.Components = components;
              });
            }
          }
        } catch (TimeoutException) {
          await message.ModifyAsync(x => x.Components = new ComponentBuilder().Build());
        }
      });
    }

    private Dictionary<string, List<string>> GetCategorizedCommands() {
      var categorizedCommands = new Dictionary<string, List<string>>();

      foreach (var module in this.commands.Modules) {
        var moduleTypeInfo = module.Attributes.OfType<CategoryAttribute>().FirstOrDefault();
        string category = moduleTypeInfo?.Name ?? "Uncategorized";

        if (!categorizedCommands.ContainsKey(category)) {
          categorizedCommands[category] = new List<string>();
        }

        foreach (var cmd in module.Commands) {
          var preconditionResult = cmd.CheckPreconditionsAsync(Context, null).Result;
          if (preconditionResult.IsSuccess) {
            string commandInfo = $"**{cmd.Name}** - {cmd.Summary ?? "No summary"}";
            categorizedCommands[category].Add(commandInfo);
          }
        }
      }

      return categorizedCommands;
    }

    private Embed BuildOverviewEmbed(Dictionary<string, List<string>> categorizedCommands) {
      var embed = new EmbedBuilder()
        .WithTitle("📖 Command Help")
        .WithDescription("Use the dropdown menu below to browse commands by category.")
        .WithColor(Color.Blue)
        .WithFooter($"Requested by {Context.User.Username}")
        .WithCurrentTimestamp();

      foreach (var category in categorizedCommands.OrderBy(c => c.Key)) {
        if (category.Value.Count > 0) {
          var emoji = GetCategoryEmoji(category.Key);
          embed.AddField(
            $"{emoji} {category.Key}",
            $"{category.Value.Count} command(s)",
            true
          );
        }
      }

      return embed.Build();
    }

    private Embed BuildCategoryEmbed(string categoryName, Dictionary<string, List<string>> categorizedCommands) {
      var category = categorizedCommands.FirstOrDefault(c => c.Key.ToLower() == categoryName.ToLower());
      var emoji = GetCategoryEmoji(category.Key);

      var embed = new EmbedBuilder()
        .WithTitle($"{emoji} {category.Key} Commands")
        .WithDescription(string.Join("\n", category.Value))
        .WithColor(GetCategoryColor(category.Key))
        .WithFooter($"Requested by {Context.User.Username}")
        .WithCurrentTimestamp();

      return embed.Build();
    }

    private Emoji GetCategoryEmoji(string category) {
      return category switch {
        "Information" => new Emoji("📚"),
        "Moderation" => new Emoji("🛡️"),
        "Utility" => new Emoji("🔧"),
        _ => new Emoji("📁")
      };
    }

    private Color GetCategoryColor(string category) {
      return category switch {
        "Information" => Color.Blue,
        "Moderation" => Color.Red,
        "Utility" => Color.Green,
        _ => Color.Purple
      };
    }
  }

  public static class InteractionUtility {
    public static async Task<SocketInteraction> WaitForInteractionAsync(
      DiscordSocketClient client,
      TimeSpan timeout,
      Func<SocketInteraction, bool> filter
    ) {
      var tcs = new TaskCompletionSource<SocketInteraction>();
      var cts = new CancellationTokenSource(timeout);

      async Task Handler(SocketInteraction interaction) {
        if (filter(interaction)) {
          tcs.TrySetResult(interaction);
        }
        await Task.CompletedTask;
      }

      client.InteractionCreated += Handler;
      cts.Token.Register(() => tcs.TrySetException(new TimeoutException()));

      try {
        return await tcs.Task;
      } finally {
        client.InteractionCreated -= Handler;
        cts.Dispose();
      }
    }
  }
}
