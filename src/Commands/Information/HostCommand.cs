/*
  SPDX-License-Identifier: Unlicense
  Project: Nyra
  File: Commands/Information/TestCommand.cs
  Authors: Invra
  Notes: A bot information command
*/

using System;
using System.Diagnostics;
using System.Reflection;
using Discord;
using Discord.Commands;
using Discord.WebSocket;
using Nyra.HardwareInfo;

namespace Nyra.Commands {
  [Category("Information")]
  public class HostCommand : ModuleBase<SocketCommandContext> {
    private readonly DiscordSocketClient client;
    public HostCommand(DiscordSocketClient client) => this.client = client;

    [Command("host")]
    [Summary("Get bot host's information.")]
    public async Task HostAsync() {
      GetHardwareInfo hardware = new GetHardwareInfo();
      var msg = await ReplyAsync("Gathering...");

      var embed = new EmbedBuilder()
        .WithTitle("Host Info")
        .WithColor(Discord.Color.Purple)
        .AddField("CPU Model", hardware.CpuModel ?? "Unknown", true)
        .AddField("Processors", $"{hardware.CpuCores}", true)
        .AddField("Memory", $"{hardware.MemoryUsed:F2} GB / {hardware.MemoryTotal:F2} GB", true)
        .AddField("OS", hardware.OsVersion, true)
        .AddField("64-bit Proc", Environment.Is64BitProcess, true)
        .AddField("Elevated Proc", Environment.IsPrivilegedProcess, true)
        .WithFooter(footer => footer.Text = $"Host info requested by {Context.User.Username}")
        .WithCurrentTimestamp()
        .Build();

      await msg.ModifyAsync(m => {
        m.Content = string.Empty;
        m.Embed = embed;
      });
    }
  }
}
