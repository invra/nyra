namespace TerryDavis.Commands {
  public class HelpCommand : ModuleBase<SocketCommandContext> {
    private readonly CommandService _commands;

    public HelpCommand(CommandService commands) {
      _commands = commands;
    }

    [Command("help")]
    [Summary("Shows a list of available commands.")]
    public async Task HelpAsync() {
      var embed = new EmbedBuilder()
        .WithTitle("Commands List")
        .WithColor(Color.Blue)
        .WithFooter($"Requested by {Context.User.Username}")
        .WithCurrentTimestamp();

      foreach (var module in _commands.Modules) {
        var commandDescriptions = new List<string>();

        foreach (var cmd in module.Commands) {
          var preconditionResult = await cmd.CheckPreconditionsAsync(Context);
          if (preconditionResult.IsSuccess) {
            commandDescriptions.Add(
              $"{cmd.Summary ?? "No summary"}"
            );
          }
        }

        if (commandDescriptions.Count > 0) {
          embed.AddField(module.Name, string.Join("\n", commandDescriptions));
        }
      }

      await ReplyAsync(embed: embed.Build());
    }
  }
}
