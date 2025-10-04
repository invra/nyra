/*
  SPDX-License-Identifier: Unlicense
  Project: Nyra
  File: Utils/Information/InfoCommand.cs
  Authors: Invra
  Notes: A bot information command
*/

using System.Diagnostics;
using System.Management;
using System.Runtime.InteropServices;
using Spectre.Console;
using System.Reflection;

public partial class Testing {
  [LibraryImport("libhardwareinfo", EntryPoint = "add")]
  public static partial int Add(int left, int right);
}

namespace Nyra.Commands
{
    [Category("Information")]
    public class TestCommand : ModuleBase<SocketCommandContext>
    {
        private readonly DiscordSocketClient client;
        public TestCommand(DiscordSocketClient client) => this.client = client;

        private static string FormatUptime(TimeSpan uptime)
        {
            var parts = new List<string>();

            if (uptime.Days > 0) parts.Add($"{uptime.Days}d");
            if (uptime.Hours > 0) parts.Add($"{uptime.Hours}h");
            if (uptime.Minutes > 0) parts.Add($"{uptime.Minutes}m");
            if (uptime.Seconds > 0) parts.Add($"{uptime.Seconds}s");
            if (parts.Count == 0) return "0s";

            return string.Join(" ", parts);
        }

        [Command("test")]
        [Summary("Testing command for libary.")]
        public async Task PingAsync()
        {
            var msg = await ReplyAsync("Gathering...");
            TimeSpan uptime = DateTime.UtcNow - Process.GetCurrentProcess().StartTime.ToUniversalTime();
            string uptimeString = FormatUptime(uptime);

            var dotnetVersion = Environment.Version.ToString();
            var discordNetVersion = Assembly.GetAssembly(typeof(DiscordSocketClient))?
              .GetName().Version?.ToString() ?? "Unknown";

            var embed = new EmbedBuilder()
              .WithTitle("Testing Embed")
              .WithColor(Discord.Color.Orange)
              .AddField("1 + 3", $"{Testing.Add(1, 3)}", true)
              .WithFooter(footer => footer.Text = $"Info requested by {Context.User.Username}")
              .WithCurrentTimestamp()
              .Build();

            await msg.ModifyAsync(m =>
            {
                m.Content = string.Empty;
                m.Embed = embed;
            });
        }
    }
}
