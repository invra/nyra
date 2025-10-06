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
    public async Task TestAsync() {
      GetHardwareInfo hardware = new GetHardwareInfo();
      var msg = await ReplyAsync("Gathering...");

      var embed = new EmbedBuilder()
        .WithTitle("Testing Embed")
        .WithColor(Discord.Color.Blue)
        .AddField("CPU Model", hardware.CpuModel, true)
        .AddField("Processors", hardware.CpuCores, true)
        .AddField("Memory", $"{hardware.MemoryUsed:F2} GB / {hardware.MemoryTotal} GB", true)
        .WithFooter(footer => footer.Text = $"Requested by {Context.User.Username}")
        .WithCurrentTimestamp()
        .Build();

      await msg.ModifyAsync(m => {
        m.Content = string.Empty;
        m.Embed = embed;
      });
    }
  }
}
