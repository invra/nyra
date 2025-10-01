/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: Utils/Moderation/PurgeCommand.cs
    Authors: Invra
    Notes: A purge command
*/

namespace Nyra.Commands {
  [Category("Moderation")]
  public class PurgeCommand : ModuleBase<SocketCommandContext> {
    private readonly DiscordSocketClient client;

    public PurgeCommand(DiscordSocketClient client) {
      this.client = client;
    }

    [Command("purge")]
    [Summary("Deletes a specified number of messages from the current channel.")]
    [RequireUserPermission(GuildPermission.ManageMessages)]
    [RequireBotPermission(GuildPermission.ManageMessages)]
    public async Task PurgeAsync(int amount) {
      if (amount <= 0) {
        await ReplyAsync("Please specify a number greater than 0.");
        return;
      }

      var messages = await Context.Channel.GetMessagesAsync(amount + 1).FlattenAsync();
      var filtered = messages.Where(m => (DateTimeOffset.UtcNow - m.Timestamp).TotalDays <= 14);
      if (Context.Channel is ITextChannel textChannel) {
        await textChannel.DeleteMessagesAsync(filtered);
      } else {
        await ReplyAsync("This command only works in text channels, not DMs.");
        return;
      }

      var confirmation = await ReplyAsync($"Deleted {filtered.Count() - 1} messages.");
      await Task.Delay(3000);
      await confirmation.DeleteAsync();
    }
  }
}
