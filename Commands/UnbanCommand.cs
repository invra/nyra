namespace Nyra.Commands {
  public class UnbanCommand : ModuleBase<SocketCommandContext> {
    private readonly DiscordSocketClient client;

    public UnbanCommand(DiscordSocketClient client) {
      this.client = client;
    }

    [Command("unban")]
    [Summary("Unbans a user by ID.")]
    public async Task UnbanAsync(string userIdInput) {
      if (Context.Guild == null) {
        await ReplyAsync("This command cannot be used in DMs.");
        return;
      }

      if (
          !ulong.TryParse(
              userIdInput.Replace("<@!", "").Replace("<@", "").Replace(">", ""),
              out ulong userId
          )
      ) {
        await ReplyAsync("Invalid user ID.");
        return;
      }

      var mod = (SocketGuildUser)Context.User;
      if (!mod.GuildPermissions.BanMembers) {
        await ReplyAsync("You don't have permission to unban members!");
        return;
      }

      try {
        RestBan? banInfo = null;
        await foreach (var batch in Context.Guild.GetBansAsync()) {
          banInfo = batch.FirstOrDefault(b => b.User.Id == userId);
          if (banInfo != null)
            break;
        }

        if (banInfo == null) {
          await ReplyAsync("That user is not banned.");
          return;
        }

        await Context.Guild.RemoveBanAsync(userId);

        var embed = new EmbedBuilder()
            .WithTitle("User Unbanned")
            .WithColor(Color.Orange)
            .AddField(
                "Unbanned User",
                $"{banInfo.User.Username}#{banInfo.User.Discriminator}",
                true
            )
            .AddField("Moderator", Context.User.Username, true)
            .WithCurrentTimestamp()
            .Build();

        await ReplyAsync(embed: embed);
      } catch (Exception ex) {
        await ReplyAsync($"Failed to unban user: {ex.Message}");
      }
    }
  }
}
