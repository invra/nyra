using Discord;
using Discord.Commands;
using Discord.WebSocket;

namespace TerryDavis.Commands {
  public class KickCommand : ModuleBase<SocketCommandContext> {
    private readonly DiscordSocketClient client;

    public KickCommand(DiscordSocketClient client) {
      this.client = client;
    }

    [Command("kick")]
    [Summary("Kicks a user by mention or ID.")]
    public async Task KickAsync(
        string userInput,
        [Remainder] string reason = "No reason provided"
    ) {
      if (Context.Guild == null) {
        await ReplyAsync("This command cannot be used in DMs.");
        return;
      }

      var guildUser = ParseUser(userInput);
      if (guildUser == null) {
        await ReplyAsync("User not found. Make sure to mention them or use their ID.");
        return;
      }

      var mod = (SocketGuildUser)Context.User;
      if (!mod.GuildPermissions.KickMembers) {
        await ReplyAsync("You don't have permission to kick members!");
        return;
      }

      if (guildUser.Id == Context.User.Id) {
        await ReplyAsync("You can't kick yourself!");
        return;
      }

      if (guildUser.GuildPermissions.Administrator) {
        await ReplyAsync("You can't kick an administrator!");
        return;
      }

      try {
        await guildUser.KickAsync(reason);

        var msg = await ReplyAsync($"Kicking {guildUser.Username}...");

        var embed = new EmbedBuilder()
            .WithTitle("User Kicked")
            .WithColor(Color.Green)
            .AddField(
                "Kicked User",
                $"{guildUser.Username}#{guildUser.Discriminator}",
                true
            )
            .AddField("Moderator", Context.User.Username, true)
            .AddField("Reason", reason)
            .WithCurrentTimestamp()
            .Build();

        await msg.ModifyAsync(m => {
          m.Content = string.Empty;
          m.Embed = embed;
        });
      } catch (Exception ex) {
        await ReplyAsync($"Failed to kick {guildUser.Username}: {ex.Message}");
      }
    }

    private SocketGuildUser? ParseUser(string input) {
      if (input.StartsWith("<@") && input.EndsWith(">"))
        input = input.Replace("<@!", "").Replace("<@", "").Replace(">", "");

      if (!ulong.TryParse(input, out ulong id))
        return null;

      return Context.Guild.GetUser(id);
    }
  }
}
