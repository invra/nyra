namespace Nyra.Commands {
  [Category("Utility")]
  public class PingCommand : ModuleBase<SocketCommandContext> {
    private readonly DiscordSocketClient client;

    public PingCommand(DiscordSocketClient client) {
      this.client = client;
    }

    [Command("ping")]
    [Summary("Replies with pong and latency stats in an embed.")]
    public async Task PingAsync() {
      int gatewayLatency = client.Latency;

      var stopwatch = System.Diagnostics.Stopwatch.StartNew();
      var msg = await ReplyAsync("Probing the Gateway...");
      stopwatch.Stop();
      long roundTrip = stopwatch.ElapsedMilliseconds;

      var embed = new EmbedBuilder()
        .WithTitle("Gateway latency")
        .WithColor(Color.Purple)
        .AddField("Gateway Latency", $"{gatewayLatency} ms", true)
        .AddField("Message Round-Trip", $"{roundTrip} ms", true)
        .WithFooter(footer => footer.Text = $"Test co-ordinated by {Context.User.Username}")
        .WithCurrentTimestamp()
        .Build();

      await msg.ModifyAsync(m => {
        m.Content = string.Empty;
        m.Embed = embed;
      });
    }
  }
}
