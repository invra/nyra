/*  SPDX-License-Identifier: Unlicense
    Project: Nyra
    File: Utils/Information/InfoCommand.cs
    Authors: Invra
    Notes: A bot information command
*/

using System.Diagnostics;
using System.Reflection;

namespace Nyra.Commands {
  [Category("Information")]
  public class InfoCommand : ModuleBase<SocketCommandContext> {
    private readonly DiscordSocketClient client;

    public InfoCommand(DiscordSocketClient client) {
      this.client = client;
    }

    private static string FormatUptime(TimeSpan uptime) {
      var parts = new List<string>();

      if (uptime.Days > 0) parts.Add($"{uptime.Days}d");
      if (uptime.Hours > 0) parts.Add($"{uptime.Hours}h");
      if (uptime.Minutes > 0) parts.Add($"{uptime.Minutes}m");
      if (uptime.Seconds > 0) parts.Add($"{uptime.Seconds}s");

      if (parts.Count == 0) return "0s";

      return string.Join(" ", parts);
    }

    [Command("info")]
    [Summary("Replies with information about the bot itself.")]
    public async Task PingAsync() {
      var msg = await ReplyAsync("Calculating info...");

      TimeSpan uptime = DateTime.UtcNow - Process.GetCurrentProcess().StartTime.ToUniversalTime();
      string uptimeString = FormatUptime(uptime);
      var dotnetVersion = Environment.Version.ToString();
      var discordNetVersion = Assembly.GetAssembly(typeof(DiscordSocketClient))?
        .GetName().Version?.ToString() ?? "Unknown";

      var embed = new EmbedBuilder()
        .WithTitle("Project Info")
        .WithColor(Color.Orange)
        .AddField("C# Code (Lines)", $"{getFileLineCounts("cs")} lines", true)
        .AddField("Runtime", $".NET {dotnetVersion}", true)
        .AddField("Discord.Net", $"v{discordNetVersion}", true)
        .AddField("Uptime", uptimeString, true)
        .AddField("Users", client.Guilds.Sum(g => g.MemberCount), true)
        .AddField("Guilds", client.Guilds.Count, true)
        .WithFooter(footer => footer.Text = $"Info requested by {Context.User.Username}")
        .WithCurrentTimestamp()
        .Build();

      await msg.ModifyAsync(m => {
        m.Content = string.Empty;
        m.Embed = embed;
      });
    }

    private static int getFileLineCounts(string extension) {
      if (extension.StartsWith(".")) extension = extension[1..];
      string currentDirectory = Directory.GetCurrentDirectory();
      var files = Directory.GetFiles(currentDirectory, $"*.{extension}", SearchOption.AllDirectories);
      int totalLines = 0;

      foreach (var file in files) {
        try {
          totalLines += File.ReadAllLines(file).Length;
        } catch (Exception ex) {
          Console.WriteLine($"Failed to read {file}: {ex.Message}");
        }
      }

      return totalLines;
    }
  }
}
