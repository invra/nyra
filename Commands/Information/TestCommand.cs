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
  public class TestCommand : ModuleBase<SocketCommandContext> {
    private readonly DiscordSocketClient client;
    public TestCommand(DiscordSocketClient client) => this.client = client;

    [Command("test")]
    [Summary("Testing command for library.")]
    public async Task PingAsync() {
      GetHardwareInfo hardware = new GetHardwareInfo();
      var msg = await ReplyAsync("Gathering...");

      var embed = new EmbedBuilder()
        .WithTitle("Testing Embed")
        .WithColor(Discord.Color.Orange)
        .AddField("CPU Model", hardware.CpuModel, true)
        .AddField("CPU Cores", hardware.CpuCores, true)
        .AddField("Memory Usage", hardware.MemoryTotal, true)
        .WithFooter(footer => footer.Text = $"Info requested by {Context.User.Username}")
        .WithCurrentTimestamp()
        .Build();

      await msg.ModifyAsync(m => {
        m.Content = string.Empty;
        m.Embed = embed;
      });
    }
  }
}
